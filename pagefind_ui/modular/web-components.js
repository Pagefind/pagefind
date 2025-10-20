
import * as PagefindWC from "./web-components/index.js";

// Expose utilities on window for programmatic access
window.PagefindWebComponents = PagefindWC;

// Components are already registered via their imports
// No need to call customElements.define again
