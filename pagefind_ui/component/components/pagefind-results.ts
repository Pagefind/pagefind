import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import { compile, type Template } from "adequate-little-templates";
import type {
  PagefindSearchResult,
  PagefindRawResult,
  PagefindResultData,
  PagefindError,
} from "../types";

type TemplateResult = Element | Element[] | string;

interface ResultTemplateData {
  meta: Record<string, string | undefined>;
  excerpt: string;
  url: string;
  sub_results: Array<{ title: string; url: string; excerpt: string }>;
  options: {
    link_target: string | null;
    show_images: boolean;
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
    `[Pagefind Results]: Expected template to return HTML element or string, got ${typeof templateResult}`,
  );
  return [];
};

const DEFAULT_RESULT_TEMPLATE = `<li class="pf-result">
  <div class="pf-result-card">
    {{#if and(options.show_images, meta.image)}}
    <img class="pf-result-image" src="{{ meta.image }}" alt="{{ meta.image_alt | default(meta.title) }}">
    {{/if}}
    <div class="pf-result-content">
      <p class="pf-result-title">
        <a class="pf-result-link" href="{{ meta.url | default(url) | safeUrl }}"{{#if options.link_target}} target="{{ options.link_target }}"{{/if}}{{#if eq(options.link_target, "_blank")}} rel="noopener"{{/if}}>{{ meta.title }}</a>
      </p>
      {{#if excerpt}}
      <p class="pf-result-excerpt">{{+ excerpt +}}</p>
      {{/if}}
    </div>
  </div>
  {{#if sub_results}}
  <ul class="pf-heading-chips">
    {{#each sub_results as sub}}
    <li class="pf-heading-chip">
      <a class="pf-heading-link" href="{{ sub.url | safeUrl }}"{{#if options.link_target}} target="{{ options.link_target }}"{{/if}}{{#if eq(options.link_target, "_blank")}} rel="noopener"{{/if}}>{{ sub.title }}</a>
      <p class="pf-heading-excerpt">{{+ sub.excerpt +}}</p>
    </li>
    {{/each}}
  </ul>
  {{/if}}
</li>`;

const DEFAULT_PLACEHOLDER_TEMPLATE = `<li class="pf-result" aria-hidden="true">
  <div class="pf-result-card">
    <div class="pf-skeleton pf-skeleton-image"></div>
    <div class="pf-result-content">
      <p class="pf-result-title pf-skeleton pf-skeleton-title"></p>
      <p class="pf-result-excerpt pf-skeleton pf-skeleton-excerpt"></p>
    </div>
  </div>
</li>`;

const defaultResultTemplate: Template<ResultTemplateData> = compile(
  DEFAULT_RESULT_TEMPLATE,
);
const defaultPlaceholderTemplate: Template<Record<string, never>> = compile(
  DEFAULT_PLACEHOLDER_TEMPLATE,
);

const nearestScrollParent = (el: Element | null): Element | null => {
  if (!(el instanceof HTMLElement)) return null;
  const overflowY = window.getComputedStyle(el).overflowY;
  const isScrollable = overflowY !== "visible" && overflowY !== "hidden";
  return isScrollable
    ? el
    : nearestScrollParent(el.parentNode as Element | null);
};

interface ResultRenderOptions {
  showImages: boolean;
  showSubResults: boolean;
  maxSubResults: number;
  linkTarget: string | null;
}

interface ResultOptions {
  result: PagefindRawResult;
  placeholderNodes: Node[];
  resultFn: (
    result: PagefindResultData,
    options: ResultRenderOptions,
  ) => TemplateResult;
  intersectionEl: Element | null;
  showImages: boolean;
  showSubResults: boolean;
  maxSubResults: number;
  linkTarget: string | null;
}

class Result {
  rawResult: PagefindRawResult;
  placeholderNodes: Node[];
  resultFn: (
    result: PagefindResultData,
    options: ResultRenderOptions,
  ) => TemplateResult;
  intersectionEl: Element | null;
  showImages: boolean;
  showSubResults: boolean;
  maxSubResults: number;
  linkTarget: string | null;
  result: PagefindResultData | null = null;
  private observer: IntersectionObserver | null = null;

  constructor(opts: ResultOptions) {
    this.rawResult = opts.result;
    this.placeholderNodes = opts.placeholderNodes;
    this.resultFn = opts.resultFn;
    this.intersectionEl = opts.intersectionEl;
    this.showImages = opts.showImages;
    this.showSubResults = opts.showSubResults;
    this.maxSubResults = opts.maxSubResults;
    this.linkTarget = opts.linkTarget;
    this.setupObserver();
  }

  setupObserver(): void {
    if (this.result !== null || this.observer !== null) return;
    if (!this.placeholderNodes?.length) return;

    const options = {
      root: this.intersectionEl,
      rootMargin: "50px", // Start loading slightly before visible
      threshold: 0.01,
    };

    this.observer = new IntersectionObserver((entries, obs) => {
      if (this.result !== null) return;
      if (entries?.[0]?.isIntersecting) {
        this.load();
        obs.disconnect();
        this.observer = null;
      }
    }, options);

    this.observer.observe(this.placeholderNodes[0] as Element);
  }

  async load(): Promise<void> {
    if (!this.placeholderNodes?.length) return;

    this.result = await this.rawResult.data();
    const resultTemplate = this.resultFn(this.result, {
      showImages: this.showImages,
      showSubResults: this.showSubResults,
      maxSubResults: this.maxSubResults,
      linkTarget: this.linkTarget,
    });
    const resultNodes = templateNodes(resultTemplate);

    while (this.placeholderNodes.length > 1) {
      const node = this.placeholderNodes.pop();
      if (node instanceof Element) node.remove();
    }

    const firstNode = this.placeholderNodes[0];
    if (firstNode instanceof Element) {
      firstNode.replaceWith(...resultNodes);
    }
  }

  cleanup(): void {
    if (this.observer) {
      this.observer.disconnect();
      this.observer = null;
    }
  }
}

export class PagefindResults extends PagefindElement {
  static get observedAttributes(): string[] {
    return [
      "show-images",
      "hide-sub-results",
      "max-sub-results",
      "link-target",
    ];
  }

  containerEl: HTMLUListElement | null = null;
  intersectionEl: Element | null = document.body;
  results: Result[] = [];
  showImages: boolean = false;
  hideSubResults: boolean = false;
  maxSubResults: number = 3;
  linkTarget: string | null = null;

  resultTemplate: ((result: PagefindResultData) => TemplateResult) | null =
    null;

  private compiledResultTemplate: Template<ResultTemplateData> | null = null;
  private compiledPlaceholderTemplate: Template<Record<string, never>> | null =
    null;

  selectedIndex: number = -1;

  constructor() {
    super();
  }

  init(): void {
    if (this.hasAttribute("show-images")) {
      this.showImages = this.getAttribute("show-images") !== "false";
    }
    if (this.hasAttribute("hide-sub-results")) {
      this.hideSubResults = this.getAttribute("hide-sub-results") !== "false";
    }
    if (this.hasAttribute("max-sub-results")) {
      this.maxSubResults =
        parseInt(this.getAttribute("max-sub-results") || "3", 10) || 3;
    }
    if (this.hasAttribute("link-target")) {
      this.linkTarget = this.getAttribute("link-target");
    }

    this.checkForTemplates();
    this.render();
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

    const placeholderScript = this.querySelector(
      'script[type="text/pagefind-template"][data-template="placeholder"]',
    );
    if (placeholderScript) {
      this.compiledPlaceholderTemplate = compile(
        (placeholderScript.textContent || "").trim(),
      );
    }
  }

  private buildTemplateData(
    result: PagefindResultData,
    options: ResultRenderOptions,
  ): ResultTemplateData {
    const subResults = options.showSubResults
      ? this.instance!.getDisplaySubResults(result, options.maxSubResults)
      : [];

    return {
      meta: result.meta || {},
      excerpt: result.excerpt || "",
      url: result.url || "",
      sub_results: subResults.map((sr) => ({
        title: sr.title,
        url: sr.url,
        excerpt: sr.excerpt,
      })),
      options: {
        link_target: options.linkTarget,
        show_images: options.showImages,
      },
    };
  }

  /**
   * Returns the internal render function used by the Result class.
   * Priority: JS function > script template > default template
   */
  private getResultRenderer(): (
    result: PagefindResultData,
    options: ResultRenderOptions,
  ) => TemplateResult {
    if (this.resultTemplate) {
      const userFn = this.resultTemplate;
      return (result, _options) => userFn(result);
    }

    if (this.compiledResultTemplate) {
      const template = this.compiledResultTemplate;
      return (result, options) => {
        const data = this.buildTemplateData(result, options);
        return template(data);
      };
    }

    return (result, options) => {
      const data = this.buildTemplateData(result, options);
      return defaultResultTemplate(data);
    };
  }

  private getPlaceholder(): string {
    if (this.compiledPlaceholderTemplate) {
      return this.compiledPlaceholderTemplate({});
    }
    return defaultPlaceholderTemplate({});
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

    const resultsLabel =
      this.instance?.translate("results_label") || "Search results";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.containerEl = document.createElement("ul");
    this.containerEl.className = "pf-results";
    this.containerEl.setAttribute("aria-label", resultsLabel);
    this.containerEl.setAttribute("aria-busy", "false");
    this.appendChild(this.containerEl);

    this.setupKeyboardHandlers();
  }

  appendResults(nodes: Node[]): void {
    if (!this.containerEl) return;
    for (const node of nodes) {
      this.containerEl.appendChild(node);
    }
  }

  register(instance: Instance): void {
    instance.registerResults(this, {
      keyboardNavigation: true,
      announcements: true,
    });

    instance.on(
      "results",
      (results: unknown) => {
        if (!this.containerEl) return;
        const searchResult = results as PagefindSearchResult;

        for (const result of this.results) {
          result.cleanup();
        }

        this.containerEl.innerHTML = "";
        this.containerEl.setAttribute("aria-busy", "false");
        this.intersectionEl = nearestScrollParent(this.containerEl);

        this.selectedIndex = -1;

        const count = searchResult?.results?.length ?? 0;
        const term = instance.searchTerm;
        if (term) {
          const key =
            count === 0
              ? "zero_results"
              : count === 1
                ? "one_result"
                : "many_results";
          const priority = count === 0 ? "assertive" : "polite";
          instance.announce(key, { SEARCH_TERM: term, COUNT: count }, priority);
        } else if (instance.faceted) {
          const key =
            count === 0
              ? "total_zero_results"
              : count === 1
                ? "total_one_result"
                : "total_many_results";
          const priority = count === 0 ? "assertive" : "polite";
          instance.announce(key, { COUNT: count }, priority);
        }

        const resultRenderer = this.getResultRenderer();
        this.results = searchResult.results.map((r) => {
          const placeholderNodes = templateNodes(this.getPlaceholder());
          this.appendResults(placeholderNodes);

          return new Result({
            result: r,
            placeholderNodes,
            resultFn: resultRenderer,
            intersectionEl: this.intersectionEl,
            showImages: this.showImages,
            showSubResults: !this.hideSubResults,
            maxSubResults: this.maxSubResults,
            linkTarget: this.linkTarget,
          });
        });
      },
      this,
    );

    instance.on(
      "loading",
      () => {
        if (!this.containerEl) return;

        this.containerEl.innerHTML = "";
        this.containerEl.setAttribute("aria-busy", "true");
        this.selectedIndex = -1;
      },
      this,
    );

    instance.on(
      "error",
      (error: unknown) => {
        const err = error as PagefindError;
        if (this.containerEl) {
          this.containerEl.setAttribute("aria-busy", "false");
        }

        instance.announce("error_search", {}, "assertive");

        this.showError({
          message:
            err.message ||
            instance.translate("error_search") ||
            "Failed to load search results",
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
        this.render();
      },
      this,
    );
  }

  getResultElements(): HTMLElement[] {
    if (!this.containerEl) return [];
    return Array.from(this.containerEl.querySelectorAll(".pf-result"));
  }

  getNavigableAnchors(): HTMLAnchorElement[] {
    if (!this.containerEl) return [];
    return Array.from(this.containerEl.querySelectorAll(".pf-result a"));
  }

  private setupKeyboardHandlers(): void {
    if (!this.containerEl) return;

    this.containerEl.addEventListener("keydown", (e) => {
      const anchor = (e.target as Element).closest("a");
      if (!anchor) return;

      const anchors = this.getNavigableAnchors();
      const index = anchors.indexOf(anchor as HTMLAnchorElement);
      if (index === -1) return;

      if (e.key === "ArrowDown") {
        e.preventDefault();
        if (index < anchors.length - 1) {
          const next = anchors[index + 1];
          next.focus();
          this.scrollToCenter(next, e.repeat);
        }
      } else if (e.key === "ArrowUp") {
        e.preventDefault();
        if (index > 0) {
          const prev = anchors[index - 1];
          prev.focus();
          this.scrollToCenter(prev, e.repeat);
        } else {
          // At first anchor, go back to input
          this.instance?.focusPreviousInput(document.activeElement as Element);
        }
      } else if (e.key === "Backspace") {
        e.preventDefault();
        this.instance?.focusInputAndDelete(document.activeElement as Element);
      } else if (e.key === "/") {
        e.preventDefault();
        this.instance?.focusPreviousInput(document.activeElement as Element);
      } else if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
        // Printable character - return to input and type it
        e.preventDefault();
        this.instance?.focusInputAndType(
          document.activeElement as Element,
          e.key,
        );
      }
    });

    this.containerEl.addEventListener("focusin", (e) => {
      const anchor = (e.target as Element).closest("a");
      if (!anchor) return;

      this.clearSelection();
      anchor.setAttribute("data-pf-selected", "");

      const navigateText =
        this.instance?.translate("keyboard_navigate") || "navigate";
      const selectText =
        this.instance?.translate("keyboard_select") || "select";
      const searchText =
        this.instance?.translate("keyboard_search") || "search";
      this.instance?.registerShortcut(
        { label: "↑↓", description: navigateText },
        this,
      );
      this.instance?.registerShortcut(
        { label: "↵", description: selectText },
        this,
      );
      this.instance?.registerShortcut(
        { label: "/", description: searchText },
        this,
      );
    });

    this.containerEl.addEventListener("focusout", (e) => {
      const focusEvent = e as FocusEvent;
      if (!this.containerEl?.contains(focusEvent.relatedTarget as Node)) {
        this.clearSelection();
        this.instance?.deregisterAllShortcuts(this);
      }
    });
  }

  private scrollToCenter(el: HTMLElement, instant: boolean = false): void {
    const container = this.intersectionEl || nearestScrollParent(el);
    if (!container || !(container instanceof HTMLElement)) return;
    if (container === document.body || container === document.documentElement)
      return;

    const elRect = el.getBoundingClientRect();
    const containerRect = container.getBoundingClientRect();
    const elRelativeTop = elRect.top - containerRect.top + container.scrollTop;
    const targetScroll =
      elRelativeTop - container.clientHeight / 2 + el.offsetHeight / 2;
    container.scrollTo({
      top: targetScroll,
      behavior: instant ? "instant" : "smooth",
    });
  }

  clearSelection(): void {
    this.containerEl
      ?.querySelectorAll("[data-pf-selected]")
      .forEach((el) => el.removeAttribute("data-pf-selected"));
  }

  cleanup(): void {
    for (const result of this.results) {
      result.cleanup();
    }
    this.results = [];
  }

  update(): void {
    this.render();
  }
}

if (!customElements.get("pagefind-results")) {
  customElements.define("pagefind-results", PagefindResults);
}
