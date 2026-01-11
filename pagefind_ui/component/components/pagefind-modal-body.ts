import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";

export class PagefindModalBody extends PagefindElement {
  init(): void {
    this.classList.add("pf-modal-body");
    // Prevent scrollable container from being in tab order,
    // as all children should be interactable
    this.setAttribute("tabindex", "-1");
  }

  register(_instance: Instance): void {
    /* structural - unregistered */
  }
}

if (!customElements.get("pagefind-modal-body")) {
  customElements.define("pagefind-modal-body", PagefindModalBody);
}
