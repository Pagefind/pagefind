import { Instance } from "../modular-core.js";

const instanceConfigs = new WeakMap();

class InstanceManager {
    constructor() {
        this.instances = new Map();
        this.defaultOptions = {
            bundlePath: this.detectBundlePath(),
        };
        this._pendingOptions = new Map();
    }

    detectBundlePath() {
        try {
            // Important: Check that the element is indeed a <script> node, to avoid a DOM clobbering vulnerability
            if (document?.currentScript && document.currentScript.tagName.toUpperCase() === 'SCRIPT') {
                const scriptPath = new URL(document.currentScript.src).pathname.match(
                    /^(.*\/)(?:pagefind-)?.*\.js.*$/
                );
                if (scriptPath) {
                    return scriptPath[1];
                }
            }
        } catch (e) {
            // Fall through to default
        }
        return "/pagefind/";
    }

    getInstance(name = "default", options = {}) {
        if (this.instances.has(name)) {
            return this.instances.get(name);
        }

        const pendingOptions = this._pendingOptions.get(name) || {};

        const instanceOptions = {
            ...this.defaultOptions,
            ...options,
            ...pendingOptions,
        };

        const instance = new Instance(instanceOptions);
        this.instances.set(name, instance);

        instanceConfigs.set(instance, pendingOptions);

        this._pendingOptions.delete(name);

        return instance;
    }

    setInstanceOptions(name, options) {
        if (this.instances.has(name)) {
            console.warn(`[Pagefind Web Components]: Instance "${name}" already exists, options cannot be changed`);
            return;
        }

        this._pendingOptions.set(name, options);
    }

    getInstanceConfig(name) {
        if (this.instances.has(name)) {
            return instanceConfigs.get(this.instances.get(name)) || {};
        }
        return this._pendingOptions.get(name) || {};
    }

    hasInstance(name) {
        return this.instances.has(name);
    }

    removeInstance(name) {
        this.instances.delete(name);
    }

    getInstanceNames() {
        return Array.from(this.instances.keys());
    }
}

let instanceManager = null;

export function getInstanceManager() {
    if (!instanceManager) {
        instanceManager = new InstanceManager();
    }
    return instanceManager;
}

/**
 * Configure options for a named instance
 * Must be called before any components using that instance are created
 */
export function configureInstance(name, options) {
    const manager = getInstanceManager();

    if (manager.hasInstance(name)) {
        console.warn(`[Pagefind Web Components]: Instance "${name}" already exists, configuration ignored`);
        return manager.getInstance(name);
    }

    return manager.getInstance(name, options);
}
