import { PagefindElement } from "./base-element.js";

export class PagefindSummary extends PagefindElement {
    static get observedAttributes() {
        return ['default-message'];
    }

    constructor() {
        super();
        this.containerEl = null;
        this.term = "";
        this.defaultMessage = "";
    }

    init() {
        if (this.hasAttribute('default-message')) {
            this.defaultMessage = this.getAttribute('default-message');
        }

        this.render();
    }

    render() {
        this.innerHTML = "";

        this.containerEl = document.createElement("div");
        this.containerEl.className = "pagefind-modular-summary";
        this.containerEl.textContent = this.defaultMessage;

        this.containerEl.setAttribute("aria-live", "polite");
        this.containerEl.setAttribute("aria-atomic", "true");

        this.appendChild(this.containerEl);
    }

    reconcileAria() {}

    register(instance) {
        instance.registerSummary(this);
        instance.on("search", (term, _filters) => {
            this.term = term;
        });

        instance.on("results", (results) => {
            if (!this.containerEl || !results) return;

            if (!this.term) {
                this.containerEl.textContent = this.defaultMessage;
                return;
            }

            let count = results?.results?.length ?? 0;
            this.containerEl.textContent = `${count} result${count === 1 ? '' : 's'} for ${this.term}`;
        });

        instance.on("loading", () => {
            if (!this.containerEl) return;
            this.containerEl.textContent = `Searching for ${this.term}...`;
        });

        instance.on("error", (error) => {
            if (!this.containerEl) return;
            this.containerEl.textContent = `Error: ${error.message || 'Search failed'}`;
        });
    }

    update() {
        if (this.hasAttribute('default-message')) {
            this.defaultMessage = this.getAttribute('default-message');
            if (!this.term && this.containerEl) {
                this.containerEl.textContent = this.defaultMessage;
            }
        }
    }
}

// Register the custom element
if (!customElements.get('pagefind-summary')) {
    customElements.define('pagefind-summary', PagefindSummary);
}
