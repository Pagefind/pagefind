import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import { compile, type Template } from "adequate-little-templates";
import type {
  PagefindSearchResult,
  PagefindRawResult,
  PagefindResultData,
  PagefindError,
} from "../types";

const asyncSleep = (ms = 100): Promise<void> =>
  new Promise((r) => setTimeout(r, ms));

type TemplateResult = Element | Element[] | string;

interface SearchboxResultTemplateData {
  meta: Record<string, string | undefined>;
  excerpt: string;
  url: string;
  sub_results: Array<{
    title: string;
    url: string;
    excerpt: string;
    aria: {
      result_id: string;
      title_id: string;
      excerpt_id: string;
    };
  }>;
  options: {
    show_sub_results: boolean;
  };
  aria: {
    result_id: string;
    title_id: string;
    excerpt_id: string;
  };
}

const templateNodes = (templateResult: TemplateResult): Node[] => {
  if (templateResult instanceof Element) {
    return [templateResult];
  }
  if (
    Array.isArray(templateResult) &&
    templateResult.every((r) => r instanceof Element)
  ) {
    return templateResult;
  }
  if (typeof templateResult === "string" || templateResult instanceof String) {
    const wrap = document.createElement("div");
    wrap.innerHTML = templateResult as string;
    return [...wrap.childNodes];
  }
  console.error(
    `[Pagefind Searchbox]: Expected template to return HTML element or string, got ${typeof templateResult}`,
  );
  return [];
};

const DEFAULT_RESULT_TEMPLATE = `<a class="pf-searchbox-result" id="{{ aria.result_id }}" href="{{ meta.url | default(url) | safeUrl }}" role="option" aria-selected="false" aria-labelledby="{{ aria.title_id }}"{{#if excerpt}} aria-describedby="{{ aria.excerpt_id }}"{{/if}}>
  <p class="pf-searchbox-result-title" id="{{ aria.title_id }}">{{ meta.title | default("Untitled") }}</p>
  {{#if excerpt}}
  <p class="pf-searchbox-result-excerpt" id="{{ aria.excerpt_id }}">{{+ excerpt +}}</p>
  {{/if}}
</a>
{{#if and(options.show_sub_results, sub_results)}}
{{#each sub_results as sub}}
<a class="pf-searchbox-result pf-searchbox-subresult" id="{{ sub.aria.result_id }}" href="{{ sub.url | safeUrl }}" role="option" aria-selected="false" aria-labelledby="{{ sub.aria.title_id }}"{{#if sub.excerpt}} aria-describedby="{{ sub.aria.excerpt_id }}"{{/if}}>
  <p class="pf-searchbox-result-title" id="{{ sub.aria.title_id }}">{{ sub.title | default("Section") }}</p>
  {{#if sub.excerpt}}
  <p class="pf-searchbox-result-excerpt" id="{{ sub.aria.excerpt_id }}">{{+ sub.excerpt +}}</p>
  {{/if}}
</a>
{{/each}}
{{/if}}`;

const defaultResultTemplate: Template<SearchboxResultTemplateData> = compile(
  DEFAULT_RESULT_TEMPLATE,
);

const DEFAULT_PLACEHOLDER_TEMPLATE = `<div class="pf-searchbox-result pf-searchbox-placeholder" role="option" aria-selected="false">
  <p class="pf-searchbox-result-title pf-skeleton pf-skeleton-title"></p>
  <p class="pf-searchbox-result-excerpt pf-skeleton pf-skeleton-excerpt"></p>
</div>`;

const defaultPlaceholderTemplate: Template<Record<string, never>> = compile(
  DEFAULT_PLACEHOLDER_TEMPLATE,
);

interface SearchboxResultOptions {
  rawResult: PagefindRawResult;
  placeholderEl: Element;
  renderFn: (result: PagefindResultData) => TemplateResult;
  intersectionRoot: Element | null;
  onLoad?: () => void;
}

class SearchboxResult {
  rawResult: PagefindRawResult;
  placeholderEl: Element;
  renderFn: (result: PagefindResultData) => TemplateResult;
  intersectionRoot: Element | null;
  onLoad?: () => void;
  data: PagefindResultData | null = null;
  private observer: IntersectionObserver | null = null;

  constructor(opts: SearchboxResultOptions) {
    this.rawResult = opts.rawResult;
    this.placeholderEl = opts.placeholderEl;
    this.renderFn = opts.renderFn;
    this.intersectionRoot = opts.intersectionRoot;
    this.onLoad = opts.onLoad;
    this.setupObserver();
  }

  setupObserver(): void {
    if (this.data !== null || this.observer !== null) return;

    const options = {
      root: this.intersectionRoot,
      rootMargin: "50px", // Start loading slightly before visible
      threshold: 0.01,
    };

    this.observer = new IntersectionObserver((entries, obs) => {
      if (this.data !== null) return;
      if (entries?.[0]?.isIntersecting) {
        this.load();
        obs.disconnect();
        this.observer = null;
      }
    }, options);

    this.observer.observe(this.placeholderEl);
  }

  async load(): Promise<void> {
    this.data = await this.rawResult.data();
    const templateResult = this.renderFn(this.data);
    const nodes = templateNodes(templateResult);

    if (nodes.length > 0 && this.placeholderEl.parentNode) {
      const firstNode = nodes[0];
      this.placeholderEl.replaceWith(...nodes);
      if (firstNode instanceof Element) {
        this.placeholderEl = firstNode;
      }
    }

    this.onLoad?.();
  }

  cleanup(): void {
    if (this.observer) {
      this.observer.disconnect();
      this.observer = null;
    }
  }
}

export class PagefindSearchbox extends PagefindElement {
  static get observedAttributes(): string[] {
    return [
      "placeholder",
      "debounce",
      "autofocus",
      "show-sub-results",
      "max-results",
      "show-keyboard-hints",
    ];
  }

  containerEl: HTMLElement | null = null;
  inputEl: HTMLInputElement | null = null;
  dropdownEl: HTMLElement | null = null;
  resultsEl: HTMLElement | null = null;
  statusEl: HTMLElement | null = null;
  footerEl: HTMLElement | null = null;

  isOpen: boolean = false;
  isLoading: boolean = false;
  results: SearchboxResult[] = [];
  activeIndex: number = -1;
  searchID: number = 0;
  searchTerm: string = "";

  private _userPlaceholder: string | null = null;
  debounce: number = 150;
  autofocus: boolean = false;
  showSubResults: boolean = false;
  maxResults: number = 0; // 0 means no limit
  showKeyboardHints: boolean = true;

  resultTemplate: ((result: PagefindResultData) => TemplateResult) | null =
    null;

  private compiledResultTemplate: Template<SearchboxResultTemplateData> | null =
    null;
  private _documentClickHandler: ((e: MouseEvent) => void) | null = null;

  constructor() {
    super();
  }

  get placeholder(): string {
    return (
      this._userPlaceholder ||
      this.instance?.translate("placeholder") ||
      "Search..."
    );
  }

  private readAttributes(): void {
    if (this.hasAttribute("placeholder")) {
      this._userPlaceholder = this.getAttribute("placeholder");
    }
    if (this.hasAttribute("debounce")) {
      this.debounce =
        parseInt(this.getAttribute("debounce") || "150", 10) || 150;
    }
    if (this.hasAttribute("autofocus")) {
      this.autofocus = this.hasAttribute("autofocus");
    }
    if (this.hasAttribute("show-sub-results")) {
      this.showSubResults = this.getAttribute("show-sub-results") !== "false";
    }
    if (this.hasAttribute("max-results")) {
      this.maxResults = parseInt(this.getAttribute("max-results") || "0", 10);
    }
    if (this.hasAttribute("show-keyboard-hints")) {
      this.showKeyboardHints =
        this.getAttribute("show-keyboard-hints") !== "false";
    }
  }

  init(): void {
    this.readAttributes();
    this.checkForTemplates();
    this.render();
    this.setupOutsideClickHandler();
  }

  private checkForTemplates(): void {
    const resultScript = this.querySelector(
      'script[type="text/pagefind-template"]:not([data-template]), script[type="text/pagefind-template"][data-template="result"]',
    );
    if (resultScript) {
      this.compiledResultTemplate = compile(
        (resultScript.textContent || "").trim(),
      );
    }
  }

  render(): void {
    const savedScripts: HTMLScriptElement[] = [];
    this.querySelectorAll('script[type="text/pagefind-template"]').forEach(
      (s) => {
        savedScripts.push(s as HTMLScriptElement);
      },
    );

    this.innerHTML = "";

    savedScripts.forEach((s) => this.appendChild(s));

    const inputId = this.instance!.generateId("pf-sb-input");
    const resultsId = this.instance!.generateId("pf-sb-results");

    this.containerEl = document.createElement("div");
    this.containerEl.className = "pf-searchbox";
    this.appendChild(this.containerEl);

    const inputWrapper = document.createElement("div");
    inputWrapper.className = "pf-searchbox-input-wrapper";
    this.containerEl.appendChild(inputWrapper);

    this.inputEl = document.createElement("input");
    this.inputEl.id = inputId;
    this.inputEl.className = "pf-searchbox-input";
    this.inputEl.type = "text";
    this.inputEl.setAttribute("role", "combobox");
    this.inputEl.setAttribute("aria-autocomplete", "list");
    this.inputEl.setAttribute("aria-controls", resultsId);
    this.inputEl.setAttribute("aria-expanded", "false");
    this.inputEl.setAttribute("autocomplete", "off");
    this.inputEl.setAttribute("autocapitalize", "none");
    this.inputEl.placeholder = this.placeholder;
    if (this.autofocus) {
      this.inputEl.setAttribute("autofocus", "autofocus");
    }
    inputWrapper.appendChild(this.inputEl);

    this.dropdownEl = document.createElement("div");
    this.dropdownEl.className = "pf-searchbox-dropdown";
    this.containerEl.appendChild(this.dropdownEl);

    const resultsLabel =
      this.instance?.translate("results_label") || "Search results";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.resultsEl = document.createElement("div");
    this.resultsEl.id = resultsId;
    this.resultsEl.className = "pf-searchbox-results";
    this.resultsEl.setAttribute("role", "listbox");
    this.resultsEl.setAttribute("aria-label", resultsLabel);
    this.dropdownEl.appendChild(this.resultsEl);

    this.statusEl = document.createElement("div");
    this.statusEl.className = "pf-searchbox-status";
    this.statusEl.hidden = true;
    this.dropdownEl.appendChild(this.statusEl);

    if (this.showKeyboardHints) {
      this.footerEl = document.createElement("div");
      this.footerEl.className = "pf-searchbox-footer";
      this.footerEl.setAttribute("aria-hidden", "true");
      this.dropdownEl.appendChild(this.footerEl);

      this.renderFooterHints();
    }
    this.setupEventHandlers();
  }

  private renderFooterHints(): void {
    if (!this.footerEl) return;
    this.footerEl.innerHTML = "";

    const navigateText =
      this.instance?.translate("keyboard_navigate") || "navigate";
    const selectText = this.instance?.translate("keyboard_select") || "select";
    const closeText = this.instance?.translate("keyboard_close") || "close";

    const navHint = document.createElement("div");
    navHint.className = "pf-searchbox-footer-hint";
    const navKeyUp = document.createElement("span");
    navKeyUp.className = "pf-searchbox-footer-key";
    navKeyUp.textContent = "↑";
    navHint.appendChild(navKeyUp);
    const navKeyDown = document.createElement("span");
    navKeyDown.className = "pf-searchbox-footer-key";
    navKeyDown.textContent = "↓";
    navHint.appendChild(navKeyDown);
    navHint.appendChild(document.createTextNode(` ${navigateText}`));
    this.footerEl.appendChild(navHint);

    const selectHint = document.createElement("div");
    selectHint.className = "pf-searchbox-footer-hint";
    const selectKey = document.createElement("span");
    selectKey.className = "pf-searchbox-footer-key";
    selectKey.textContent = "↵";
    selectHint.appendChild(selectKey);
    selectHint.appendChild(document.createTextNode(` ${selectText}`));
    this.footerEl.appendChild(selectHint);

    const closeHint = document.createElement("div");
    closeHint.className = "pf-searchbox-footer-hint";
    const closeKey = document.createElement("span");
    closeKey.className = "pf-searchbox-footer-key";
    closeKey.textContent = "esc";
    closeHint.appendChild(closeKey);
    closeHint.appendChild(document.createTextNode(` ${closeText}`));
    this.footerEl.appendChild(closeHint);
  }

  private setupEventHandlers(): void {
    if (!this.inputEl || !this.resultsEl) return;

    this.inputEl.addEventListener("input", async (e) => {
      const value = (e.target as HTMLInputElement).value;
      this.searchTerm = value;

      if (!value || !value.trim()) {
        this.closeDropdown();
        this.results = [];
        this.instance?.triggerSearch("");
        return;
      }

      this.openDropdown();
      this.showLoadingState();

      const thisSearchID = ++this.searchID;
      await asyncSleep(this.debounce);

      if (thisSearchID !== this.searchID) {
        return;
      }

      this.instance?.triggerSearch(value);
    });

    this.inputEl.addEventListener("keydown", (e) => {
      switch (e.key) {
        case "ArrowDown":
          e.preventDefault();
          if (!this.isOpen && this.inputEl?.value.trim()) {
            this.openDropdown();
          }
          if (this.isOpen && this.results.length > 0) {
            this.moveSelection(1);
          }
          break;

        case "ArrowUp":
          e.preventDefault();
          if (this.isOpen && this.results.length > 0) {
            this.moveSelection(-1);
          }
          break;

        case "Enter":
          if (this.isOpen && this.activeIndex >= 0) {
            e.preventDefault();
            this.activateCurrentSelection(e);
          } else if (!this.isOpen && this.inputEl?.value.trim()) {
            e.preventDefault();
            this.openDropdown();
            if (this.results.length > 0) {
              this.rerenderLoadedResults();
              this.activeIndex = 0;
              this.updateSelectionUI();
            } else {
              this.instance?.triggerSearch(this.inputEl.value);
            }
          }
          break;

        case "Escape":
          if (this.isOpen) {
            e.preventDefault();
            this.closeDropdown();
          }
          break;

        case "Tab":
          if (this.isOpen) {
            this.closeDropdown();
          }
          break;
      }
    });

    this.inputEl.addEventListener("focus", () => {
      this.instance?.triggerLoad();
    });

    this.resultsEl.addEventListener("click", (e) => {
      const resultLink = (e.target as Element).closest("a.pf-searchbox-result");
      if (resultLink) {
        this.closeDropdown();
      }
    });

    this.resultsEl.addEventListener("mousemove", (e) => {
      const resultLink = (e.target as Element).closest("a.pf-searchbox-result");
      if (resultLink) {
        const index = this.getResultIndexFromElement(
          resultLink as HTMLAnchorElement,
        );
        if (index !== -1 && index !== this.activeIndex) {
          this.activeIndex = index;
          this.updateSelectionUI(false);
        }
      }
    });
  }

  private setupOutsideClickHandler(): void {
    this._documentClickHandler = (e: MouseEvent) => {
      if (this.isOpen && !this.contains(e.target as Node)) {
        this.closeDropdown();
      }
    };
    document.addEventListener("click", this._documentClickHandler);
  }

  private openDropdown(): void {
    if (this.isOpen || !this.containerEl || !this.inputEl) return;
    this.isOpen = true;
    this.containerEl.classList.add("open");
    this.inputEl.setAttribute("aria-expanded", "true");
  }

  private closeDropdown(): void {
    if (!this.isOpen || !this.containerEl || !this.inputEl) return;
    this.isOpen = false;
    this.containerEl.classList.remove("open");
    this.inputEl.setAttribute("aria-expanded", "false");
    this.inputEl.removeAttribute("aria-activedescendant");
    this.activeIndex = -1;
  }

  private showLoadingState(): void {
    if (!this.resultsEl || !this.statusEl) return;
    this.isLoading = true;
    this.resultsEl.innerHTML = "";
    this.resultsEl.setAttribute("aria-busy", "true");

    const searchingText =
      this.instance?.translate("searching", { SEARCH_TERM: this.searchTerm }) ||
      "Searching...";

    this.statusEl.textContent = searchingText;
    this.statusEl.className = "pf-searchbox-status pf-searchbox-loading";
    this.statusEl.hidden = false;
  }

  private showEmptyState(): void {
    if (!this.resultsEl || !this.statusEl) return;
    this.resultsEl.innerHTML = "";
    this.resultsEl.removeAttribute("aria-busy");

    const noResultsText =
      this.instance?.translate("zero_results", {
        SEARCH_TERM: this.searchTerm,
      }) || `No results for "${this.searchTerm}"`;

    this.statusEl.textContent = noResultsText;
    this.statusEl.className = "pf-searchbox-status pf-searchbox-empty";
    this.statusEl.hidden = false;

    this.instance?.announce(
      "zero_results",
      { SEARCH_TERM: this.searchTerm },
      "assertive",
    );
  }

  private moveSelection(delta: number): void {
    const totalItems = this.getTotalNavigableItems();
    if (totalItems === 0) return;

    let newIndex = this.activeIndex + delta;

    if (newIndex < -1) {
      newIndex = -1;
    } else if (newIndex >= totalItems) {
      newIndex = totalItems - 1;
    }

    this.activeIndex = newIndex;
    this.updateSelectionUI(true);
  }

  private getTotalNavigableItems(): number {
    if (!this.resultsEl) return 0;
    return this.resultsEl.querySelectorAll(".pf-searchbox-result").length;
  }

  private updateSelectionUI(scroll: boolean = false): void {
    if (!this.resultsEl || !this.inputEl) return;

    this.resultsEl.querySelectorAll("[data-pf-selected]").forEach((el) => {
      el.removeAttribute("data-pf-selected");
      el.setAttribute("aria-selected", "false");
    });

    const activeEl = this.getResultElementByIndex(this.activeIndex);

    if (activeEl) {
      activeEl.setAttribute("data-pf-selected", "");
      activeEl.setAttribute("aria-selected", "true");
      this.inputEl.setAttribute("aria-activedescendant", activeEl.id);
      if (scroll) {
        this.scrollToCenter(activeEl);
      }
    } else {
      this.inputEl.removeAttribute("aria-activedescendant");
    }
  }

  private scrollToCenter(el: HTMLElement): void {
    if (!this.resultsEl) return;
    const container = this.resultsEl;
    const elTop = el.offsetTop;
    const elHeight = el.offsetHeight;
    const containerHeight = container.clientHeight;
    const targetScroll = elTop - containerHeight / 2 + elHeight / 2;
    container.scrollTo({ top: targetScroll, behavior: "smooth" });
  }

  private getResultElementByIndex(index: number): HTMLElement | null {
    if (index < 0 || !this.resultsEl) return null;
    const allLinks = this.resultsEl.querySelectorAll("a.pf-searchbox-result");
    return (allLinks[index] as HTMLElement) || null;
  }

  private getResultIndexFromElement(el: HTMLAnchorElement): number {
    if (!this.resultsEl) return -1;
    const allLinks = Array.from(
      this.resultsEl.querySelectorAll("a.pf-searchbox-result"),
    );
    return allLinks.indexOf(el);
  }

  private activateCurrentSelection(keyboardEvent: KeyboardEvent): void {
    const activeEl = this.getResultElementByIndex(
      this.activeIndex,
    ) as HTMLAnchorElement | null;
    if (!activeEl || !activeEl.href) return;

    if (keyboardEvent.metaKey || keyboardEvent.ctrlKey) {
      window.open(activeEl.href, "_blank");
    } else if (keyboardEvent.shiftKey) {
      window.open(activeEl.href, "_blank");
    } else {
      window.location.href = activeEl.href;
    }

    this.closeDropdown();
  }

  private handleResults(searchResult: PagefindSearchResult): void {
    this.isLoading = false;

    if (this.resultsEl) {
      this.resultsEl.removeAttribute("aria-busy");
    }
    if (this.statusEl) {
      this.statusEl.hidden = true;
    }

    for (const result of this.results) {
      result.cleanup();
    }

    if (!searchResult.results || searchResult.results.length === 0) {
      this.results = [];
      this.showEmptyState();
      return;
    }

    const limitedResults =
      this.maxResults > 0
        ? searchResult.results.slice(0, this.maxResults)
        : searchResult.results;

    if (this.resultsEl) {
      this.resultsEl.innerHTML = "";
    }

    const renderer = this.getResultRenderer();

    this.results = limitedResults.map((rawResult) => {
      const placeholderHtml = defaultPlaceholderTemplate({});
      const placeholderNodes = templateNodes(placeholderHtml);
      const placeholderEl = placeholderNodes[0] as Element;

      if (this.resultsEl && placeholderEl) {
        this.resultsEl.appendChild(placeholderEl);
      }

      return new SearchboxResult({
        rawResult,
        placeholderEl,
        renderFn: renderer,
        intersectionRoot: this.resultsEl,
        onLoad: () => {
          this.updateSelectionUI();
        },
      });
    });

    this.activeIndex = 0;
    this.updateSelectionUI();
    this.announceResults();
  }

  private buildTemplateData(
    result: PagefindResultData,
  ): SearchboxResultTemplateData {
    const subResults = this.showSubResults
      ? this.instance!.getDisplaySubResults(result)
      : [];

    const resultId = this.instance!.generateId("pf-sb-result");

    return {
      meta: result.meta || {},
      excerpt: result.excerpt || "",
      url: result.url || "",
      sub_results: subResults.map((sr) => {
        const subResultId = this.instance!.generateId("pf-sb-result");
        return {
          title: sr.title,
          url: sr.url,
          excerpt: sr.excerpt,
          aria: {
            result_id: subResultId,
            title_id: `${subResultId}-title`,
            excerpt_id: `${subResultId}-excerpt`,
          },
        };
      }),
      options: {
        show_sub_results: this.showSubResults,
      },
      aria: {
        result_id: resultId,
        title_id: `${resultId}-title`,
        excerpt_id: `${resultId}-excerpt`,
      },
    };
  }

  /**
   * Returns the render function for results.
   * Priority: JS function > script template > default template
   */
  private getResultRenderer(): (result: PagefindResultData) => TemplateResult {
    if (this.resultTemplate) {
      return this.resultTemplate;
    }

    if (this.compiledResultTemplate) {
      const template = this.compiledResultTemplate;
      return (result) => {
        const data = this.buildTemplateData(result);
        return template(data);
      };
    }

    return (result) => {
      const data = this.buildTemplateData(result);
      return defaultResultTemplate(data);
    };
  }

  private rerenderLoadedResults(): void {
    if (!this.resultsEl) return;
    this.resultsEl.innerHTML = "";

    for (const result of this.results) {
      if (result.data) {
        const templateData = this.buildTemplateData(result.data);
        let templateResult: TemplateResult;
        if (this.resultTemplate) {
          templateResult = this.resultTemplate(result.data);
        } else if (this.compiledResultTemplate) {
          templateResult = this.compiledResultTemplate(templateData);
        } else {
          templateResult = defaultResultTemplate(templateData);
        }
        const nodes = templateNodes(templateResult);
        for (const node of nodes) {
          if (node instanceof Element) {
            this.resultsEl.appendChild(node);
            result.placeholderEl = node;
            break;
          }
        }
        for (const node of nodes.slice(1)) {
          this.resultsEl.appendChild(node);
        }
      } else {
        const placeholderHtml = defaultPlaceholderTemplate({});
        const placeholderNodes = templateNodes(placeholderHtml);
        const placeholderEl = placeholderNodes[0] as Element;
        if (placeholderEl) {
          this.resultsEl.appendChild(placeholderEl);
          result.placeholderEl = placeholderEl;
          result.cleanup();
          result.setupObserver();
        }
      }
    }
  }

  private announceResults(): void {
    const count = this.results.length;
    if (count === 0) {
      this.instance?.announce(
        "zero_results",
        { SEARCH_TERM: this.searchTerm },
        "assertive",
      );
    } else {
      const key = count === 1 ? "one_result" : "many_results";
      this.instance?.announce(key, {
        SEARCH_TERM: this.searchTerm,
        COUNT: count,
      });
    }
  }

  register(instance: Instance): void {
    instance.registerInput(this, {
      keyboardNavigation: true,
    });
    instance.registerResults(this, {
      keyboardNavigation: true,
      announcements: true,
    });

    instance.on(
      "loading",
      () => {
        if (this.searchTerm && this.searchTerm.trim()) {
          this.openDropdown();
          this.showLoadingState();
        }
      },
      this,
    );

    instance.on(
      "results",
      (results: unknown) => {
        this.handleResults(results as PagefindSearchResult);
      },
      this,
    );

    instance.on(
      "error",
      (error: unknown) => {
        const err = error as PagefindError;
        this.isLoading = false;
        const errorText = instance.translate("error_search") || "Search failed";
        this.showError({
          message: err.message || errorText,
          details: err.bundlePath
            ? `Bundle path: ${err.bundlePath}`
            : undefined,
        });

        instance.announce("error_search", {}, "assertive");
      },
      this,
    );

    instance.on(
      "search",
      (term: unknown) => {
        if (this.inputEl && document.activeElement !== this.inputEl) {
          this.inputEl.value = term as string;
          this.searchTerm = term as string;
        }
      },
      this,
    );

    instance.on(
      "translations",
      () => {
        const currentValue = this.inputEl?.value || "";
        const wasOpen = this.isOpen;
        this.render();
        if (this.inputEl && currentValue) {
          this.inputEl.value = currentValue;
        }
        if (wasOpen) {
          this.openDropdown();
          if (this.results.length > 0) {
            this.rerenderLoadedResults();
            this.updateSelectionUI();
          }
        }
      },
      this,
    );
  }

  cleanup(): void {
    for (const result of this.results) {
      result.cleanup();
    }
    this.results = [];

    if (this._documentClickHandler) {
      document.removeEventListener("click", this._documentClickHandler);
      this._documentClickHandler = null;
    }
  }

  update(): void {
    this.readAttributes();
    if (this._documentClickHandler) {
      document.removeEventListener("click", this._documentClickHandler);
      this._documentClickHandler = null;
    }
    this.render();
    this.setupOutsideClickHandler();
  }

  focus(): void {
    if (this.inputEl) {
      this.inputEl.focus();
    }
  }
}

if (!customElements.get("pagefind-searchbox")) {
  customElements.define("pagefind-searchbox", PagefindSearchbox);
}
