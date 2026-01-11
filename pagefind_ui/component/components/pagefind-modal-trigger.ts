import { PagefindElement } from "./base-element";
import { Instance, PagefindComponent } from "../core/instance";

interface ModalComponent extends PagefindComponent {
  dialogEl?: HTMLDialogElement;
  open?: () => void;
}

interface NavigatorUAData {
  platform?: string;
}

export class PagefindModalTrigger extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["placeholder", "shortcut", "hide-shortcut", "compact"];
  }

  buttonEl: HTMLButtonElement | null = null;
  private _userPlaceholder: string | null = null;
  shortcut: string = "k";
  hideShortcut: boolean = false;
  compact: boolean = false;
  isMac: boolean = false;
  private _keydownHandler: ((e: KeyboardEvent) => void) | null = null;

  constructor() {
    super();
  }

  get placeholder(): string {
    return (
      this._userPlaceholder ||
      this.instance?.translate("keyboard_search") ||
      "Search"
    );
  }

  init(): void {
    this.isMac = this.detectMac();
    this.readAttributes();
    this.render();
    this.setupKeyboardShortcut();
  }

  private detectMac(): boolean {
    try {
      const uaData = (
        navigator as Navigator & { userAgentData?: NavigatorUAData }
      ).userAgentData;
      if (uaData?.platform) {
        return uaData.platform.toLowerCase().includes("mac");
      }
    } catch (e) {}
    return /mac/i.test(navigator.userAgent);
  }

  private readAttributes(): void {
    if (this.hasAttribute("placeholder")) {
      this._userPlaceholder = this.getAttribute("placeholder");
    }
    if (this.hasAttribute("shortcut")) {
      this.shortcut = (this.getAttribute("shortcut") || "k").toLowerCase();
    }
    if (this.hasAttribute("hide-shortcut")) {
      this.hideShortcut = this.getAttribute("hide-shortcut") !== "false";
    }
    if (this.hasAttribute("compact")) {
      this.compact = this.getAttribute("compact") !== "false";
    }
  }

  render(): void {
    this.innerHTML = "";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    this.buttonEl = document.createElement("button");
    this.buttonEl.className = "pf-trigger-btn";
    this.buttonEl.type = "button";
    this.buttonEl.setAttribute("aria-haspopup", "dialog");
    this.buttonEl.setAttribute("aria-expanded", "false");
    this.buttonEl.setAttribute("aria-label", this.placeholder || "Search");
    this.buttonEl.setAttribute(
      "aria-keyshortcuts",
      this.isMac
        ? `Meta+${this.shortcut.toUpperCase()}`
        : `Control+${this.shortcut.toUpperCase()}`,
    );

    const icon = document.createElement("span");
    icon.className = "pf-trigger-icon";
    icon.setAttribute("aria-hidden", "true");
    this.buttonEl.appendChild(icon);

    if (!this.compact) {
      const text = document.createElement("span");
      text.className = "pf-trigger-text";
      text.textContent = this.placeholder;
      this.buttonEl.appendChild(text);
    }

    if (!this.hideShortcut) {
      const shortcutContainer = document.createElement("span");
      shortcutContainer.className = "pf-trigger-shortcut";
      shortcutContainer.setAttribute("aria-hidden", "true");

      const modKey = document.createElement("span");
      modKey.className = "pf-trigger-key";
      modKey.textContent = this.isMac ? "\u2318" : "Ctrl";
      shortcutContainer.appendChild(modKey);

      const shortcutKey = document.createElement("span");
      shortcutKey.className = "pf-trigger-key";
      shortcutKey.textContent = this.shortcut.toUpperCase();
      shortcutContainer.appendChild(shortcutKey);

      this.buttonEl.appendChild(shortcutContainer);
    }

    this.appendChild(this.buttonEl);

    this.buttonEl.addEventListener("click", () => {
      this.openModal();
    });
  }

  private setupKeyboardShortcut(): void {
    this._keydownHandler = (e: KeyboardEvent) => {
      const modifierPressed = this.isMac ? e.metaKey : e.ctrlKey;
      const keyPressed = e.key.toLowerCase() === this.shortcut;

      if (modifierPressed && keyPressed) {
        e.preventDefault();
        this.openModal();
      }
    };

    document.addEventListener("keydown", this._keydownHandler);
  }

  openModal(): void {
    const modals = (this.instance?.getUtilities("modal") ||
      []) as ModalComponent[];
    const modal = modals[0];

    if (modal && typeof modal.open === "function") {
      modal.open();
      if (this.buttonEl) {
        this.buttonEl.setAttribute("aria-expanded", "true");
      }
    }
  }

  handleModalClose(): void {
    if (this.buttonEl) {
      this.buttonEl.setAttribute("aria-expanded", "false");
      this.buttonEl.focus();
    }
  }

  register(instance: Instance): void {
    instance.registerUtility(this, "modal-trigger");

    instance.on(
      "translations",
      () => {
        this.render();
      },
      this,
    );
  }

  reconcileAria(): void {
    const modals = (this.instance?.getUtilities("modal") ||
      []) as ModalComponent[];
    const modal = modals[0];
    if (modal?.dialogEl?.id && this.buttonEl) {
      this.buttonEl.setAttribute("aria-controls", modal.dialogEl.id);
    }
  }

  cleanup(): void {
    if (this._keydownHandler) {
      document.removeEventListener("keydown", this._keydownHandler);
      this._keydownHandler = null;
    }
  }

  update(): void {
    this.readAttributes();
    this.render();
  }
}

if (!customElements.get("pagefind-modal-trigger")) {
  customElements.define("pagefind-modal-trigger", PagefindModalTrigger);
}
