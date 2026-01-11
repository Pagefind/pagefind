import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";

export class PagefindKeyboardHints extends PagefindElement {
  init(): void {
    this.classList.add("pf-keyboard-hints");
    // Keyboard hints are visual aids for sighted users, not meaningful for screen readers
    this.setAttribute("aria-hidden", "true");
  }

  render(): void {
    this.innerHTML = "";

    if (this.instance?.direction === "rtl") {
      this.setAttribute("dir", "rtl");
    } else {
      this.removeAttribute("dir");
    }

    const shortcuts = this.instance?.getActiveShortcuts() || [];

    if (shortcuts.length === 0) {
      return;
    }

    // Deduplicate by label
    // for example, only show ↑↓ once even if multiple components have it
    const seen = new Set<string>();
    for (const shortcut of shortcuts) {
      if (seen.has(shortcut.label)) continue;
      seen.add(shortcut.label);

      const hint = document.createElement("div");
      hint.className = "pf-keyboard-hint";

      const key = document.createElement("kbd");
      key.className = "pf-keyboard-key";
      key.textContent = shortcut.label;
      hint.appendChild(key);

      hint.appendChild(document.createTextNode(` ${shortcut.description}`));
      this.appendChild(hint);
    }
  }

  register(instance: Instance): void {
    instance.registerUtility(this, "keyboard-hints");
    this.render();

    instance.on(
      "translations",
      () => {
        this.render();
      },
      this,
    );
  }
}

if (!customElements.get("pagefind-keyboard-hints")) {
  customElements.define("pagefind-keyboard-hints", PagefindKeyboardHints);
}
