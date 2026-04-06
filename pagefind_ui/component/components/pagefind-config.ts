import { PagefindElement } from "./base-element";
import { Instance } from "../core/instance";

export class PagefindConfig extends PagefindElement {
  init(): void {
    this.setAttribute("hidden", "");
  }

  register(instance: Instance): void {
    instance.registerUtility(this);

    const bundlePath = this.getAttribute("bundle-path");
    if (bundlePath) {
      instance.options.bundlePath = bundlePath;
    }

    const baseUrl = this.getAttribute("base-url");
    if (baseUrl) {
      instance.pagefindOptions.baseUrl = baseUrl;
    }

    const excerptLength = this.getAttribute("excerpt-length");
    if (excerptLength) {
      instance.pagefindOptions.excerptLength = parseInt(excerptLength, 10);
    }

    const lang = this.getAttribute("lang");
    if (lang) {
      instance.setLanguage(lang);
    }

    const metaCacheTag = this.getAttribute("meta-cache-tag");
    if (metaCacheTag) {
      instance.pagefindOptions.metaCacheTag = metaCacheTag;
    }

    if (this.hasAttribute("no-worker")) {
      instance.pagefindOptions.noWorker = true;
    }

    if (this.hasAttribute("faceted")) {
      instance.faceted = true;
    }

    if (this.hasAttribute("preload")) {
      instance.triggerLoad();
    }
  }
}

if (!customElements.get("pagefind-config")) {
  customElements.define("pagefind-config", PagefindConfig);
}
