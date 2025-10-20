import { PagefindElement, generateId } from "./base-element.js";
import El from "../helpers/element-builder.js";

const asyncSleep = (ms = 100) => new Promise(r => setTimeout(r, ms));

export class PagefindInput extends PagefindElement {
    static get observedAttributes() {
        return ['placeholder', 'debounce', 'autofocus'];
    }

    constructor() {
        super();
        this.inputEl = null;
        this.clearEl = null;
        this.searchID = 0;

        this.placeholder = "";
        this.debounce = 300;
        this.autofocus = false;
    }

    readAttributes() {
        if (this.hasAttribute('placeholder')) {
            this.placeholder = this.getAttribute('placeholder');
        }
        if (this.hasAttribute('debounce')) {
            this.debounce = parseInt(this.getAttribute('debounce'), 10) || 300;
        }
        if (this.hasAttribute('autofocus')) {
            this.autofocus = this.hasAttribute('autofocus');
        }
    }

    init() {
        this.readAttributes();
        this.render();
    }

    setupLazyLoading(callback) {
        this.readAttributes();
        this.render();

        const handler = () => {
            callback();
            if (this.inputEl) {
                this.inputEl.removeEventListener('focus', handler);
                this.inputEl.removeEventListener('input', handler);
            }
        };

        if (this.inputEl) {
            this.inputEl.addEventListener('focus', handler, { once: true });
            this.inputEl.addEventListener('input', handler, { once: true });
        }
    }

    render() {
        this.innerHTML = "";

        const inputId = generateId("pfmod-input");

        const wrapper = new El("form")
            .class("pagefind-modular-input-wrapper")
            .attrs({
                role: "search",
                "aria-label": "Search this site",
                action: "javascript:void(0);"
            });

        new El("label").attrs({
            "for": inputId,
            "data-pfmod-sr-hidden": "true"
        }).text("Search this site").addTo(wrapper);

        this.inputEl = new El("input")
            .id(inputId)
            .class("pagefind-modular-input")
            .attrs({
                autocapitalize: "none",
                enterkeyhint: "search",
                placeholder: this.placeholder
            })
            .addTo(wrapper);

        if (this.autofocus) {
            this.inputEl.setAttribute('autofocus', 'autofocus');
        }

        this.clearEl = new El("button")
            .class("pagefind-modular-input-clear")
            .attrs({"data-pfmod-suppressed": "true"})
            .text("Clear")
            .addTo(wrapper);

        wrapper.addTo(this);

        this.setupEventHandlers();
    }

    setupEventHandlers() {
        // Input event - trigger search with debounce
        this.inputEl.addEventListener("input", async (e) => {
            if (this.instance && typeof e?.target?.value === "string") {
                this.updateState(e.target.value);

                const thisSearchID = ++this.searchID;
                await asyncSleep(this.debounce);

                if (thisSearchID !== this.searchID) {
                    return null;
                }

                this.instance?.triggerSearch(e.target.value);
            }
        });

        // Escape key - clear input
        this.inputEl.addEventListener("keydown", (e) => {
            if (e.key === "Escape") {
                ++this.searchID;
                this.inputEl.value = "";
                this.instance?.triggerSearch("");
                this.updateState("");
            }
            if (e.key === "Enter") {
                e.preventDefault();
            }
        });

        // Focus event - trigger load
        this.inputEl.addEventListener("focus", () => {
            this.instance?.triggerLoad();
        });

        // Clear button click
        this.clearEl.addEventListener("click", () => {
            this.inputEl.value = "";
            this.instance.triggerSearch("");
            this.updateState("");
            this.inputEl.focus();
        });
    }

    updateState(term) {
        if (this.clearEl) {
            if (term && term?.length) {
                this.clearEl.removeAttribute("data-pfmod-suppressed");
            } else {
                this.clearEl.setAttribute("data-pfmod-suppressed", "true");
            }
        }
    }

    setupAria() {
        // Find related results component
        const results = this.findRelatedComponent('pagefind-results');

        if (results) {
            // Ensure both have IDs
            this.ensureId("pagefind-input");
            if (!results.id) {
                results.id = results.ensureId ? results.ensureId("pagefind-results") : `pagefind-results-${Date.now()}`;
            }

            // Set ARIA attributes
            this.inputEl.setAttribute('aria-controls', results.id);
            this.inputEl.setAttribute('role', 'combobox');
            this.inputEl.setAttribute('aria-expanded', 'false');
            this.inputEl.setAttribute('aria-autocomplete', 'list');

            // Update aria-expanded when there are results
            if (this.instance) {
                this.instance.on("results", (searchResults) => {
                    if (searchResults?.results?.length > 0) {
                        this.inputEl.setAttribute('aria-expanded', 'true');
                    } else {
                        this.inputEl.setAttribute('aria-expanded', 'false');
                    }
                });
            }
        }
    }

    register(instance) {
        // Listen for search events to sync input value
        instance.on("search", (term, _filters) => {
            if (this.inputEl && document.activeElement !== this.inputEl) {
                this.inputEl.value = term;
                this.updateState(term);
            }
        });

        // Listen for error events
        instance.on("error", (error) => {
            this.showError({
                message: error.message || "Search initialization failed",
                details: error.bundlePath ? `Bundle path: ${error.bundlePath}` : null
            });
        });
    }

    update() {
        // Re-render when attributes change
        this.render();
    }

    focus() {
        if (this.inputEl) {
            this.inputEl.focus();
        }
    }
}

// Register the custom element
if (!customElements.get('pagefind-input')) {
    customElements.define('pagefind-input', PagefindInput);
}
