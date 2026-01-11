export type AnnouncerPriority = "polite" | "assertive";

// Pagefind search results (from WASM)
export interface PagefindSearchResult {
  results: PagefindRawResult[];
  filters?: FilterCounts;
  totalFilters?: FilterCounts;
  unfilteredTotalCount?: number;
}

export interface PagefindRawResult {
  id: string;
  data: () => Promise<PagefindResultData>;
}

export interface PagefindResultData {
  url: string;
  excerpt: string;
  meta: Record<string, string>;
  sub_results?: PagefindSubResult[];
  locations?: number[];
  weighted_locations?: WeightedLocation[];
  filters?: Record<string, string[]>;
  anchors?: PagefindAnchor[];
  content?: string;
  word_count?: number;
}

export interface PagefindSubResult {
  url: string;
  title: string;
  excerpt: string;
  locations?: number[];
  weighted_locations?: WeightedLocation[];
  anchors?: PagefindAnchor[];
}

export interface WeightedLocation {
  weight: number;
  balanced_score: number;
  location: number;
}

export interface PagefindAnchor {
  element: string;
  id: string;
  text?: string;
  location?: number;
}

export type FilterCounts = Record<string, Record<string, number>>;
export type FilterSelection = Record<string, string[]>;

export interface InstanceOptions {
  bundlePath?: string;
  mergeIndex?: MergeIndexConfig[];
  [key: string]: unknown;
}

export interface MergeIndexConfig {
  bundlePath: string;
  [key: string]: unknown;
}

export type InstanceEventMap = {
  search: [term: string, filters: FilterSelection];
  filters: [data: { available: FilterCounts; total: FilterCounts }];
  loading: [];
  results: [results: PagefindSearchResult];
  error: [error: PagefindError];
  translations: [translations: TranslationStrings, direction: TextDirection];
};

export type InstanceEvent = keyof InstanceEventMap;

export interface PagefindError {
  type?: string;
  message: string;
  bundlePath?: string;
  error?: Error;
}

export type TextDirection = "ltr" | "rtl";

export interface TranslationStrings {
  language: string;
  direction: TextDirection;
  [key: string]: string | TextDirection;
}

export interface TranslationFile {
  thanks_to?: string;
  comments?: string;
  direction?: TextDirection;
  strings: Record<string, string>;
}

export interface ComponentCapabilities {
  keyboardNavigation?: boolean;
  announcements?: boolean;
  [key: string]: unknown;
}

export interface Shortcut {
  label: string;
  description: string;
  owner?: Element;
}

export type HookCallback = (...args: unknown[]) => void;

export interface HookEntry {
  callback: HookCallback;
  owner?: Element;
}

export interface PagefindAPI {
  options: (opts: Record<string, unknown>) => Promise<void>;
  search: (
    term: string | null,
    options?: { filters?: FilterSelection },
  ) => Promise<PagefindSearchResult>;
  filters: () => Promise<FilterCounts>;
  mergeIndex: (url: string, options?: Record<string, unknown>) => Promise<void>;
  destroy?: () => void;
}
