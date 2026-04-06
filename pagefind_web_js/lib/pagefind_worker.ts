import { Pagefind } from "./coupled_search.js";

interface WorkerMessage {
  id: string;
  instanceId?: string;
  method: string;
  args: any[];
}

interface WorkerResponse {
  id: string;
  result?: any;
  error?: string;
}

let dataCallbacks: Map<string, any> = new Map();
let instanceDataIds: Map<string, Set<string>> = new Map();
let instances: Map<string, Pagefind> = new Map();

// Defensive fallback if a message arrives without an instanceId
const DEFAULT_INSTANCE = "default";

const getInstance = (instanceId: string): Pagefind => {
  const instance = instances.get(instanceId);
  if (!instance) {
    throw new Error(`Pagefind instance "${instanceId}" not initialized`);
  }
  return instance;
};

const registerDataCallback = (
  instanceId: string,
  dataId: string,
  dataFn: any,
): void => {
  dataCallbacks.set(dataId, { getData: dataFn });
  if (!instanceDataIds.has(instanceId)) {
    instanceDataIds.set(instanceId, new Set());
  }
  instanceDataIds.get(instanceId)!.add(dataId);
};

const handleMessage = async (
  message: WorkerMessage,
): Promise<WorkerResponse> => {
  const { id, method, args } = message;
  const instanceId = message.instanceId ?? DEFAULT_INSTANCE;

  try {
    switch (method) {
      case "init": {
        const [options] = args;
        instances.set(instanceId, new Pagefind(options));
        return { id, result: true };
      }

      case "options": {
        const pagefindInstance = getInstance(instanceId);
        const [options] = args;
        await pagefindInstance.options(options);
        return { id, result: true };
      }

      case "enterPlaygroundMode": {
        const pagefindInstance = getInstance(instanceId);
        await pagefindInstance.enterPlaygroundMode();
        return { id, result: true };
      }

      case "mergeIndex": {
        const pagefindInstance = getInstance(instanceId);
        const [indexPath, options] = args;
        await pagefindInstance.mergeIndex(indexPath, options);
        return { id, result: true };
      }

      case "search": {
        const pagefindInstance = getInstance(instanceId);
        const [term, options] = args;
        const results = await pagefindInstance.search(term, options);

        // Convert the data() functions to IDs that can be called later
        if (results && results.results) {
          for (let i = 0; i < results.results.length; i++) {
            const result = results.results[i];
            const dataFn = result.data;
            const dataId = `data_${id}_${i}`;

            registerDataCallback(instanceId, dataId, dataFn);

            result.data = dataId as any;
          }
        }

        if (results && options && options.verbose) {
          results.search_environment = "webworker";
        }

        return { id, result: results };
      }

      case "debouncedSearch": {
        const pagefindInstance = getInstance(instanceId);
        const [term, options, debounceTimeoutMs] = args;
        const results = await pagefindInstance.debouncedSearch(
          term,
          options,
          debounceTimeoutMs,
        );

        // Convert the data() functions to IDs that can be called later
        if (results && results.results) {
          for (let i = 0; i < results.results.length; i++) {
            const result = results.results[i];
            const dataFn = result.data;
            const dataId = `data_${id}_${i}`;

            registerDataCallback(instanceId, dataId, dataFn);

            result.data = dataId as any;
          }
        }

        if (results && options && options.verbose) {
          results.search_environment = "webworker";
        }

        return { id, result: results };
      }

      case "preload": {
        const pagefindInstance = getInstance(instanceId);
        const [term, options] = args;
        await pagefindInstance.preload(term, options);
        return { id, result: true };
      }

      case "filters": {
        const pagefindInstance = getInstance(instanceId);
        const result = await pagefindInstance.filters();
        return { id, result };
      }

      case "getData": {
        const [dataId] = args;
        const callback = dataCallbacks.get(dataId);
        if (!callback?.getData) {
          // Instance may have been destroyed while data() was in-flight
          return { id, result: null };
        }
        const data = await callback.getData();
        return { id, result: data };
      }

      case "releaseData": {
        const [dataId] = args;
        dataCallbacks.delete(dataId);
        instanceDataIds.get(instanceId)?.delete(dataId);
        return { id, result: true };
      }

      case "destroy": {
        instances.delete(instanceId);
        const dataIds = instanceDataIds.get(instanceId);
        if (dataIds) {
          for (const dataId of dataIds) {
            dataCallbacks.delete(dataId);
          }
          instanceDataIds.delete(instanceId);
        }
        return { id, result: true };
      }

      default:
        throw new Error(`Unknown method: ${method}`);
    }
  } catch (error) {
    return {
      id,
      error: error instanceof Error ? error.message : String(error),
    };
  }
};

self.addEventListener("message", async (event: MessageEvent) => {
  const message = event.data as WorkerMessage;
  const response = await handleMessage(message);
  self.postMessage(response);
});

export {};
