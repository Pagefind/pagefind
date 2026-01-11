import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";

export class PagefindModalFooter extends PagefindElement {
  init(): void {
    this.classList.add("pf-modal-footer");
  }

  register(_instance: Instance): void {
    /* structural - unregistered */
  }
}

if (!customElements.get("pagefind-modal-footer")) {
  customElements.define("pagefind-modal-footer", PagefindModalFooter);
}
