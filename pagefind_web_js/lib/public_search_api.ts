import { PagefindWrapper } from "./search_wrapper.js";

let pagefind: PagefindWrapper | undefined = undefined;
let initial_options: PagefindIndexOptions | undefined = undefined;

const deriveBasePath = (explicit?: string): string | undefined => {
  if (explicit) return explicit;
  if (typeof import.meta.url !== "undefined") {
    return import.meta.url.match(
      /^(.*\/)pagefind.js.*$/,
    )?.[1];
  }
};

const detectLanguage = (): string => {
  if (typeof document !== "undefined" && document?.querySelector) {
    return (
      document.querySelector("html")?.getAttribute("lang") || "unknown"
    ).toLowerCase();
  }
  return "unknown";
};

const init_pagefind = () => {
  if (!pagefind) {
    pagefind = new PagefindWrapper({
      ...initial_options,
      basePath: deriveBasePath(initial_options?.basePath),
      language: detectLanguage(),
      primary: true,
    });
  }
};

export const options = async (new_options: PagefindIndexOptions) => {
  if (pagefind) {
    await pagefind.options(new_options);
  } else {
    initial_options = new_options;
  }
};
export const init = async () => {
  init_pagefind();
};
export const destroy = async () => {
  if (pagefind) {
    await pagefind.destroy();
  }
  pagefind = undefined;
  initial_options = undefined;
};

export const mergeIndex = async (
  indexPath: string,
  options: PagefindIndexOptions,
) => {
  init_pagefind();
  return await pagefind!.mergeIndex(indexPath, options);
};
export const search = async (term: string, options: PagefindSearchOptions) => {
  init_pagefind();
  return await pagefind!.search(term, options);
};
export const debouncedSearch = async (
  term: string,
  options: PagefindSearchOptions,
  debounceTimeoutMs: number = 300,
) => {
  init_pagefind();
  return await pagefind!.debouncedSearch(term, options, debounceTimeoutMs);
};
export const preload = async (term: string, options: PagefindSearchOptions) => {
  init_pagefind();
  return await pagefind!.preload(term, options);
};
export const filters = async () => {
  init_pagefind();
  return await pagefind!.filters();
};

/**
 * Creates an independent Pagefind instance with its own configuration.
 * Use this when you need multiple search instances on the same page
 * with different options.
 * All instances share a single web worker and WASM module internally.
 */
export const createInstance = (
  instanceOptions?: PagefindIndexOptions,
): PagefindInstance => {
  const wrapper = new PagefindWrapper({
    ...instanceOptions,
    basePath: deriveBasePath(instanceOptions?.basePath),
    language: detectLanguage(),
    primary: true,
  });

  return {
    options: (opts: PagefindIndexOptions) => wrapper.options(opts),
    init: () => wrapper.waitForInit(),
    destroy: () => wrapper.destroy(),
    mergeIndex: (indexPath: string, options: PagefindIndexOptions) =>
      wrapper.mergeIndex(indexPath, options),
    search: (term: string, options: PagefindSearchOptions = {}) =>
      wrapper.search(term, options),
    debouncedSearch: (
      term: string,
      options?: PagefindSearchOptions,
      debounceTimeoutMs: number = 300,
    ) => wrapper.debouncedSearch(term, options, debounceTimeoutMs),
    preload: (term: string, options: PagefindSearchOptions = {}) =>
      wrapper.preload(term, options),
    filters: () => wrapper.filters(),
  };
};
