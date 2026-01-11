import { Instance } from "../core/instance";
import type { InstanceOptions } from "../types";

class InstanceManager {
  private instances: Map<string, Instance> = new Map();
  private defaultOptions: InstanceOptions;

  constructor() {
    this.defaultOptions = {
      bundlePath: this.detectBundlePath(),
    };
  }

  private detectBundlePath(): string {
    try {
      // Important: Check that the element is indeed a <script> node, to avoid a DOM clobbering vulnerability
      if (
        document?.currentScript &&
        document.currentScript.tagName.toUpperCase() === "SCRIPT"
      ) {
        const scriptPath = new URL(
          (document.currentScript as HTMLScriptElement).src,
        ).pathname.match(/^(.*\/)(?:pagefind[-_])?.*\.js.*$/);
        if (scriptPath) {
          return scriptPath[1];
        }
      }
    } catch (e) {}
    return "/pagefind/";
  }

  getInstance(
    name: string = "default",
    options: InstanceOptions = {},
  ): Instance {
    const existing = this.instances.get(name);
    if (existing) {
      return existing;
    }

    const instanceOptions: InstanceOptions = {
      ...this.defaultOptions,
      ...options,
    };

    const instance = new Instance(name, instanceOptions);
    this.instances.set(name, instance);

    return instance;
  }

  hasInstance(name: string): boolean {
    return this.instances.has(name);
  }

  removeInstance(name: string): void {
    this.instances.delete(name);
  }

  getInstanceNames(): string[] {
    return Array.from(this.instances.keys());
  }
}

let instanceManager: InstanceManager | null = null;

export function getInstanceManager(): InstanceManager {
  if (!instanceManager) {
    instanceManager = new InstanceManager();
  }
  return instanceManager;
}

/**
 * Configure options for a named instance
 */
export function configureInstance(
  name: string,
  options: InstanceOptions,
): Instance {
  const manager = getInstanceManager();

  if (manager.hasInstance(name)) {
    console.warn(
      `[Pagefind Component UI]: Instance "${name}" already exists, configuration ignored`,
    );
    return manager.getInstance(name);
  }

  return manager.getInstance(name, options);
}
