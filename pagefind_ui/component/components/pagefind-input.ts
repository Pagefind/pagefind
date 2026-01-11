import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import type { PagefindError } from "../types";

const asyncSleep = (ms = 100): Promise<void> =>
  new Promise((r) => setTimeout(r, ms));

export class PagefindInput extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["placeholder", "debounce", "autofocus"];
  }

  inputEl: HTMLInputElement | null = null;
  clearEl: HTMLButtonElement | null = null;
  searchID: number = 0;

  placeholder: string = "";
  debounce: number = 300;
  autofocus: boolean = false;

  constructor() {
    super();
  }

  readAttributes(): void {
    if (this.hasAttribute("placeholder")) {
      this.placeholder = this.getAttribute("placeholder") || "";
    }
    if (this.hasAttribute("debounce")) {
      this.debounce =
        parseInt(this.getAttribute("debounce") || "300", 10) || 300;
    }
    if (this.hasAttribute("autofocus")) {
      this.autofocus = this.hasAttribute("autofocus");
    }
  }

  init(): void {
    this.readAttributes();
    this.render();
  }

  render(): void {
    this.innerHTML = "";

    const inputId = this.instance!.generateId("pfmod-input");

    const searchLabel =
      this.instance?.translate("search_label") || "Search this site";
    const clearText = this.instance?.translate("clear_search") || "Clear";
    const placeholderText =
      this.placeholder || this.instance?.translate("placeholder") || "Search";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    const wrapper = document.createElement("search");
    wrapper.className = "pf-input-wrapper";
    wrapper.setAttribute("role", "search");
    wrapper.setAttribute("aria-label", searchLabel);

    const label = document.createElement("label");
    label.setAttribute("for", inputId);
    label.setAttribute("data-pf-sr-hidden", "true");
    label.textContent = searchLabel;
    wrapper.appendChild(label);

    this.inputEl = document.createElement("input");
    this.inputEl.id = inputId;
    this.inputEl.className = "pf-input";
    this.inputEl.setAttribute("type", "search");
    this.inputEl.setAttribute("autocomplete", "off");
    this.inputEl.setAttribute("autocapitalize", "none");
    this.inputEl.setAttribute("enterkeyhint", "search");
    this.inputEl.setAttribute("placeholder", placeholderText);
    if (this.autofocus) {
      this.inputEl.setAttribute("autofocus", "autofocus");
    }

    const hintId = this.instance!.generateId("pf-input-hint");
    const hintText =
      this.instance?.translate("input_hint") ||
      "Results will appear as you type";
    const hint = document.createElement("span");
    hint.id = hintId;
    hint.setAttribute("data-pf-sr-hidden", "true");
    hint.textContent = hintText;
    this.inputEl.setAttribute("aria-describedby", hintId);

    wrapper.appendChild(this.inputEl);
    wrapper.appendChild(hint);

    this.clearEl = document.createElement("button");
    this.clearEl.className = "pf-input-clear";
    this.clearEl.setAttribute("type", "button");
    this.clearEl.setAttribute("data-pf-suppressed", "true");
    this.clearEl.textContent = clearText;
    wrapper.appendChild(this.clearEl);

    this.appendChild(wrapper);

    this.setupEventHandlers();
  }

  setupEventHandlers(): void {
    if (!this.inputEl || !this.clearEl) return;

    this.inputEl.addEventListener("input", async (e) => {
      const target = e.target as HTMLInputElement;
      if (this.instance && typeof target?.value === "string") {
        this.updateState(target.value);

        const thisSearchID = ++this.searchID;
        await asyncSleep(this.debounce);

        if (thisSearchID !== this.searchID) {
          return;
        }

        this.instance?.triggerSearch(target.value);
      }
    });

    this.inputEl.addEventListener("keydown", (e) => {
      if (e.key === "Escape") {
        ++this.searchID;
        if (this.inputEl) this.inputEl.value = "";
        this.instance?.triggerSearch("");
        this.updateState("");
      }
      if (e.key === "ArrowDown") {
        e.preventDefault();
        if (this.inputEl) {
          this.instance?.focusNextResults(this.inputEl);
        }
      }
    });

    this.inputEl.addEventListener("focus", () => {
      this.instance?.triggerLoad();
      const navigateText =
        this.instance?.translate("keyboard_navigate") || "navigate";
      const clearText = this.instance?.translate("keyboard_clear") || "clear";
      this.instance?.registerShortcut(
        { label: "↓", description: navigateText },
        this,
      );
      this.instance?.registerShortcut(
        { label: "esc", description: clearText },
        this,
      );
    });

    this.inputEl.addEventListener("blur", () => {
      this.instance?.deregisterAllShortcuts(this);
    });

    this.clearEl.addEventListener("click", () => {
      if (this.inputEl) {
        this.inputEl.value = "";
        this.instance?.triggerSearch("");
        this.updateState("");
        this.inputEl.focus();
      }
    });
  }

  updateState(term: string): void {
    if (this.clearEl) {
      if (term && term?.length) {
        this.clearEl.removeAttribute("data-pf-suppressed");
      } else {
        this.clearEl.setAttribute("data-pf-suppressed", "true");
      }
    }
  }

  register(instance: Instance): void {
    instance.registerInput(this, {
      keyboardNavigation: true,
    });

    instance.on(
      "search",
      (term: unknown) => {
        if (this.inputEl && document.activeElement !== this.inputEl) {
          this.inputEl.value = term as string;
          this.updateState(term as string);
        }
      },
      this,
    );

    instance.on(
      "error",
      (error: unknown) => {
        const err = error as PagefindError;
        this.showError({
          message: err.message || "Search initialization failed",
          details: err.bundlePath
            ? `Bundle path: ${err.bundlePath}`
            : undefined,
        });
      },
      this,
    );

    instance.on(
      "translations",
      () => {
        const currentValue = this.inputEl?.value || "";
        this.render();
        if (this.inputEl && currentValue) {
          this.inputEl.value = currentValue;
          this.updateState(currentValue);
        }
      },
      this,
    );
  }

  update(): void {
    this.render();
  }

  focus(): void {
    if (this.inputEl) {
      this.inputEl.focus();
    }
  }
}

if (!customElements.get("pagefind-input")) {
  customElements.define("pagefind-input", PagefindInput);
}
