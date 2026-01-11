import * as PagefindComponents from "./components/index";

declare global {
  interface Window {
    PagefindComponents: typeof PagefindComponents;
  }
}

// Expose utilities on window for programmatic access
if (typeof window !== "undefined") {
  window.PagefindComponents = PagefindComponents;
}

// Export for ES module consumers
export * from "./components/index";
export { Instance } from "./core/instance";
export type * from "./types";
