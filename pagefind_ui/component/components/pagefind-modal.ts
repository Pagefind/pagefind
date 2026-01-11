import { PagefindElement } from "./base-element";
import { Instance, PagefindComponent } from "../core/instance";

interface ModalTrigger extends PagefindComponent {
  buttonEl?: HTMLButtonElement;
  handleModalClose?: () => void;
}

export class PagefindModal extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["reset-on-close"];
  }

  dialogEl: HTMLDialogElement | null = null;
  resetOnClose: boolean = false;
  private _isOpen: boolean = false;
  private _closeHandler: (() => void) | null = null;

  constructor() {
    super();
  }

  init(): void {
    if (this.hasAttribute("reset-on-close")) {
      this.resetOnClose = this.getAttribute("reset-on-close") !== "false";
    }

    this.render();
  }

  render(): void {
    const hasChildren = this.children.length > 0;
    const children = hasChildren ? Array.from(this.children) : null;

    this.innerHTML = "";

    const dialogId = this.id || this.instance!.generateId("pagefind-modal");
    const searchLabel = this.instance?.translate("keyboard_search") || "search";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.dialogEl = document.createElement("dialog");
    this.dialogEl.className = "pf-modal";
    this.dialogEl.id = dialogId;
    this.dialogEl.setAttribute("aria-label", searchLabel);

    if (hasChildren && children) {
      // User provided structure - move children into dialog as-is
      children.forEach((child) => this.dialogEl!.appendChild(child));
    } else {
      // Generate default structure, inheriting instance attribute if present
      const inst = this.getAttribute("instance");

      const header = document.createElement("pagefind-modal-header");
      const input = document.createElement("pagefind-input");
      if (inst) input.setAttribute("instance", inst);
      header.appendChild(input);

      const body = document.createElement("pagefind-modal-body");
      const summary = document.createElement("pagefind-summary");
      const results = document.createElement("pagefind-results");
      if (inst) {
        summary.setAttribute("instance", inst);
        results.setAttribute("instance", inst);
      }
      body.append(summary, results);

      const footer = document.createElement("pagefind-modal-footer");
      const hints = document.createElement("pagefind-keyboard-hints");
      if (inst) hints.setAttribute("instance", inst);
      footer.appendChild(hints);

      this.dialogEl.append(header, body, footer);
    }

    this.appendChild(this.dialogEl);
    this.setupEventHandlers();
  }

  private setupEventHandlers(): void {
    if (!this.dialogEl) return;

    this._closeHandler = () => {
      this._isOpen = false;
      this.handleClose();
    };
    this.dialogEl.addEventListener("close", this._closeHandler);

    this.dialogEl.addEventListener(
      "keydown",
      (e) => {
        if (e.key === "Escape") {
          e.stopPropagation();
        }
      },
      true,
    );

    this.dialogEl.addEventListener("click", (e) => {
      if (e.target === this.dialogEl) {
        this.close();
      }
    });
  }

  open(): void {
    if (this._isOpen || !this.dialogEl) return;

    this._isOpen = true;
    this.dialogEl.showModal();

    const closeText = this.instance?.translate("keyboard_close") || "close";
    this.instance?.registerShortcut(
      { label: "esc", description: closeText },
      this,
    );

    requestAnimationFrame(() => {
      const input = this.querySelector(
        "pagefind-input",
      ) as PagefindComponent | null;
      if (input && typeof input.focus === "function") {
        input.focus();
      } else {
        const inputEl = this.querySelector("input");
        if (inputEl) {
          inputEl.focus();
        }
      }
    });

    const triggers = (this.instance?.getUtilities("modal-trigger") ||
      []) as ModalTrigger[];
    triggers.forEach((t) => t.buttonEl?.setAttribute("aria-expanded", "true"));
  }

  close(): void {
    if (!this._isOpen || !this.dialogEl) return;

    this.dialogEl.close();
  }

  private handleClose(): void {
    this.instance?.deregisterAllShortcuts(this);

    if (this.resetOnClose && this.instance) {
      this.instance.triggerSearch("");
    }

    const triggers = (this.instance?.getUtilities("modal-trigger") ||
      []) as ModalTrigger[];
    const trigger = triggers[0];
    if (trigger && typeof trigger.handleModalClose === "function") {
      trigger.handleModalClose();
    }
  }

  get isOpen(): boolean {
    return this._isOpen;
  }

  register(instance: Instance): void {
    instance.registerUtility(this, "modal");

    instance.on(
      "translations",
      () => {
        const wasOpen = this._isOpen;
        this.render();
        if (wasOpen) {
          this.open();
        }
      },
      this,
    );
  }

  reconcileAria(): void {
    const triggers = (this.instance?.getUtilities("modal-trigger") ||
      []) as ModalTrigger[];
    triggers.forEach((t) => {
      if (t.buttonEl && this.dialogEl?.id) {
        t.buttonEl.setAttribute("aria-controls", this.dialogEl.id);
      }
    });
  }

  cleanup(): void {
    if (this.dialogEl && this._closeHandler) {
      this.dialogEl.removeEventListener("close", this._closeHandler);
    }
    this.instance?.deregisterAllShortcuts(this);
  }

  update(): void {
    if (this.hasAttribute("reset-on-close")) {
      this.resetOnClose = this.getAttribute("reset-on-close") !== "false";
    }
  }
}

if (!customElements.get("pagefind-modal")) {
  customElements.define("pagefind-modal", PagefindModal);
}
