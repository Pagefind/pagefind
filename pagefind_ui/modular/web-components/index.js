export { getInstanceManager, configureInstance } from "./instance-manager.js";

// Import and auto-register all web components
import "./pagefind-config.js";
import "./pagefind-input.js";
import "./pagefind-summary.js";
import "./pagefind-results.js";

// Export component classes for programmatic use
export { PagefindConfig } from "./pagefind-config.js";
export { PagefindInput } from "./pagefind-input.js";
export { PagefindSummary } from "./pagefind-summary.js";
export { PagefindResults } from "./pagefind-results.js";

// Export base class for custom components
export { PagefindElement } from "./base-element.js";
