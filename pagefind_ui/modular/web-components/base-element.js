import { getInstanceManager } from "./instance-manager.js";

const idChars = "abcdef";

const randomSeg = (length = 3) => {
  let word = "";
  for (let i = 0; i < length; i++) {
    word += idChars[Math.floor(Math.random() * idChars.length)];
  }
  return word;
};

export const generateId = (prefix, length = 2) => {
  const segments = Array.from({ length }, () => randomSeg()).join("-");
  const id = `${prefix}-${segments}`;

  if (document.getElementById(id)) {
    return generateId(length + 1);
  }

  return id;
};

/**
 * Base class for all Pagefind web components
 * Provides core functionality for auto-registration, attribute handling, and ARIA management
 */
export class PagefindElement extends HTMLElement {
  constructor() {
    super();
    this.instance = null;
    this._initialized = false;
  }

  connectedCallback() {
    if (this._initialized) return;
    this._initialized = true;

    // Always render the component UI
    this.init();

    // Always get/create the instance (instance itself is lazy)
    const instanceName = this.getAttribute("instance") || "default";
    const manager = getInstanceManager();
    this.instance = manager.getInstance(instanceName);

    // Register with instance
    if (this.register && typeof this.register === "function") {
      this.register(this.instance);
    }
  }

  disconnectedCallback() {
    if (this.cleanup && typeof this.cleanup === "function") {
      this.cleanup();
    }
  }

  attributeChangedCallback(name, oldValue, newValue) {
    if (!this._initialized || oldValue === newValue) return;

    const prop = this.kebabToCamel(name);

    if (newValue === "false") {
      this[prop] = false;
    } else if (newValue === "true") {
      this[prop] = true;
    } else if (newValue === null || newValue === undefined) {
      this[prop] = false;
    } else {
      this[prop] = newValue;
    }

    if (this.update && typeof this.update === "function") {
      this.update();
    }
  }

  kebabToCamel(str) {
    return str.replace(/-([a-z])/g, (g) => g[1].toUpperCase());
  }

  ensureId(prefix = "pagefind") {
    if (!this.id) {
      this.id = generateId(prefix);
    }
    return this.id;
  }

  init() {}

  reconcileAria() {}

  register(instance) {}

  showError(error) {
    const errorEl = document.createElement("div");
    errorEl.className = "pagefind-error";
    errorEl.innerHTML = `
            <strong>Pagefind Error:</strong> ${this.escapeHtml(error.message || "Unknown error")}
            ${error.details ? `<br><small>${this.escapeHtml(error.details)}</small>` : ""}
        `;
    this.appendChild(errorEl);

    this.dispatchEvent(
      new CustomEvent("pagefind-error", {
        detail: error,
        bubbles: true,
        composed: true,
      }),
    );
  }

  escapeHtml(text) {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }
}
