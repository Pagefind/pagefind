---
title: "CSS Variables"
nav_title: "CSS Variables"
nav_section: Customization
weight: 91
---

The Component UI uses CSS custom properties for theming. Override these variables to match your site's design.

## Core Variables

```css
:root {
  /* Colors */
  --pf-text: #1a1a1a;
  --pf-text-secondary: #666;
  --pf-text-muted: #767676;
  --pf-background: #fff;
  --pf-border: #e0e0e0;
  --pf-border-focus: #999;
  --pf-skeleton: #eee;
  --pf-skeleton-shine: #f5f5f5;
  --pf-hover: #f5f5f5;
  --pf-mark: #1a1a1a;
  --pf-scroll-shadow: rgba(0, 0, 0, 0.08);

  /* Shadows */
  --pf-shadow-sm: 0 2px 8px rgba(0, 0, 0, 0.06);
  --pf-shadow-md: 0 4px 12px rgba(0, 0, 0, 0.1);
  --pf-shadow-lg: 0 16px 48px rgba(0, 0, 0, 0.2);

  /* Error states */
  --pf-error-bg: #fef2f2;
  --pf-error-border: #fecaca;
  --pf-error-text: #dc2626;
  --pf-error-text-secondary: #b91c1c;

  /* Focus ring */
  --pf-outline-focus: #0969da;
  --pf-outline-width: 2px;
  --pf-outline-offset: 2px;

  /* Typography */
  --pf-font: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;

  /* Sizing */
  --pf-input-height: 36px;
  --pf-input-font-size: 16px; /* Keep at 16px+ to prevent iOS auto-zoom on focus */
  --pf-summary-font-size: 12px;
  --pf-result-title-font-size: 14px;
  --pf-result-excerpt-font-size: 13px;

  /* Results layout */
  --pf-results-display: flex;
  --pf-results-flex-direction: column;
  --pf-results-flex-wrap: nowrap;
  --pf-results-columns: none; /* grid-template-columns value, used when display is grid */
  --pf-results-gap: 8px;

  --pf-border-radius: 6px;
  --pf-image-width: 64px;
  --pf-image-height: 48px;

  /* Icons (SVG data URIs) */
  --pf-icon-search: url("data:image/svg+xml,...");
  --pf-icon-arrow: url("data:image/svg+xml,...");

  /* Modal */
  --pf-modal-backdrop: rgba(0, 0, 0, 0.5);
  --pf-modal-max-width: 560px;
  --pf-modal-max-height: min(80dvh, 800px);
  --pf-modal-top: 10dvh;

  /* Searchbox dimensions */
  --pf-searchbox-max-width: 480px;
  --pf-searchbox-dropdown-max-height: 320px;

  /* Dropdown settings */
  --pf-dropdown-max-height: 280px;
  --pf-dropdown-z-index: 9999;
}
```

## Dark Mode

The component UI supports dark mode via the `data-pf-theme="dark"` attribute on any ancestor element:

```html
<div data-pf-theme="dark">
  <pagefind-searchbox></pagefind-searchbox>
</div>
```

Or override variables directly:

```css
@media (prefers-color-scheme: dark) {
  :root {
    --pf-text: #e5e5e5;
    --pf-text-secondary: #a0a0a0;
    --pf-text-muted: #949494;
    --pf-background: #1a1a1a;
    --pf-border: #333;
    --pf-border-focus: #555;
    --pf-skeleton: #2a2a2a;
    --pf-skeleton-shine: #333;
    --pf-hover: #252525;
    --pf-mark: #e5e5e5;
    --pf-scroll-shadow: rgba(255, 255, 255, 0.1);
    --pf-outline-focus: #58a6ff;

    --pf-shadow-sm: 0 2px 8px rgba(0, 0, 0, 0.3);
    --pf-shadow-md: 0 4px 12px rgba(0, 0, 0, 0.4);
    --pf-shadow-lg: 0 16px 48px rgba(0, 0, 0, 0.5);

    --pf-error-bg: #2a1a1a;
    --pf-error-border: #5c2828;
    --pf-error-text: #f87171;
    --pf-error-text-secondary: #ef4444;
  }
}
```

The Component UI does not honor `prefers-color-scheme` automatically because it cannot determine if the site it is placed on does so.
