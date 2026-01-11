import {
  findNextComponentInTabOrder,
  findPreviousComponentInTabOrder,
} from "./focus-utils";
import { getTranslations, interpolate } from "./translations";
import { Announcer } from "./announcer";
import type {
  AnnouncerPriority,
  PagefindSearchResult,
  PagefindResultData,
  PagefindSubResult,
  FilterCounts,
  FilterSelection,
  InstanceOptions,
  MergeIndexConfig,
  InstanceEvent,
  PagefindError,
  TranslationStrings,
  TextDirection,
  ComponentCapabilities,
  Shortcut,
  HookCallback,
  HookEntry,
  PagefindAPI,
} from "../types";

export interface PagefindComponent extends HTMLElement {
  instance: Instance | null;
  componentType?: string;
  componentSubtype?: string | null;
  capabilities?: ComponentCapabilities;
  inputEl?: HTMLInputElement | null;
  reconcileAria?: () => void;
  register?: (instance: Instance) => void;
  render?: () => void;
  getResultElements?: () => HTMLElement[];
}

let scriptBundlePath: string | undefined;
try {
  // Important: Check that the element is indeed a <script> node, to avoid a DOM clobbering vulnerability
  if (
    document?.currentScript &&
    document.currentScript.tagName.toUpperCase() === "SCRIPT"
  ) {
    const match = new URL(
      (document.currentScript as HTMLScriptElement).src,
    ).pathname.match(/^(.*\/)(?:pagefind[-_])?component[-_]?ui.js.*$/);
    if (match) {
      scriptBundlePath = match[1];
    }
  }
} catch (e) {
  scriptBundlePath = "/pagefind/";
}

type HookMap = {
  [K in InstanceEvent]: Array<HookCallback | HookEntry>;
};

export class Instance {
  private __pagefind__: PagefindAPI | null = null;
  private __loadPromise__: Promise<void> | null = null;
  private __searchID__: number = 0;
  private __hooks__: HookMap;

  private _translations: TranslationStrings | null = null;
  private _userTranslations: Record<string, string> = {};
  private _direction: TextDirection = "ltr";
  private _languageSet: boolean = false;

  private _announcer: Announcer;

  components: PagefindComponent[] = [];
  componentsByType: Record<string, PagefindComponent[]> = {};

  searchTerm: string = "";
  searchFilters: FilterSelection = {};
  searchResult: PagefindSearchResult = { results: [] };
  availableFilters: FilterCounts | null = null;
  totalFilters: FilterCounts | null = null;
  activeShortcuts: Shortcut[] = [];
  faceted: boolean = false;

  name: string;
  private generatedIds: Set<string> = new Set();
  options: {
    bundlePath: string;
    mergeIndex: MergeIndexConfig[];
  };
  pagefindOptions: Record<string, unknown>;

  constructor(name: string, opts: InstanceOptions = {}) {
    this.name = name;
    this.__hooks__ = {
      search: [],
      filters: [],
      loading: [],
      results: [],
      error: [],
      translations: [],
    };

    this.options = {
      bundlePath: opts.bundlePath ?? scriptBundlePath ?? "/pagefind/",
      mergeIndex: opts.mergeIndex ?? [],
    };

    const pagefindOpts = { ...opts };
    delete pagefindOpts.bundlePath;
    delete pagefindOpts.mergeIndex;

    this.pagefindOptions = pagefindOpts;

    this._announcer = new Announcer(this.generateId.bind(this));
  }

  generateId(prefix: string, length = 2): string {
    const idChars = "abcdef";
    const randomSeg = (len = 3): string => {
      let word = "";
      for (let i = 0; i < len; i++) {
        word += idChars[Math.floor(Math.random() * idChars.length)];
      }
      return word;
    };

    const instancePart = this.name !== "default" ? `${this.name}-` : "";
    const segments = Array.from({ length }, () => randomSeg()).join("-");
    const id = `${prefix}-${instancePart}${segments}`;

    if (this.generatedIds.has(id) || document.getElementById(id)) {
      return this.generateId(prefix, length + 1);
    }

    this.generatedIds.add(id);
    return id;
  }

  add(component: PagefindComponent): void {
    component?.register?.(this);
    this.components.push(component);
  }

  registerInput(
    component: PagefindComponent,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "input", null, capabilities);
  }

  registerResults(
    component: PagefindComponent,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "results", null, capabilities);
  }

  registerSummary(
    component: PagefindComponent,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "summary", null, capabilities);
  }

  registerFilter(
    component: PagefindComponent,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "filter", null, capabilities);
  }

  registerSort(
    component: PagefindComponent,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "sort", null, capabilities);
  }

  registerUtility(
    component: PagefindComponent,
    subtype: string | null = null,
    capabilities: ComponentCapabilities = {},
  ): void {
    this._registerComponent(component, "utility", subtype, capabilities);
  }

  private _registerComponent(
    component: PagefindComponent,
    type: string,
    subtype: string | null = null,
    capabilities: ComponentCapabilities = {},
  ): void {
    if (!this.componentsByType[type]) {
      this.componentsByType[type] = [];
    }

    // Auto-detect the language of this html page
    // on first component registration
    if (!this._languageSet) {
      this.setLanguage();
    }

    if (this.components.includes(component)) {
      // Update capabilities but don't re-add
      component.capabilities = capabilities;
      this.reconcileAria();
      return;
    }

    component.componentType = type;
    component.componentSubtype = subtype;
    component.capabilities = capabilities;
    this.componentsByType[type].push(component);
    this.components.push(component);

    this.reconcileAria();
  }

  getInputs(requiredCapability: string | null = null): PagefindComponent[] {
    const components = this.componentsByType["input"] || [];
    if (!requiredCapability) return components;
    return components.filter((c) => c.capabilities?.[requiredCapability]);
  }

  getResults(requiredCapability: string | null = null): PagefindComponent[] {
    const components = this.componentsByType["results"] || [];
    if (!requiredCapability) return components;
    return components.filter((c) => c.capabilities?.[requiredCapability]);
  }

  getSummaries(requiredCapability: string | null = null): PagefindComponent[] {
    const components = this.componentsByType["summary"] || [];
    if (!requiredCapability) return components;
    return components.filter((c) => c.capabilities?.[requiredCapability]);
  }

  getFilters(requiredCapability: string | null = null): PagefindComponent[] {
    const components = this.componentsByType["filter"] || [];
    if (!requiredCapability) return components;
    return components.filter((c) => c.capabilities?.[requiredCapability]);
  }

  getSorts(requiredCapability: string | null = null): PagefindComponent[] {
    const components = this.componentsByType["sort"] || [];
    if (!requiredCapability) return components;
    return components.filter((c) => c.capabilities?.[requiredCapability]);
  }

  getUtilities(
    subtype: string | null = null,
    requiredCapability: string | null = null,
  ): PagefindComponent[] {
    let utilities = this.componentsByType["utility"] || [];
    if (subtype !== null) {
      utilities = utilities.filter((u) => u.componentSubtype === subtype);
    }
    if (requiredCapability) {
      utilities = utilities.filter((c) => c.capabilities?.[requiredCapability]);
    }
    return utilities;
  }

  /**
   * Check if any component has registered announcement capability.
   * Used to determine if Instance should handle announcements as a fallback.
   */
  hasAnnouncementCapability(): boolean {
    return this.components.some((c) => c.capabilities?.announcements === true);
  }

  /**
   * Register an active shortcut. Triggers hints to re-render.
   */
  registerShortcut(shortcut: Omit<Shortcut, "owner">, owner: Element): void {
    const entry: Shortcut = { ...shortcut, owner };
    this.activeShortcuts.push(entry);
    this.notifyShortcutsChanged();
  }

  /**
   * Deregister a shortcut by owner + label.
   */
  deregisterShortcut(label: string, owner: Element): void {
    this.activeShortcuts = this.activeShortcuts.filter(
      (s) => !(s.label === label && s.owner === owner),
    );
    this.notifyShortcutsChanged();
  }

  /**
   * Deregister all shortcuts from an owner.
   */
  deregisterAllShortcuts(owner: Element): void {
    this.activeShortcuts = this.activeShortcuts.filter(
      (s) => s.owner !== owner,
    );
    this.notifyShortcutsChanged();
  }

  /**
   * Get currently active shortcuts.
   */
  getActiveShortcuts(): Shortcut[] {
    return this.activeShortcuts;
  }

  /**
   * Notify keyboard-hints utilities to re-render
   * due to shortcuts changing
   */
  notifyShortcutsChanged(): void {
    const hints = this.getUtilities("keyboard-hints");
    hints.forEach((h) => h.render?.());
  }

  /**
   * Focus the first result in the next keyboard-navigable results component.
   */
  focusNextResults(fromElement: Element): boolean {
    const results = this.getResults("keyboardNavigation");
    const resultsComponent = findNextComponentInTabOrder(fromElement, results);
    if (!resultsComponent) return false;

    const resultEls =
      (resultsComponent as PagefindComponent).getResultElements?.() ||
      resultsComponent.querySelectorAll(".pf-result") ||
      [];
    const firstLink = resultEls[0]?.querySelector("a");
    if (firstLink) {
      (firstLink as HTMLElement).focus();
      return true;
    }
    return false;
  }

  /**
   * Focus the previous keyboard-navigable input component.
   */
  focusPreviousInput(fromElement: Element): boolean {
    const inputs = this.getInputs("keyboardNavigation");
    const inputComponent = findPreviousComponentInTabOrder(
      fromElement,
      inputs,
    ) as PagefindComponent | null;
    if (!inputComponent) return false;

    if (inputComponent.focus) {
      inputComponent.focus();
      return true;
    }
    const inputEl = inputComponent.querySelector("input");
    if (inputEl) {
      inputEl.focus();
      return true;
    }
    return false;
  }

  /**
   * Focus previous keyboard-navigable input and append a character.
   */
  focusInputAndType(fromElement: Element, char: string): void {
    const inputs = this.getInputs("keyboardNavigation");
    const inputComponent = findPreviousComponentInTabOrder(
      fromElement,
      inputs,
    ) as PagefindComponent | null;
    const inputEl =
      inputComponent?.inputEl || inputComponent?.querySelector("input");
    if (inputEl) {
      inputEl.value += char;
      inputEl.focus();
      inputEl.dispatchEvent(new Event("input", { bubbles: true }));
    }
  }

  /**
   * Focus previous keyboard-navigable input and delete last character.
   */
  focusInputAndDelete(fromElement: Element): void {
    const inputs = this.getInputs("keyboardNavigation");
    const inputComponent = findPreviousComponentInTabOrder(
      fromElement,
      inputs,
    ) as PagefindComponent | null;
    const inputEl =
      inputComponent?.inputEl || inputComponent?.querySelector("input");
    if (inputEl) {
      inputEl.value = inputEl.value.slice(0, -1);
      inputEl.focus();
      inputEl.dispatchEvent(new Event("input", { bubbles: true }));
    }
  }

  /**
   * Trigger ARIA reconciliation on all registered components.
   */
  reconcileAria(): void {
    this.components.forEach((c) => c.reconcileAria?.());
  }

  /**
   * Get current text direction.
   */
  get direction(): TextDirection {
    return this._direction;
  }

  /**
   * Set the language for translations.
   */
  setLanguage(langCode?: string): void {
    if (!langCode) {
      langCode = document?.documentElement?.lang || "en";
    }

    this._translations = getTranslations(langCode);
    this._direction = (this._translations.direction as TextDirection) || "ltr";
    this._languageSet = true;

    this.__dispatch__("translations", this._translations, this._direction);
  }

  /**
   * Set user translation overrides.
   */
  setTranslations(overrides: Record<string, string>): void {
    this._userTranslations = { ...this._userTranslations, ...overrides };
    this.__dispatch__("translations", this._translations, this._direction);
  }

  /**
   * Get a translated string.
   */
  translate(
    key: string,
    replacements: Record<string, string | number> = {},
  ): string {
    const str = this._userTranslations[key] ?? this._translations?.[key];
    return interpolate(typeof str === "string" ? str : undefined, replacements);
  }

  /**
   * Announce a message to screen readers using a translation key.
   */
  announce(
    key: string,
    replacements: Record<string, string | number> = {},
    priority: AnnouncerPriority = "polite",
  ): void {
    const message = this.translate(key, replacements);
    if (message) {
      this._announcer.announce(message, priority);
    }
  }

  /**
   * Announce a raw message to screen readers (bypasses translation system).
   */
  announceRaw(message: string, priority: AnnouncerPriority = "polite"): void {
    this._announcer.announce(message, priority);
  }

  /**
   * Clear any pending announcements.
   */
  clearAnnouncements(): void {
    this._announcer.clear();
  }

  on(
    event: InstanceEvent,
    callback: HookCallback,
    owner: Element | null = null,
  ): void {
    if (!this.__hooks__[event]) {
      const supportedEvents = Object.keys(this.__hooks__).join(", ");
      console.error(
        `[Pagefind Component UI]: Unknown event type ${event}. Supported events: [${supportedEvents}]`,
      );
      return;
    }
    if (typeof callback !== "function") {
      console.error(
        `[Pagefind Component UI]: Expected callback to be a function, received ${typeof callback}`,
      );
      return;
    }

    // If owner provided, check for existing handler from same owner to prevent duplicates
    if (owner) {
      const existingIndex = this.__hooks__[event].findIndex(
        (h) => typeof h === "object" && h.owner === owner,
      );
      if (existingIndex !== -1) {
        this.__hooks__[event][existingIndex] = { callback, owner };
        return;
      }
      this.__hooks__[event].push({ callback, owner });
    } else {
      this.__hooks__[event].push(callback);
    }
  }

  triggerLoad(): Promise<void> {
    return this.__load__();
  }

  triggerSearch(term: string): void {
    this.searchTerm = term;
    this.__dispatch__("search", term, this.searchFilters);
    this.__search__(term, this.searchFilters);
  }

  triggerSearchWithFilters(term: string, filters: FilterSelection): void {
    this.searchTerm = term;
    this.searchFilters = filters;
    this.__dispatch__("search", term, filters);
    this.__search__(term, filters);
  }

  triggerFilters(filters: FilterSelection): void {
    this.searchFilters = filters;
    this.__dispatch__("search", this.searchTerm, filters);
    this.__search__(this.searchTerm, filters);
  }

  triggerFilter(filter: string, values: string[]): void {
    this.searchFilters = this.searchFilters || {};
    this.searchFilters[filter] = values;
    this.__dispatch__("search", this.searchTerm, this.searchFilters);
    this.__search__(this.searchTerm, this.searchFilters);
  }

  __dispatch__(e: InstanceEvent, ...args: unknown[]): void {
    this.__hooks__[e]?.forEach((hook) => {
      if (typeof hook === "function") {
        hook(...args);
      } else if (hook?.callback) {
        hook.callback(...args);
      }
    });
  }

  async __clear__(): Promise<void> {
    this.__dispatch__("results", { results: [], unfilteredTotalCount: 0 });
    if (this.__pagefind__) {
      this.availableFilters = await this.__pagefind__.filters();
      this.totalFilters = this.availableFilters;
      this.__dispatch__("filters", {
        available: this.availableFilters,
        total: this.totalFilters,
      });
    }
  }

  async __search__(term: string, filters: FilterSelection): Promise<void> {
    this.__dispatch__("loading");
    await this.__load__();
    const thisSearch = ++this.__searchID__;

    // In faceted mode, search even with empty term to show all/filtered results
    if ((!term || !term.length) && !this.faceted) {
      return this.__clear__();
    }

    if (!this.__pagefind__) return;

    const searchTerm = term && term.length ? term : null;
    const results = await this.__pagefind__.search(searchTerm, { filters });
    if (results && this.__searchID__ === thisSearch) {
      if (results.filters && Object.keys(results.filters)?.length) {
        this.availableFilters = results.filters;
        this.totalFilters = results.totalFilters ?? null;
        this.__dispatch__("filters", {
          available: this.availableFilters,
          total: this.totalFilters,
        });
      }
      this.searchResult = results;
      this.__dispatch__("results", this.searchResult);

      // Fallback: announce results if no component has claimed announcement capability
      if (!this.hasAnnouncementCapability() && term) {
        const count = results.results?.length ?? 0;
        const key =
          count === 0
            ? "zero_results"
            : count === 1
              ? "one_result"
              : "many_results";
        const priority = count === 0 ? "assertive" : "polite";
        this.announce(key, { SEARCH_TERM: term, COUNT: count }, priority);
      }
    }
  }

  async __load__(): Promise<void> {
    if (this.__pagefind__) {
      return;
    }

    if (this.__loadPromise__) {
      return this.__loadPromise__;
    }

    this.__loadPromise__ = this.__doLoad__();

    try {
      await this.__loadPromise__;
    } finally {
      this.__loadPromise__ = null;
    }
  }

  private async __doLoad__(): Promise<void> {
    if (this.__pagefind__) return;

    let imported_pagefind: PagefindAPI;
    try {
      imported_pagefind = await import(
        /* @vite-ignore */
        `${this.options.bundlePath}pagefind.js`
      );
    } catch (e) {
      console.error(e);
      console.error(
        [
          `Pagefind couldn't be loaded from ${this.options.bundlePath}pagefind.js`,
          `You can configure this by passing a bundlePath option to the Pagefind Component UI`,
        ].join("\n"),
      );
      // Important: Check that the element is indeed a <script> node, to avoid a DOM clobbering vulnerability
      if (
        document?.currentScript &&
        document.currentScript.tagName.toUpperCase() === "SCRIPT"
      ) {
        console.error(
          `[DEBUG: Loaded from ${
            (document.currentScript as HTMLScriptElement)?.src ??
            "bad script location"
          }]`,
        );
      } else {
        console.error("no known script location");
      }

      this.__dispatch__("error", {
        type: "bundle_load_failed",
        message: "Could not load search bundle",
        bundlePath: this.options.bundlePath,
        error: e,
      } as PagefindError);

      // Fallback: announce error if no component has claimed announcement capability
      if (!this.hasAnnouncementCapability()) {
        this.announce("error_search", {}, "assertive");
      }
      return;
    }

    await imported_pagefind.options(this.pagefindOptions || {});
    for (const index of this.options.mergeIndex) {
      if (!index.bundlePath) {
        throw new Error("mergeIndex requires a bundlePath parameter");
      }
      const { bundlePath: url, ...indexOpts } = index;
      await imported_pagefind.mergeIndex(url, indexOpts);
    }
    this.__pagefind__ = imported_pagefind;

    this.availableFilters = await this.__pagefind__.filters();
    this.totalFilters = this.availableFilters;
    this.__dispatch__("filters", {
      available: this.availableFilters,
      total: this.totalFilters,
    });

    // In faceted mode, trigger initial search to show all results
    if (this.faceted && this.__searchID__ === 0) {
      this.triggerSearch("");
    }
  }

  /**
   * Thin sub-results to the top N by relevance (location count).
   * Preserves original order while keeping only the most relevant entries.
   */
  thinSubResults(results: PagefindSubResult[], limit = 3): PagefindSubResult[] {
    if (results.length <= limit) return results;
    const topUrls = [...results]
      .sort((a, b) => (b.locations?.length ?? 0) - (a.locations?.length ?? 0))
      .slice(0, limit)
      .map((r) => r.url);
    return results.filter((r) => topUrls.includes(r.url));
  }

  /**
   * Get sub-results for display, excluding the root result and thinning to limit.
   */
  getDisplaySubResults(
    result: PagefindResultData,
    limit = 3,
  ): PagefindSubResult[] {
    if (!Array.isArray(result.sub_results)) return [];
    const hasRootSubResult =
      result.sub_results[0]?.url === (result.meta?.url || result.url);
    const subResults = hasRootSubResult
      ? result.sub_results.slice(1)
      : result.sub_results;
    return this.thinSubResults(subResults, limit);
  }
}
