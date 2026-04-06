import { Pagefind } from "./coupled_search.js";

const hasWorkerSupport =
  typeof window !== "undefined" &&
  typeof document !== "undefined" &&
  typeof Worker !== "undefined";

// Shared worker state: one worker serves all PagefindWrapper instances.
// Each wrapper gets a unique instanceId to namespace its messages.
let sharedWorker: Worker | null = null;
let sharedWorkerRefCount = 0;
let sharedMessageHandlers: Map<
  string,
  { resolve: Function; reject: Function }
> = new Map();

let nextInstanceId = 0;
const generateInstanceId = (): string => `pf_${nextInstanceId++}`;

function initSharedWorker(basePath: string): boolean {
  if (sharedWorker) return true;

  try {
    const workerUrl = `${basePath}pagefind-worker.js`;
    sharedWorker = new Worker(workerUrl);

    sharedWorker.addEventListener("error", (error) => {
      console.warn(
        "The Pagefind web worker encountered an error, falling back to main thread:",
        error,
      );
      sharedWorker = null;

      // Reject all pending messages across all instances
      const pending = Array.from(sharedMessageHandlers.values());
      sharedMessageHandlers.clear();
      for (const { reject } of pending) {
        reject(new Error("Worker failed, falling back to main thread"));
      }
    });

    sharedWorker.addEventListener("message", (event) => {
      const { id, result, error } = event.data;
      const pending = sharedMessageHandlers.get(id);
      if (pending) {
        sharedMessageHandlers.delete(id);
        if (error) {
          pending.reject(new Error(error));
        } else {
          pending.resolve(result);
        }
      }
    });

    return true;
  } catch (e) {
    return false;
  }
}

function releaseSharedWorker(): void {
  sharedWorkerRefCount--;
  if (sharedWorkerRefCount <= 0 && sharedWorker) {
    sharedWorker.terminate();
    sharedWorker = null;
    sharedWorkerRefCount = 0;

    const pending = Array.from(sharedMessageHandlers.values());
    sharedMessageHandlers.clear();
    for (const { reject } of pending) {
      reject(new Error("Pagefind worker terminated"));
    }
  }
}

let globalMessageId = 0;

function sendWorkerMessage(
  instanceId: string,
  method: string,
  args: any[],
): Promise<any> {
  if (!sharedWorker) {
    return Promise.reject(new Error("Worker not available"));
  }

  return new Promise((resolve, reject) => {
    const id = `msg_${globalMessageId++}`;
    sharedMessageHandlers.set(id, { resolve, reject });
    sharedWorker!.postMessage({ id, instanceId, method, args });
  });
}

export class PagefindWrapper {
  private instanceId: string;
  private fallback: Pagefind | null = null;
  private basePath: string;
  private initOptions: PagefindIndexOptions;
  private cleanup: FinalizationRegistry<string> | undefined;
  private initPromise: Promise<void> | null = null;
  private initialized = false;
  private useWorker = false;

  private initCleanup() {
    if (typeof FinalizationRegistry !== "undefined") {
      this.cleanup = new FinalizationRegistry((dataId: string) => {
        if (this.useWorker && sharedWorker) {
          try {
            sendWorkerMessage(this.instanceId, "releaseData", [dataId]).catch(
              () => {},
            );
          } catch (e) {
            // If the worker is dead, that's the ultimate GC :-)
          }
        }
      });
    }
  }

  constructor(options: PagefindIndexOptions = {}) {
    this.instanceId = generateInstanceId();
    this.basePath = options.basePath || "/pagefind/";
    this.initOptions = options;

    if (/[^\/]$/.test(this.basePath)) {
      this.basePath = `${this.basePath}/`;
    }

    if (
      hasWorkerSupport &&
      window?.location?.origin &&
      this.basePath.startsWith(window.location.origin)
    ) {
      this.basePath = this.basePath.replace(window.location.origin, "");
    }

    // Pass the resolved basePath downstream so the worker/fallback
    // doesn't need access to window.location.origin to strip it again.
    this.initOptions = { ...this.initOptions, basePath: this.basePath };

    this.initCleanup();
    this.initPromise = this.init();
  }

  private async init() {
    if (hasWorkerSupport && !(this.initOptions as any).noWorker) {
      const workerAvailable = initSharedWorker(this.basePath);

      if (workerAvailable) {
        try {
          sharedWorkerRefCount++;
          this.useWorker = true;

          await Promise.race([
            sendWorkerMessage(this.instanceId, "init", [this.initOptions]),
            new Promise((_, reject) =>
              setTimeout(
                () => reject(new Error("Worker initialization timeout")),
                5000,
              ),
            ),
          ]);
          this.initialized = true;
        } catch (error) {
          console.warn(
            "Failed to initialize Pagefind in the web worker, falling back to main thread:",
            error,
          );
          // Clean up the possibly-orphaned instance in the worker
          sendWorkerMessage(this.instanceId, "destroy", []).catch(() => {});
          this.useWorker = false;
          sharedWorkerRefCount--;
          this.initFallback();
          this.initialized = true;
        }
      } else {
        this.initFallback();
        this.initialized = true;
      }
    } else {
      this.initFallback();
      this.initialized = true;
    }
  }

  waitForInit(): Promise<void> {
    return this.initPromise ?? Promise.resolve();
  }

  private initFallback() {
    if (!this.fallback) {
      this.fallback = new Pagefind(this.initOptions);
    }
  }

  private async sendMessage(method: string, args: any[]): Promise<any> {
    // Wait for initialization to complete unless this is the init message itself
    if (!this.initialized && method !== "init") {
      if (this.initPromise) {
        await this.initPromise;
      }
    }

    if (this.fallback) {
      const fn = (this.fallback as any)[method];
      if (typeof fn === "function") {
        const result = await fn.apply(this.fallback, args);

        // Mark any verbose search results with the main thread marker,
        // which is mainly used in the test suite.
        if (
          (method === "search" || method === "debouncedSearch") &&
          result &&
          args[1] &&
          args[1].verbose
        ) {
          result.search_environment = "mainthread";
        }

        return result;
      }
      throw new Error(`Method ${method} not found on fallback`);
    }

    if (!this.useWorker || !sharedWorker) {
      throw new Error("Worker not initialized");
    }

    return sendWorkerMessage(this.instanceId, method, args);
  }

  async options(options: PagefindIndexOptions) {
    return this.sendMessage("options", [options]);
  }

  async enterPlaygroundMode() {
    return this.sendMessage("enterPlaygroundMode", []);
  }

  async mergeIndex(indexPath: string, options: PagefindIndexOptions = {}) {
    return this.sendMessage("mergeIndex", [indexPath, options]);
  }

  async search(
    term: string,
    options: PagefindSearchOptions = {},
  ): Promise<PagefindIndexesSearchResults> {
    const results = await this.sendMessage("search", [term, options]);

    // Convert result data IDs into worker-calling functions
    if (results && results.results) {
      for (const result of results.results) {
        if (typeof result.data === "string") {
          const dataId = result.data;

          // Register this result object for cleanup when GC'd
          if (this.cleanup) {
            this.cleanup.register(result, dataId);
          }

          result.data = async () => {
            return this.sendMessage("getData", [dataId]);
          };
        }
      }
    }

    return results;
  }

  async debouncedSearch(
    term: string,
    options?: PagefindSearchOptions,
    debounceTimeoutMs?: number,
  ): Promise<PagefindIndexesSearchResults | null> {
    const results = await this.sendMessage("debouncedSearch", [
      term,
      options,
      debounceTimeoutMs,
    ]);

    // Convert result data IDs into worker-calling functions
    if (results && results.results) {
      for (const result of results.results) {
        if (typeof result.data === "string") {
          const dataId = result.data;

          // Register this result object for cleanup when GC'd
          if (this.cleanup) {
            this.cleanup.register(result, dataId);
          }

          result.data = async () => {
            return this.sendMessage("getData", [dataId]);
          };
        }
      }
    }

    return results;
  }

  async preload(term: string, options: PagefindSearchOptions = {}) {
    return this.sendMessage("preload", [term, options]);
  }

  async filters(): Promise<PagefindFilterCounts> {
    return this.sendMessage("filters", []);
  }

  async destroy() {
    if (this.useWorker) {
      try {
        await sendWorkerMessage(this.instanceId, "destroy", []);
      } catch (e) {
        // May already be dead
      }
      this.useWorker = false;
      releaseSharedWorker();
    }
    if (this.fallback) {
      this.fallback = null;
    }
  }
}
