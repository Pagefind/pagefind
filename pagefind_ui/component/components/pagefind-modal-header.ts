import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";

interface ModalElement extends HTMLElement {
  close?: () => void;
}

export class PagefindModalHeader extends PagefindElement {
  private closeBtn: HTMLButtonElement | null = null;

  init(): void {
    this.classList.add("pf-modal-header");

    const content = document.createElement("div");
    content.className = "pf-modal-header-content";
    while (this.firstChild) {
      content.appendChild(this.firstChild);
    }

    // Create close button visible on mobile only
    this.closeBtn = document.createElement("button");
    this.closeBtn.type = "button";
    this.closeBtn.className = "pf-modal-close";
    this.closeBtn.setAttribute(
      "aria-label",
      this.instance?.translate("keyboard_close") || "Close",
    );
    this.closeBtn.innerHTML = `<svg width="20" height="20" viewBox="0 0 20 20" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round"><path d="M15 5L5 15M5 5l10 10"/></svg>`;
    this.closeBtn.addEventListener("click", () => {
      const modal = this.closest("pagefind-modal") as ModalElement | null;
      if (modal && typeof modal.close === "function") {
        modal.close();
      }
    });

    this.append(content, this.closeBtn);
  }

  register(instance: Instance): void {
    instance.registerUtility(this, "modal-header");

    instance.on(
      "translations",
      () => {
        if (this.closeBtn) {
          this.closeBtn.setAttribute(
            "aria-label",
            instance.translate("keyboard_close") || "Close",
          );
        }
      },
      this,
    );
  }
}

if (!customElements.get("pagefind-modal-header")) {
  customElements.define("pagefind-modal-header", PagefindModalHeader);
}
