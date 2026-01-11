import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import type { PagefindSearchResult, PagefindError } from "../types";

export class PagefindSummary extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["default-message"];
  }

  containerEl: HTMLElement | null = null;
  term: string = "";
  defaultMessage: string = "";

  constructor() {
    super();
  }

  init(): void {
    if (this.hasAttribute("default-message")) {
      this.defaultMessage = this.getAttribute("default-message") || "";
    }

    this.render();
  }

  render(): void {
    this.innerHTML = "";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.containerEl = document.createElement("div");
    this.containerEl.className = "pf-summary";
    this.containerEl.textContent = this.defaultMessage;

    this.appendChild(this.containerEl);
  }

  reconcileAria(): void {}

  register(instance: Instance): void {
    instance.registerSummary(this);
    instance.on(
      "search",
      (term: unknown) => {
        this.term = term as string;
      },
      this,
    );

    instance.on(
      "results",
      (results: unknown) => {
        if (!this.containerEl || !results) return;
        const searchResult = results as PagefindSearchResult;
        const count = searchResult?.results?.length ?? 0;

        if (!this.term) {
          if (instance.faceted) {
            const key =
              count === 0
                ? "total_zero_results"
                : count === 1
                  ? "total_one_result"
                  : "total_many_results";
            const text = instance.translate(key, { COUNT: count });
            this.containerEl.textContent =
              text || `${count} result${count === 1 ? "" : "s"}`;
          } else {
            this.containerEl.textContent = this.defaultMessage;
          }
          return;
        }

        const key =
          count === 0
            ? "zero_results"
            : count === 1
              ? "one_result"
              : "many_results";
        const text = instance.translate(key, {
          SEARCH_TERM: this.term,
          COUNT: count,
        });
        this.containerEl.textContent =
          text || `${count} result${count === 1 ? "" : "s"} for ${this.term}`;
      },
      this,
    );

    instance.on(
      "loading",
      () => {
        if (!this.containerEl) return;
        const text = instance.translate("searching", {
          SEARCH_TERM: this.term,
        });
        this.containerEl.textContent = text || `Searching for ${this.term}...`;
      },
      this,
    );

    instance.on(
      "error",
      (error: unknown) => {
        if (!this.containerEl) return;
        const err = error as PagefindError;
        const errorText = instance.translate("error_search") || "Search failed";
        this.containerEl.textContent = err.message || errorText;
      },
      this,
    );

    instance.on(
      "translations",
      () => {
        this.render();
      },
      this,
    );
  }

  update(): void {
    if (this.hasAttribute("default-message")) {
      this.defaultMessage = this.getAttribute("default-message") || "";
      if (!this.term && this.containerEl) {
        this.containerEl.textContent = this.defaultMessage;
      }
    }
  }
}

if (!customElements.get("pagefind-summary")) {
  customElements.define("pagefind-summary", PagefindSummary);
}
