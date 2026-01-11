import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";
import type { FilterCounts, FilterSelection, PagefindError } from "../types";

type SortOption = "default" | "alphabetical" | "count-desc" | "count-asc";

interface FilterElementRefs {
  label: HTMLLabelElement;
  countSpan: HTMLSpanElement;
  checkbox: HTMLInputElement;
}

interface GroupElementRefs {
  group: HTMLElement;
  optionsContainer: HTMLElement;
  selectedCountSpan: HTMLSpanElement | null;
}

export class PagefindFilterPane extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["show-empty", "expanded", "open", "sort", "auto-open-threshold"];
  }

  containerEl: HTMLElement | null = null;
  showEmpty: boolean = false;
  expanded: boolean = false;
  openFilters: string[] = [];
  sortOption: SortOption = "default";
  autoOpenThreshold: number = 6;
  selectedFilters: Record<string, Set<string>> = {};
  availableFilters: FilterCounts | null = null;
  totalFilters: FilterCounts | null = null;

  filterElements: Map<string, FilterElementRefs> = new Map();
  groupElements: Map<string, GroupElementRefs> = new Map();
  isRendered: boolean = false;

  constructor() {
    super();
  }

  init(): void {
    if (this.hasAttribute("show-empty")) {
      this.showEmpty = this.getAttribute("show-empty") !== "false";
    }
    if (this.hasAttribute("expanded")) {
      this.expanded = this.getAttribute("expanded") !== "false";
    }
    if (this.hasAttribute("open")) {
      this.openFilters = (this.getAttribute("open") || "")
        .split(",")
        .map((s) => s.trim().toLowerCase())
        .filter((s) => s.length > 0);
    }
    if (this.hasAttribute("sort")) {
      const sortVal = this.getAttribute("sort") as SortOption;
      if (
        ["default", "alphabetical", "count-desc", "count-asc"].includes(sortVal)
      ) {
        this.sortOption = sortVal;
      }
    }
    if (this.hasAttribute("auto-open-threshold")) {
      this.autoOpenThreshold = parseInt(
        this.getAttribute("auto-open-threshold") || "6",
        10,
      );
    }

    this.render();
  }

  private sortValues(
    values: [string, number][],
    availableValues: Record<string, number>,
  ): [string, number][] {
    if (this.sortOption === "default") {
      return values;
    }

    const sorted = [...values];
    switch (this.sortOption) {
      case "alphabetical":
        sorted.sort((a, b) => a[0].localeCompare(b[0]));
        break;
      case "count-desc":
        sorted.sort((a, b) => {
          const countA = availableValues[a[0]] ?? a[1];
          const countB = availableValues[b[0]] ?? b[1];
          return countB - countA;
        });
        break;
      case "count-asc":
        sorted.sort((a, b) => {
          const countA = availableValues[a[0]] ?? a[1];
          const countB = availableValues[b[0]] ?? b[1];
          return countA - countB;
        });
        break;
    }
    return sorted;
  }

  render(): void {
    this.innerHTML = "";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.containerEl = document.createElement("div");
    this.containerEl.className = "pf-filter-pane";
    this.appendChild(this.containerEl);
  }

  getSelectedText(count: number): string {
    return String(count);
  }

  shouldGroupStartOpen(
    filterName: string,
    valueCount: number,
    filterCount: number,
  ): boolean {
    if (this.openFilters.length > 0) {
      return this.openFilters.includes(filterName.toLowerCase());
    }
    return (
      this.autoOpenThreshold > 0 &&
      filterCount === 1 &&
      valueCount <= this.autoOpenThreshold
    );
  }

  hasStructureChanged(): boolean {
    if (!this.totalFilters) return false;

    const currentGroups = new Set(Object.keys(this.totalFilters));
    const renderedGroups = new Set(this.groupElements.keys());

    if (currentGroups.size !== renderedGroups.size) return true;
    for (const group of currentGroups) {
      if (!renderedGroups.has(group)) return true;
    }

    for (const [filterName, values] of Object.entries(this.totalFilters)) {
      const currentValues = new Set(Object.keys(values));
      for (const value of currentValues) {
        if (!this.filterElements.has(`${filterName}:${value}`)) return true;
      }
    }

    return false;
  }

  handleFiltersUpdate(): void {
    if (!this.containerEl || !this.totalFilters) return;

    const filterNames = Object.keys(this.totalFilters);
    if (filterNames.length === 0) {
      this.containerEl.setAttribute("data-pf-hidden", "true");
      return;
    }

    this.containerEl.removeAttribute("data-pf-hidden");

    if (!this.isRendered || this.hasStructureChanged()) {
      this.renderFilters();
    } else {
      this.updateFilters();
    }
  }

  renderFilters(): void {
    if (!this.containerEl || !this.totalFilters) return;

    this.containerEl.innerHTML = "";
    this.filterElements.clear();
    this.groupElements.clear();

    const filterNames = Object.keys(this.totalFilters);

    for (const filterName of filterNames) {
      const values = this.totalFilters[filterName];
      const availableValues = this.availableFilters?.[filterName] || {};

      const group = this.renderFilterGroup(
        filterName,
        values,
        availableValues,
        filterNames.length,
      );
      if (group) {
        this.containerEl.appendChild(group);
      }
    }

    this.isRendered = true;
  }

  updateFilters(): void {
    for (const [key, elements] of this.filterElements) {
      const colonIndex = key.indexOf(":");
      const filterName = key.slice(0, colonIndex);
      const value = key.slice(colonIndex + 1);
      const availableCount = this.availableFilters?.[filterName]?.[value] ?? 0;
      const totalCount = this.totalFilters?.[filterName]?.[value] ?? 0;
      const isSelected = this.selectedFilters[filterName]?.has(value);

      const count = isSelected ? totalCount : availableCount;
      elements.countSpan.textContent = String(count);

      const shouldShow = this.showEmpty || availableCount > 0 || isSelected;
      elements.label.toggleAttribute("data-pf-hidden", !shouldShow);

      elements.checkbox.checked = isSelected || false;
    }

    for (const [filterName, elements] of this.groupElements) {
      const selectedCount = this.selectedFilters[filterName]?.size || 0;

      if (elements.selectedCountSpan) {
        if (selectedCount > 0) {
          elements.selectedCountSpan.textContent =
            this.getSelectedText(selectedCount);
          elements.selectedCountSpan.removeAttribute("data-pf-hidden");
        } else {
          elements.selectedCountSpan.setAttribute("data-pf-hidden", "true");
        }
      }

      let hasVisibleOptions = false;
      const options = elements.optionsContainer.querySelectorAll(
        ".pf-filter-checkbox",
      );
      for (const option of options) {
        if (!option.hasAttribute("data-pf-hidden")) {
          hasVisibleOptions = true;
          break;
        }
      }
      elements.group.toggleAttribute("data-pf-hidden", !hasVisibleOptions);
    }
  }

  renderFilterGroup(
    filterName: string,
    values: Record<string, number>,
    availableValues: Record<string, number>,
    filterCount: number,
  ): HTMLElement | null {
    const rawEntries = Object.entries(values);
    if (rawEntries.length === 0) return null;

    const valueEntries = this.sortValues(rawEntries, availableValues);
    const displayName =
      filterName.charAt(0).toUpperCase() + filterName.slice(1);
    const selectedCount = this.selectedFilters[filterName]?.size || 0;
    const shouldOpen =
      this.expanded ||
      this.shouldGroupStartOpen(filterName, valueEntries.length, filterCount);

    let group: HTMLElement;
    let optionsContainer: HTMLElement;
    let selectedCountSpan: HTMLSpanElement | null = null;

    if (this.expanded) {
      group = document.createElement("fieldset");
      group.className = "pf-filter-group";

      const legend = document.createElement("legend");
      legend.className = "pf-filter-group-title";
      const titleSpan = document.createElement("span");
      titleSpan.className = "pf-filter-group-name";
      titleSpan.textContent = displayName;
      legend.appendChild(titleSpan);
      group.appendChild(legend);

      optionsContainer = document.createElement("div");
      optionsContainer.className = "pf-filter-options";
      group.appendChild(optionsContainer);
    } else {
      group = document.createElement("details");
      group.className = "pf-filter-group";
      (group as HTMLDetailsElement).dataset.filterName = filterName;
      if (shouldOpen) {
        (group as HTMLDetailsElement).open = true;
      }

      const summary = document.createElement("summary");
      summary.className = "pf-filter-group-title";

      const titleSpan = document.createElement("span");
      titleSpan.className = "pf-filter-group-name";
      titleSpan.textContent = displayName;
      summary.appendChild(titleSpan);

      selectedCountSpan = document.createElement("span");
      selectedCountSpan.className = "pf-filter-group-count";
      selectedCountSpan.setAttribute("aria-hidden", "true");
      if (selectedCount > 0) {
        selectedCountSpan.textContent = this.getSelectedText(selectedCount);
      } else {
        selectedCountSpan.setAttribute("data-pf-hidden", "true");
      }
      summary.appendChild(selectedCountSpan);

      group.appendChild(summary);

      const fieldset = document.createElement("fieldset");
      fieldset.className = "pf-filter-fieldset";

      const legend = document.createElement("legend");
      legend.setAttribute("data-pf-sr-hidden", "");
      legend.textContent = displayName;
      fieldset.appendChild(legend);

      optionsContainer = document.createElement("div");
      optionsContainer.className = "pf-filter-options";
      fieldset.appendChild(optionsContainer);

      group.appendChild(fieldset);
    }

    this.groupElements.set(filterName, {
      group,
      optionsContainer,
      selectedCountSpan,
    });

    for (const [value, totalCount] of valueEntries) {
      const availableCount = availableValues[value] ?? 0;
      const isSelected = this.selectedFilters[filterName]?.has(value) || false;
      const count = isSelected ? totalCount : availableCount;
      const shouldShow = this.showEmpty || availableCount > 0 || isSelected;

      this.renderCheckbox(
        optionsContainer,
        filterName,
        value,
        count,
        isSelected,
        shouldShow,
      );
    }

    return group;
  }

  renderCheckbox(
    container: HTMLElement,
    filterName: string,
    value: string,
    count: number,
    isSelected: boolean,
    shouldShow: boolean,
  ): void {
    const checkboxId = this.instance!.generateId(
      `pf-filter-${filterName}-${value}`,
    );

    const label = document.createElement("label");
    label.className = "pf-filter-checkbox";
    label.setAttribute("for", checkboxId);
    if (!shouldShow) {
      label.setAttribute("data-pf-hidden", "true");
    }

    const checkbox = document.createElement("input");
    checkbox.type = "checkbox";
    checkbox.className = "pf-checkbox-input";
    checkbox.id = checkboxId;
    checkbox.name = filterName;
    checkbox.value = value;
    checkbox.checked = isSelected;
    checkbox.addEventListener("change", (e) => {
      this.handleCheckboxChange(
        filterName,
        value,
        (e.target as HTMLInputElement).checked,
      );
    });
    label.appendChild(checkbox);

    const textNode = document.createTextNode(value);
    label.appendChild(textNode);

    const countSpan = document.createElement("span");
    countSpan.className = "pf-filter-checkbox-count";
    countSpan.textContent = String(count);
    label.appendChild(countSpan);

    container.appendChild(label);

    this.filterElements.set(`${filterName}:${value}`, {
      label,
      countSpan,
      checkbox,
    });
  }

  handleCheckboxChange(
    filterName: string,
    value: string,
    checked: boolean,
  ): void {
    if (!this.selectedFilters[filterName]) {
      this.selectedFilters[filterName] = new Set();
    }

    if (checked) {
      this.selectedFilters[filterName].add(value);
    } else {
      this.selectedFilters[filterName].delete(value);
    }

    const groupElements = this.groupElements.get(filterName);
    if (groupElements?.selectedCountSpan) {
      const selectedCount = this.selectedFilters[filterName].size;
      if (selectedCount > 0) {
        groupElements.selectedCountSpan.textContent =
          this.getSelectedText(selectedCount);
        groupElements.selectedCountSpan.removeAttribute("data-pf-hidden");
      } else {
        groupElements.selectedCountSpan.setAttribute("data-pf-hidden", "true");
      }
    }

    const selectedValues = Array.from(this.selectedFilters[filterName]);

    if (selectedValues.length === 0) {
      delete this.selectedFilters[filterName];
      const filters: FilterSelection = {};
      for (const [name, values] of Object.entries(this.selectedFilters)) {
        filters[name] = Array.from(values);
      }
      this.instance?.triggerFilters(filters);
    } else {
      this.instance?.triggerFilter(filterName, selectedValues);
    }
  }

  register(instance: Instance): void {
    instance.registerFilter(this);

    instance.on(
      "filters",
      (filters: unknown) => {
        const f = filters as { available: FilterCounts; total: FilterCounts };
        this.availableFilters = f.available;
        this.totalFilters = f.total;
        this.handleFiltersUpdate();
      },
      this,
    );

    instance.on(
      "search",
      (_term: unknown, filters: unknown) => {
        this.selectedFilters = {};
        const f = filters as FilterSelection | undefined;
        if (f) {
          for (const [name, values] of Object.entries(f)) {
            if (Array.isArray(values) && values.length > 0) {
              this.selectedFilters[name] = new Set(values);
            }
          }
        }
        if (this.isRendered) {
          this.updateFilters();
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

    instance.on(
      "translations",
      () => {
        this.render();
        this.isRendered = false;
        this.handleFiltersUpdate();
      },
      this,
    );
  }

  update(): void {
    if (this.hasAttribute("show-empty")) {
      this.showEmpty = this.getAttribute("show-empty") !== "false";
    }
    if (this.hasAttribute("expanded")) {
      this.expanded = this.getAttribute("expanded") !== "false";
    }
    if (this.hasAttribute("open")) {
      this.openFilters = (this.getAttribute("open") || "")
        .split(",")
        .map((s) => s.trim().toLowerCase())
        .filter((s) => s.length > 0);
    }
    if (this.isRendered) {
      this.isRendered = false;
      this.handleFiltersUpdate();
    }
  }
}

if (!customElements.get("pagefind-filter-pane")) {
  customElements.define("pagefind-filter-pane", PagefindFilterPane);
}
