import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import type { FilterCounts, FilterSelection, PagefindError } from "../types";

interface OptionRef {
  el: HTMLElement;
  value: string;
}

type SortOption = "default" | "alphabetical" | "count-desc" | "count-asc";

export class PagefindFilterDropdown extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["filter", "label", "single-select", "show-empty", "wrap", "sort", "hide-clear"];
  }

  isOpen: boolean = false;
  activeIndex: number = -1;
  selectedValues: Set<string> = new Set();
  isRendered: boolean = false;
  filtersLoaded: boolean = false;

  filterName: string | null = null;
  availableFilters: Record<string, number> = {};
  totalFilters: Record<string, number> = {};

  singleSelect: boolean = false;
  showEmpty: boolean = false;
  wrapLabels: boolean = false;
  hideClear: boolean = false;
  sortOption: SortOption = "default";

  wrapperEl: HTMLElement | null = null;
  triggerEl: HTMLButtonElement | null = null;
  menuEl: HTMLElement | null = null;
  optionsEl: HTMLElement | null = null;
  clearEl: HTMLButtonElement | null = null;
  badgeEl: HTMLElement | null = null;
  optionElements: OptionRef[] = [];

  typeAheadBuffer: string = "";
  typeAheadTimeout: ReturnType<typeof setTimeout> | null = null;

  constructor() {
    super();
    this._handleClickOutside = this._handleClickOutside.bind(this);
  }

  init(): void {
    this.filterName = this.getAttribute("filter");
    if (!this.filterName) {
      this.showError({
        message: "filter attribute is required on <pagefind-filter-dropdown>",
      });
      return;
    }

    this.singleSelect = this.hasAttribute("single-select");
    this.showEmpty = this.hasAttribute("show-empty");
    this.wrapLabels = this.hasAttribute("wrap");
    this.hideClear = this.hasAttribute("hide-clear");
    if (this.hasAttribute("sort")) {
      const sortVal = this.getAttribute("sort") as SortOption;
      if (
        ["default", "alphabetical", "count-desc", "count-asc"].includes(sortVal)
      ) {
        this.sortOption = sortVal;
      }
    }

    this.render();
  }

  private sortValues(values: string[]): string[] {
    if (this.sortOption === "default") {
      return values;
    }

    const sorted = [...values];
    switch (this.sortOption) {
      case "alphabetical":
        sorted.sort((a, b) => a.localeCompare(b));
        break;
      case "count-desc":
        sorted.sort((a, b) => {
          const countA = this.availableFilters[a] ?? this.totalFilters[a] ?? 0;
          const countB = this.availableFilters[b] ?? this.totalFilters[b] ?? 0;
          return countB - countA;
        });
        break;
      case "count-asc":
        sorted.sort((a, b) => {
          const countA = this.availableFilters[a] ?? this.totalFilters[a] ?? 0;
          const countB = this.availableFilters[b] ?? this.totalFilters[b] ?? 0;
          return countA - countB;
        });
        break;
    }
    return sorted;
  }

  render(): void {
    this.innerHTML = "";
    const id = this.ensureId("pf-dropdown");
    const triggerId = `${id}-trigger`;
    const menuId = `${id}-menu`;

    this.wrapperEl = document.createElement("div");
    this.wrapperEl.className = "pf-dropdown-wrapper";

    this.triggerEl = document.createElement("button");
    this.triggerEl.type = "button";
    this.triggerEl.id = triggerId;
    this.triggerEl.className = "pf-dropdown-trigger";
    if (this.wrapLabels) this.triggerEl.classList.add("wrap");
    this.triggerEl.setAttribute("role", "combobox");
    this.triggerEl.setAttribute("aria-haspopup", "listbox");
    this.triggerEl.setAttribute("aria-expanded", "false");
    this.triggerEl.setAttribute("aria-controls", menuId);

    const labelSpan = document.createElement("span");
    labelSpan.className = "pf-dropdown-trigger-label";
    if (this.wrapLabels) labelSpan.classList.add("wrap");
    labelSpan.textContent = this.getAttribute("label") || this.filterName || "";
    this.triggerEl.appendChild(labelSpan);

    this.badgeEl = document.createElement("span");
    this.badgeEl.className = "pf-dropdown-selected-badge";
    this.badgeEl.setAttribute("data-pf-hidden", "true");
    // Count badge is visual only, accessible text set in updateBadge
    this.badgeEl.setAttribute("aria-hidden", "true");
    this.badgeEl.textContent = "0";
    this.triggerEl.appendChild(this.badgeEl);

    const arrow = document.createElement("span");
    arrow.className = "pf-dropdown-arrow";
    arrow.setAttribute("aria-hidden", "true");
    this.triggerEl.appendChild(arrow);

    this.wrapperEl.appendChild(this.triggerEl);

    this.menuEl = document.createElement("div");
    this.menuEl.id = menuId;
    this.menuEl.className = "pf-dropdown-menu";
    this.menuEl.hidden = true;

    this.optionsEl = document.createElement("div");
    this.optionsEl.className = "pf-dropdown-options";
    this.optionsEl.setAttribute("role", "listbox");
    this.optionsEl.setAttribute(
      "aria-multiselectable",
      this.singleSelect ? "false" : "true",
    );
    this.optionsEl.setAttribute("aria-labelledby", triggerId);
    this.menuEl.appendChild(this.optionsEl);

    this.wrapperEl.appendChild(this.menuEl);

    // Clear button (optional)
    if (!this.hideClear) {
      this.clearEl = document.createElement("button");
      this.clearEl.type = "button";
      this.clearEl.className = "pf-dropdown-clear";
      this.clearEl.setAttribute("aria-disabled", "true");
      this.clearEl.setAttribute(
        "aria-label",
        (this.instance?.translate("clear_search") || "Clear") +
          " " +
          (this.getAttribute("label") || this.filterName || ""),
      );
      this.clearEl.textContent =
        this.instance?.translate("clear_search") || "Clear";
      this.wrapperEl.appendChild(this.clearEl);
      this.clearEl.addEventListener("click", () => this.clearAll());
    }

    this.appendChild(this.wrapperEl);

    this.triggerEl.addEventListener("click", () => this.toggle());
    this.triggerEl.addEventListener("focus", () =>
      this.instance?.triggerLoad(),
    );
    this.triggerEl.addEventListener("keydown", (e) => {
      if (this.isOpen) {
        this.handleMenuKeydown(e);
      } else {
        this.handleTriggerKeydown(e);
      }
    });

    this.isRendered = true;
  }

  toggle(): void {
    if (this.isOpen) {
      this.close();
    } else {
      this.open();
    }
  }

  open(): void {
    // Trigger Pagefind load when opening
    this.instance?.triggerLoad();

    if (this.isOpen || !this.menuEl || !this.triggerEl || !this.optionsEl)
      return;
    this.isOpen = true;

    if (!this.filtersLoaded) {
      this.showLoadingState();
    }

    this.menuEl.hidden = false;
    this.triggerEl.setAttribute("aria-expanded", "true");
    this.triggerEl.classList.add("open");

    // Always apply visual focus when opening if there are options
    if (this.optionElements.length > 0) {
      const targetIndex = this.activeIndex >= 0 ? this.activeIndex : 0;
      this.setActiveIndex(targetIndex);
    }

    const navigateText =
      this.instance?.translate("keyboard_navigate") || "navigate";
    const selectText = this.instance?.translate("keyboard_select") || "select";
    const closeText = this.instance?.translate("keyboard_close") || "close";
    this.instance?.registerShortcut(
      { label: "↑↓", description: navigateText },
      this,
    );
    this.instance?.registerShortcut(
      { label: "↵", description: selectText },
      this,
    );
    this.instance?.registerShortcut(
      { label: "esc", description: closeText },
      this,
    );

    setTimeout(() => {
      document.addEventListener("click", this._handleClickOutside);
    }, 0);
  }

  close(returnFocus = true): void {
    if (!this.isOpen || !this.menuEl || !this.triggerEl || !this.optionsEl)
      return;
    this.isOpen = false;

    this.menuEl.hidden = true;
    this.triggerEl.setAttribute("aria-expanded", "false");
    this.triggerEl.classList.remove("open");

    this.triggerEl?.removeAttribute("aria-activedescendant");

    for (const { el } of this.optionElements) {
      el.classList.remove("pf-dropdown-option-focused");
    }

    this.instance?.deregisterAllShortcuts(this);

    document.removeEventListener("click", this._handleClickOutside);

    if (returnFocus) {
      this.triggerEl.focus();
    }
  }

  private _handleClickOutside(event: MouseEvent): void {
    if (this.wrapperEl && !this.wrapperEl.contains(event.target as Node)) {
      this.close(false);
    }
  }

  handleTriggerKeydown(e: KeyboardEvent): void {
    switch (e.key) {
      case "Enter":
      case " ":
        e.preventDefault();
        this.open();
        break;
      case "ArrowDown":
        e.preventDefault();
        this.open();
        this.setActiveIndex(0);
        break;
      case "ArrowUp":
        e.preventDefault();
        this.open();
        this.setActiveIndex(this.optionElements.length - 1);
        break;
    }
  }

  handleMenuKeydown(e: KeyboardEvent): void {
    switch (e.key) {
      case "ArrowDown":
        e.preventDefault();
        this.moveActiveIndex(1);
        break;
      case "ArrowUp":
        e.preventDefault();
        this.moveActiveIndex(-1);
        break;
      case "Home":
        e.preventDefault();
        this.setActiveIndex(0);
        break;
      case "End":
        e.preventDefault();
        this.setActiveIndex(this.optionElements.length - 1);
        break;
      case "Enter":
      case " ":
        e.preventDefault();
        if (
          this.activeIndex >= 0 &&
          this.activeIndex < this.optionElements.length
        ) {
          const activeOption = this.optionElements[this.activeIndex];
          if (activeOption) {
            this.toggleOption(activeOption.value);
          }
        }
        break;
      case "Escape":
        e.preventDefault();
        this.close();
        break;
      case "Tab":
        this.close(false);
        break;
      default:
        if (e.key.length === 1 && !e.ctrlKey && !e.metaKey && !e.altKey) {
          this.handleTypeAhead(e.key);
        }
    }
  }

  setActiveIndex(index: number): void {
    if (index < 0 || index >= this.optionElements.length || !this.optionsEl)
      return;

    for (const { el } of this.optionElements) {
      el.classList.remove("pf-dropdown-option-focused");
    }

    this.activeIndex = index;
    const option = this.optionElements[index];

    option.el.classList.add("pf-dropdown-option-focused");
    this.triggerEl?.setAttribute("aria-activedescendant", option.el.id);

    this.scrollToCenter(option.el);
  }

  private scrollToCenter(el: HTMLElement): void {
    if (!this.optionsEl) return;
    const container = this.optionsEl;
    const elTop = el.offsetTop;
    const elHeight = el.offsetHeight;
    const containerHeight = container.clientHeight;
    const targetScroll = elTop - containerHeight / 2 + elHeight / 2;
    container.scrollTo({ top: targetScroll, behavior: "smooth" });
  }

  moveActiveIndex(delta: number): void {
    let newIndex = this.activeIndex + delta;

    if (newIndex < 0) {
      newIndex = this.optionElements.length - 1;
    } else if (newIndex >= this.optionElements.length) {
      newIndex = 0;
    }

    this.setActiveIndex(newIndex);
  }

  handleTypeAhead(char: string): void {
    this.typeAheadBuffer += char.toLowerCase();

    if (this.typeAheadTimeout) {
      clearTimeout(this.typeAheadTimeout);
    }

    const matchIndex = this.optionElements.findIndex(({ value }) =>
      value.toLowerCase().startsWith(this.typeAheadBuffer),
    );

    if (matchIndex >= 0) {
      this.setActiveIndex(matchIndex);
    }

    this.typeAheadTimeout = setTimeout(() => {
      this.typeAheadBuffer = "";
    }, 500);
  }

  showLoadingState(): void {
    if (!this.optionsEl) return;
    this.optionsEl.innerHTML = "";
    this.optionsEl.setAttribute("aria-busy", "true");

    const srStatus = document.createElement("div");
    srStatus.setAttribute("data-pf-sr-hidden", "true");
    srStatus.textContent = "Loading filter options...";
    this.optionsEl.appendChild(srStatus);

    for (let i = 0; i < 3; i++) {
      const skeleton = document.createElement("div");
      skeleton.className = "pf-dropdown-option pf-dropdown-option-loading";
      skeleton.setAttribute("aria-hidden", "true");

      const checkbox = document.createElement("span");
      checkbox.className = "pf-dropdown-checkbox pf-skeleton";
      skeleton.appendChild(checkbox);

      const label = document.createElement("span");
      label.className = "pf-dropdown-option-label pf-skeleton";
      label.style.width = `${60 + i * 15}%`;
      label.innerHTML = "&nbsp;";
      skeleton.appendChild(label);

      this.optionsEl.appendChild(skeleton);
    }
  }

  updateOptions(): void {
    if (!this.optionsEl) return;

    this.filtersLoaded = true;
    this.optionsEl.removeAttribute("aria-busy");
    const rawValues = Object.keys(this.totalFilters || {});
    const values = this.sortValues(rawValues);

    if (rawValues.length === 0) {
      this.optionsEl.innerHTML = "";
      const error = document.createElement("div");
      error.className = "pf-dropdown-error";
      error.setAttribute("role", "alert");
      error.textContent = `No filter "${this.filterName}" found`;
      this.optionsEl.appendChild(error);
      this.optionElements = [];
      return;
    }

    this.wrapperEl?.removeAttribute("data-pf-hidden");
    this.optionsEl.innerHTML = "";
    this.optionElements = [];

    const baseId = this.id || this.ensureId("pf-dropdown");

    values.forEach((value, index) => {
      const availableCount = this.availableFilters?.[value] ?? 0;
      const totalCount = this.totalFilters[value] ?? 0;
      const isSelected = this.selectedValues.has(value);
      const shouldShow = this.showEmpty || availableCount > 0 || isSelected;

      if (!shouldShow) return;

      const count = isSelected ? totalCount : availableCount;
      const optionId = `${baseId}-option-${index}`;

      const option = this.createOption(optionId, value, count, isSelected);
      this.optionsEl!.appendChild(option);
      this.optionElements.push({ el: option, value });
    });

    // Restore focus state after rebuilding options
    if (this.isOpen && this.optionElements.length > 0) {
      if (this.activeIndex >= this.optionElements.length) {
        this.setActiveIndex(this.optionElements.length - 1);
      } else if (this.activeIndex < 0) {
        this.setActiveIndex(0);
      } else {
        // Re-apply focus to new DOM element at same index
        this.setActiveIndex(this.activeIndex);
      }
    }

    this.updateBadge();
  }

  createOption(
    id: string,
    value: string,
    count: number,
    isSelected: boolean,
  ): HTMLElement {
    const option = document.createElement("div");
    option.id = id;
    option.className = "pf-dropdown-option";
    if (this.wrapLabels) option.classList.add("wrap");
    option.setAttribute("role", "option");
    option.setAttribute("aria-selected", String(isSelected));
    option.dataset.value = value;

    const checkbox = document.createElement("span");
    checkbox.className = "pf-dropdown-checkbox";
    checkbox.setAttribute("aria-hidden", "true");
    option.appendChild(checkbox);

    const label = document.createElement("span");
    label.className = "pf-dropdown-option-label";
    if (this.wrapLabels) label.classList.add("wrap");
    label.textContent = value;
    option.appendChild(label);

    // Count is visual only, accessible version in aria-label
    const countSpan = document.createElement("span");
    countSpan.className = "pf-dropdown-option-count";
    countSpan.setAttribute("aria-hidden", "true");
    countSpan.textContent = String(count);
    option.appendChild(countSpan);

    const resultWord = count === 1 ? "result" : "results";
    option.setAttribute("aria-label", `${value}, ${count} ${resultWord}`);

    option.addEventListener("click", (e) => {
      e.stopPropagation();
      this.toggleOption(value);
    });

    return option;
  }

  toggleOption(value: string): void {
    const wasSelected = this.selectedValues.has(value);

    if (this.singleSelect) {
      if (this.selectedValues.has(value)) {
        this.selectedValues.clear();
      } else {
        this.selectedValues.clear();
        this.selectedValues.add(value);
      }
      this.close();
    } else {
      if (this.selectedValues.has(value)) {
        this.selectedValues.delete(value);
      } else {
        this.selectedValues.add(value);
      }
    }

    // Announce selection change for screen readers
    const isNowSelected = this.selectedValues.has(value);
    if (isNowSelected !== wasSelected) {
      const action = isNowSelected ? "selected" : "deselected";
      this.instance?.announceRaw(`${value} ${action}`);
    }

    this.updateOptionStates();
    this.updateBadge();

    this.dispatchFilterChange();
  }

  clearAll(): void {
    if (this.selectedValues.size === 0) return;

    this.selectedValues.clear();
    this.updateOptionStates();
    this.updateBadge();
    this.dispatchFilterChange();
  }

  dispatchFilterChange(): void {
    if (!this.filterName) return;
    const selectedArray = Array.from(this.selectedValues);

    if (selectedArray.length === 0) {
      this.instance?.triggerFilter(this.filterName, []);
    } else {
      this.instance?.triggerFilter(this.filterName, selectedArray);
    }
  }

  updateBadge(): void {
    if (!this.badgeEl || !this.triggerEl) return;
    const count = this.selectedValues.size;
    if (count > 0) {
      this.badgeEl.textContent = String(count);
      this.badgeEl.removeAttribute("data-pf-hidden");

      const label = this.getAttribute("label") || this.filterName || "";
      const filterWord = count === 1 ? "filter" : "filters";
      this.triggerEl.setAttribute(
        "aria-label",
        `${label}, ${count} ${filterWord} selected`,
      );

      if (this.clearEl) {
        this.clearEl.removeAttribute("aria-disabled");
      }
    } else {
      this.badgeEl.setAttribute("data-pf-hidden", "true");
      this.triggerEl.removeAttribute("aria-label");

      if (this.clearEl) {
        this.clearEl.setAttribute("aria-disabled", "true");
      }
    }
  }

  updateOptionStates(): void {
    for (const { el, value } of this.optionElements) {
      const isSelected = this.selectedValues.has(value);
      el.setAttribute("aria-selected", String(isSelected));
    }
  }

  register(instance: Instance): void {
    if (!this.filterName) return;

    instance.registerFilter(this);

    instance.on(
      "filters",
      (filters: unknown) => {
        const f = filters as { available: FilterCounts; total: FilterCounts };
        this.availableFilters = f.available?.[this.filterName!] || {};
        this.totalFilters = f.total?.[this.filterName!] || {};
        if (this.isRendered) {
          this.updateOptions();
        }
      },
      this,
    );

    instance.on(
      "search",
      (_term: unknown, filters: unknown) => {
        const f = filters as FilterSelection | undefined;
        const externalValues = f?.[this.filterName!] || [];
        this.selectedValues = new Set(externalValues);
        if (this.isRendered) {
          this.updateOptionStates();
          this.updateBadge();
        }
      },
      this,
    );

    instance.on(
      "error",
      (error: unknown) => {
        const err = error as PagefindError;
        this.showError({
          message: err.message || "Failed to load filters",
          details: err.bundlePath
            ? `Bundle path: ${err.bundlePath}`
            : undefined,
        });
      },
      this,
    );
  }

  update(): void {
    const newFilterName = this.getAttribute("filter");
    if (newFilterName !== this.filterName) {
      this.filterName = newFilterName;
      this.selectedValues.clear();
      this.updateOptions();
    }

    this.singleSelect = this.hasAttribute("single-select");
    this.showEmpty = this.hasAttribute("show-empty");
    this.wrapLabels = this.hasAttribute("wrap");
    this.hideClear = this.hasAttribute("hide-clear");
    if (this.hasAttribute("sort")) {
      const sortVal = this.getAttribute("sort") as SortOption;
      if (
        ["default", "alphabetical", "count-desc", "count-asc"].includes(sortVal)
      ) {
        this.sortOption = sortVal;
      }
    } else {
      this.sortOption = "default";
    }

    if (this.optionsEl) {
      this.optionsEl.setAttribute(
        "aria-multiselectable",
        this.singleSelect ? "false" : "true",
      );
    }

    const labelSpan = this.triggerEl?.querySelector(
      ".pf-dropdown-trigger-label",
    );
    if (labelSpan) {
      labelSpan.textContent =
        this.getAttribute("label") || this.filterName || "";
    }

    this.updateOptions();
  }

  cleanup(): void {
    document.removeEventListener("click", this._handleClickOutside);
    this.instance?.deregisterAllShortcuts(this);
    if (this.typeAheadTimeout) {
      clearTimeout(this.typeAheadTimeout);
    }
  }
}

if (!customElements.get("pagefind-filter-dropdown")) {
  customElements.define("pagefind-filter-dropdown", PagefindFilterDropdown);
}
