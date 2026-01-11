import { getInstanceManager } from "./instance-manager";
import { Instance } from "../core/instance";

export interface ErrorInfo {
  message: string;
  details?: string;
}

/**
 * Base class for all Pagefind web components
 */
export class PagefindElement extends HTMLElement {
  instance: Instance | null = null;
  protected _initialized: boolean = false;

  constructor() {
    super();
  }

  connectedCallback(): void {
    if (this._initialized) return;
    this._initialized = true;

    const instanceName = this.getAttribute("instance") || "default";
    const manager = getInstanceManager();
    this.instance = manager.getInstance(instanceName);

    this.init();

    if (this.register && typeof this.register === "function") {
      this.register(this.instance);
    }
  }

  disconnectedCallback(): void {
    if (this.cleanup && typeof this.cleanup === "function") {
      this.cleanup();
    }
    this._initialized = false;
  }

  attributeChangedCallback(
    name: string,
    oldValue: string | null,
    newValue: string | null,
  ): void {
    if (!this._initialized || oldValue === newValue) return;

    const prop = this.kebabToCamel(name);

    if (newValue === "false") {
      (this as Record<string, unknown>)[prop] = false;
    } else if (newValue === "true") {
      (this as Record<string, unknown>)[prop] = true;
    } else if (newValue === null || newValue === undefined) {
      (this as Record<string, unknown>)[prop] = false;
    } else {
      (this as Record<string, unknown>)[prop] = newValue;
    }

    if (this.update && typeof this.update === "function") {
      this.update();
    }
  }

  protected kebabToCamel(str: string): string {
    return str.replace(/-([a-z])/g, (g) => g[1].toUpperCase());
  }

  protected ensureId(prefix = "pagefind"): string {
    if (!this.id && this.instance) {
      this.id = this.instance.generateId(prefix);
    }
    return this.id;
  }

  init(): void {}

  reconcileAria(): void {}

  register(_instance: Instance): void {}

  cleanup(): void {}

  update(): void {}

  showError(error: ErrorInfo): void {
    const errorEl = document.createElement("div");
    errorEl.className = "pf-error";
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

  protected escapeHtml(text: string): string {
    const div = document.createElement("div");
    div.textContent = text;
    return div.innerHTML;
  }
}
