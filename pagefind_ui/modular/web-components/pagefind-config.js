import { getInstanceManager } from "./instance-manager.js";

export class PagefindConfig extends HTMLElement {
    constructor() {
        super();
        this._configured = false;
    }

    connectedCallback() {
        if (this._configured) return;
        this._configured = true;

        const instanceName = this.getAttribute("instance") || "default";
        const manager = getInstanceManager();

        const config = {};

        const bundlePath = this.getAttribute("bundle-path");
        if (bundlePath) {
            config.bundlePath = bundlePath;
        }

        if (this.hasAttribute("preload")) {
            config.preload = true;
        }

        manager.setInstanceOptions(instanceName, config);

        this.setAttribute("hidden", "");
    }
}

// Register the custom element
if (!customElements.get('pagefind-config')) {
    customElements.define('pagefind-config', PagefindConfig);
}
