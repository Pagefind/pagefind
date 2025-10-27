import { PagefindElement } from "./base-element.js";

export class PagefindConfig extends PagefindElement {
    init() {
        // Hide the config element
        this.setAttribute("hidden", "");
    }

    register(instance) {
        instance.registerUtility(this);

        // Configure the instance before it initializes
        const bundlePath = this.getAttribute("bundle-path");
        if (bundlePath) {
            instance.options.bundlePath = bundlePath;
        }

        // Trigger load if preload is set
        if (this.hasAttribute("preload")) {
            instance.triggerLoad();
        }
    }
}

// Register the custom element
if (!customElements.get('pagefind-config')) {
    customElements.define('pagefind-config', PagefindConfig);
}
