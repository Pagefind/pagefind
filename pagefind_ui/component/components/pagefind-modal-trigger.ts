import { PagefindElement } from "./base-element";
import { Instance, PagefindComponent } from "../core/instance";
import {
  type KeyBinding,
  detectMac,
  parseKeyBinding,
  keyBindingMatches,
  getShortcutDisplay,
} from "../core/keyboard-shortcuts";

interface ModalComponent extends PagefindComponent {
  dialogEl?: HTMLDialogElement;
  open?: () => void;
}

export class PagefindModalTrigger extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["placeholder", "shortcut", "hide-shortcut", "compact"];
  }

  buttonEl: HTMLButtonElement | null = null;
  private _userPlaceholder: string | null = null;
  shortcut: string = "ctrl+k";
  hideShortcut: boolean = false;
  compact: boolean = false;
  isMac: boolean = false;
  private _keydownHandler: ((e: KeyboardEvent) => void) | null = null;
  private _keyBinding: KeyBinding | null = null;

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
    this.isMac = detectMac();
    this.readAttributes();
    this.render();
    this.setupKeyboardShortcut();
  }

  private readAttributes(): void {
    if (this.hasAttribute("placeholder")) {
      this._userPlaceholder = this.getAttribute("placeholder");
    }
    if (this.hasAttribute("shortcut")) {
      this.shortcut = this.getAttribute("shortcut") || "ctrl+k";
    }
    if (this.hasAttribute("hide-shortcut")) {
      this.hideShortcut = this.getAttribute("hide-shortcut") !== "false";
    }
    if (this.hasAttribute("compact")) {
      this.compact = this.getAttribute("compact") !== "false";
    }
    // Parse the key binding
    this._keyBinding = parseKeyBinding(this.shortcut);
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

    // Set aria-keyshortcuts with the display string
    if (this._keyBinding) {
      const display = getShortcutDisplay(this._keyBinding, this.isMac);
      this.buttonEl.setAttribute("aria-keyshortcuts", display.aria);
    }

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

    if (!this.hideShortcut && this._keyBinding) {
      const shortcutContainer = document.createElement("span");
      shortcutContainer.className = "pf-trigger-shortcut";
      shortcutContainer.setAttribute("aria-hidden", "true");

      const display = getShortcutDisplay(this._keyBinding, this.isMac);
      for (const keyText of display.keys) {
        const keyEl = document.createElement("span");
        keyEl.className = "pf-trigger-key";
        keyEl.textContent = keyText;
        shortcutContainer.appendChild(keyEl);
      }

      this.buttonEl.appendChild(shortcutContainer);
    }

    this.appendChild(this.buttonEl);

    this.buttonEl.addEventListener("click", () => {
      this.openModal();
    });
  }

  private setupKeyboardShortcut(): void {
    this._keydownHandler = (e: KeyboardEvent) => {
      if (this._keyBinding && keyBindingMatches(this._keyBinding, e)) {
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
