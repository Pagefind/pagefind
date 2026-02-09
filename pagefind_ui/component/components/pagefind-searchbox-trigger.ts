import { PagefindElement } from "./base-element";
import { Instance, PagefindComponent } from "../core/instance";
import {
  type KeyBinding,
  detectMac,
  parseKeyBinding,
  keyBindingMatches,
  getShortcutDisplay,
} from "../core/keyboard-shortcuts";

interface SearchboxComponent extends PagefindComponent {
  inputEl?: HTMLInputElement | null;
}

export class PagefindSearchboxTrigger extends PagefindElement {
  static get observedAttributes(): string[] {
    return ["shortcut", "hide-shortcut"];
  }

  shortcut: string = "ctrl+k";
  hideShortcut: boolean = false;
  isMac: boolean = false;
  private _keydownHandler: ((e: KeyboardEvent) => void) | null = null;
  private _keyBinding: KeyBinding | null = null;
  private _shortcutEl: HTMLElement | null = null;

  constructor() {
    super();
  }

  init(): void {
    this.isMac = detectMac();
    this.readAttributes();
    this.setupKeyboardShortcut();
    this.renderShortcutHint();
  }

  private readAttributes(): void {
    if (this.hasAttribute("shortcut")) {
      this.shortcut = this.getAttribute("shortcut") || "ctrl+k";
    }
    if (this.hasAttribute("hide-shortcut")) {
      this.hideShortcut = this.getAttribute("hide-shortcut") !== "false";
    }
    // Parse the key binding (platform-independent)
    this._keyBinding = parseKeyBinding(this.shortcut);
  }

  private setupKeyboardShortcut(): void {
    this._keydownHandler = (e: KeyboardEvent) => {
      if (this._keyBinding && keyBindingMatches(this._keyBinding, e)) {
        // Don't trigger if user is typing in any input field
        const activeEl = document.activeElement;
        const isTyping =
          activeEl?.tagName === "INPUT" ||
          activeEl?.tagName === "TEXTAREA" ||
          (activeEl as HTMLElement)?.isContentEditable;

        // Don't trigger if focus is on a link within search results
        // (results component handles its own keyboard navigation)
        const isInResults =
          activeEl?.tagName === "A" &&
          activeEl?.closest &&
          activeEl.closest(".pf-result, pagefind-results");

        if (!isTyping && !isInResults) {
          e.preventDefault();
          this.focusSearchbox();
        }
      }
    };

    document.addEventListener("keydown", this._keydownHandler);
  }

  focusSearchbox(): void {
    const searchboxes = (this.instance?.getInputs() ||
      []) as SearchboxComponent[];
    const searchbox = searchboxes[0];

    if (searchbox?.inputEl) {
      searchbox.inputEl.focus();
    }
  }

  private renderShortcutHint(): void {
    // Remove existing shortcut hint if any
    if (this._shortcutEl) {
      this._shortcutEl.remove();
      this._shortcutEl = null;
    }

    if (this.hideShortcut || !this._keyBinding) {
      return;
    }

    const searchboxes = (this.instance?.getInputs() ||
      []) as SearchboxComponent[];
    const searchbox = searchboxes[0];

    if (!searchbox?.inputEl) {
      return;
    }

    // Find the input wrapper
    const inputWrapper = searchbox.inputEl.parentElement;
    if (!inputWrapper) {
      return;
    }

    // Create shortcut hint container (shares modal-trigger styles)
    this._shortcutEl = document.createElement("span");
    this._shortcutEl.className = "pf-trigger-shortcut";
    this._shortcutEl.setAttribute("aria-hidden", "true");

    const display = getShortcutDisplay(this._keyBinding, this.isMac);
    for (const keyText of display.keys) {
      const keyEl = document.createElement("span");
      keyEl.className = "pf-trigger-key";
      keyEl.textContent = keyText;
      this._shortcutEl.appendChild(keyEl);
    }

    inputWrapper.appendChild(this._shortcutEl);

    // Set aria-keyshortcuts on the input
    searchbox.inputEl.setAttribute("aria-keyshortcuts", display.aria);
  }

  register(instance: Instance): void {
    instance.registerUtility(this, "searchbox-trigger");
  }

  cleanup(): void {
    if (this._keydownHandler) {
      document.removeEventListener("keydown", this._keydownHandler);
      this._keydownHandler = null;
    }
    if (this._shortcutEl) {
      this._shortcutEl.remove();
      this._shortcutEl = null;
    }
  }

  update(): void {
    if (this._keydownHandler) {
      document.removeEventListener("keydown", this._keydownHandler);
      this._keydownHandler = null;
    }
    this.readAttributes();
    this.setupKeyboardShortcut();
    this.renderShortcutHint();
  }
}

if (!customElements.get("pagefind-searchbox-trigger")) {
  customElements.define("pagefind-searchbox-trigger", PagefindSearchboxTrigger);
}
