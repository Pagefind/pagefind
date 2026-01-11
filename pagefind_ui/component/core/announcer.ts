export type AnnouncerPriority = "polite" | "assertive";

interface AnnouncerRegions {
  polite: [HTMLElement, HTMLElement];
  assertive: [HTMLElement, HTMLElement];
}

// Delay before injecting content (Safari/VoiceOver timing)
const ANNOUNCE_DELAY_MS = 100;
// Delay before clearing content (enables repeat announcements)
const CLEAR_DELAY_MS = 350;

/**
 * Global ARIA live region announcer.
 *
 * Creates two pre-rendered live regions (polite + assertive) at document root,
 * double-buffered for reliable successive announcements.
 */
export class Announcer {
  private regions: AnnouncerRegions | null = null;
  private politeIndex: 0 | 1 = 0;
  private assertiveIndex: 0 | 1 = 0;
  private clearTimeoutId: ReturnType<typeof setTimeout> | null = null;
  private containerId: string;
  private idGenerator: (prefix: string) => string;

  constructor(idGenerator: (prefix: string) => string) {
    this.idGenerator = idGenerator;
    this.containerId = idGenerator("pf-announcer");
    this.createRegions();
  }

  private createRegions(): void {
    if (typeof document === "undefined") return;

    const container = document.createElement("div");
    container.id = this.containerId;
    container.setAttribute("data-pagefind-announcer", "");

    const createRegionPair = (
      priority: AnnouncerPriority,
    ): [HTMLElement, HTMLElement] => {
      const regions: HTMLElement[] = [];
      for (let i = 0; i < 2; i++) {
        const region = document.createElement("div");
        region.id = this.idGenerator(`pf-${priority}-region`);
        region.setAttribute("role", "status");
        region.setAttribute("aria-live", priority);
        region.setAttribute("aria-atomic", "true");
        region.setAttribute("data-pf-sr-hidden", "");
        container.appendChild(region);
        regions.push(region);
      }
      return regions as [HTMLElement, HTMLElement];
    };

    this.regions = {
      polite: createRegionPair("polite"),
      assertive: createRegionPair("assertive"),
    };

    document.body.appendChild(container);
  }

  /**
   * Announce a message to screen readers.
   */
  announce(message: string, priority: AnnouncerPriority = "polite"): void {
    if (!this.regions || !message) return;

    if (this.clearTimeoutId) {
      clearTimeout(this.clearTimeoutId);
      this.clearTimeoutId = null;
    }

    const currentIndex =
      priority === "polite" ? this.politeIndex : this.assertiveIndex;
    const region = this.regions[priority][currentIndex];

    if (priority === "polite") {
      this.politeIndex = currentIndex === 0 ? 1 : 0;
    } else {
      this.assertiveIndex = currentIndex === 0 ? 1 : 0;
    }

    const nextIndex =
      priority === "polite" ? this.politeIndex : this.assertiveIndex;
    this.regions[priority][nextIndex].textContent = "";

    setTimeout(() => {
      region.textContent = message;

      // Schedule clearing the content to enable repeat announcements of same message
      this.clearTimeoutId = setTimeout(() => {
        region.textContent = "";
        this.clearTimeoutId = null;
      }, CLEAR_DELAY_MS);
    }, ANNOUNCE_DELAY_MS);
  }

  /**
   * Clear all live regions immediately.
   */
  clear(): void {
    if (!this.regions) return;

    if (this.clearTimeoutId) {
      clearTimeout(this.clearTimeoutId);
      this.clearTimeoutId = null;
    }

    for (const priority of ["polite", "assertive"] as const) {
      for (const region of this.regions[priority]) {
        region.textContent = "";
      }
    }
  }

  /**
   * Remove announcer from DOM.
   */
  destroy(): void {
    this.clear();

    if (typeof document !== "undefined") {
      const container = document.getElementById(this.containerId);
      if (container) {
        container.remove();
      }
    }

    this.regions = null;
  }
}
