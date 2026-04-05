import { PagefindWrapper } from "./search_wrapper.js";

let pagefind: PagefindWrapper | undefined = undefined;
let initial_options: PagefindIndexOptions | undefined = undefined;

const init_pagefind = () => {
  if (!pagefind) {
    let basePath = initial_options?.basePath;

    // Derive basePath if not explicitly provided
    if (!basePath && typeof import.meta.url !== "undefined") {
      let derivedBasePath = import.meta.url.match(
        /^(.*\/)pagefind.js.*$/,
      )?.[1];
      if (derivedBasePath) {
        basePath = derivedBasePath;
      }
    }

    let language = "unknown";
    if (typeof document !== "undefined" && document?.querySelector) {
      const langCode =
        document.querySelector("html")?.getAttribute("lang") || "unknown";
      language = langCode.toLowerCase();
    }

    pagefind = new PagefindWrapper({
      ...initial_options,
      basePath,
      language,
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
