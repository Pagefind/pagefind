// Type-only entry point for TypeScript consumers
// (avoids triggering side effects from component registration)

export {
  getInstanceManager,
  configureInstance,
} from "./components/instance-manager";

// Component class types (using `export type` avoids executing the module)
export type { PagefindElement } from "./components/base-element";
export type { PagefindConfig } from "./components/pagefind-config";
export type { PagefindInput } from "./components/pagefind-input";
export type { PagefindSummary } from "./components/pagefind-summary";
export type { PagefindResults } from "./components/pagefind-results";
export type { PagefindFilterPane } from "./components/pagefind-filter-pane";
export type { PagefindFilterDropdown } from "./components/pagefind-filter-dropdown";
export type { PagefindModal } from "./components/pagefind-modal";
export type { PagefindModalTrigger } from "./components/pagefind-modal-trigger";
export type { PagefindModalHeader } from "./components/pagefind-modal-header";
export type { PagefindModalBody } from "./components/pagefind-modal-body";
export type { PagefindModalFooter } from "./components/pagefind-modal-footer";
export type { PagefindKeyboardHints } from "./components/pagefind-keyboard-hints";
export type { PagefindSearchbox } from "./components/pagefind-searchbox";

export type { Instance } from "./core/instance";

export type * from "./types";
