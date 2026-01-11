---
title: "<pagefind-filter-pane>"
nav_title: "<pagefind-filter-pane>"
nav_section: Components
weight: 57
---

A full filter panel displaying all available filters. Filters automatically populate based on your indexed content.

<div class="demo-box">
<pagefind-config instance="filter-pane-demo" preload></pagefind-config>
<div style="width: 200px;">
<pagefind-filter-pane instance="filter-pane-demo" open="section"></pagefind-filter-pane>
</div>
</div>

```html
<pagefind-filter-pane></pagefind-filter-pane>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `show-empty` | boolean | `false` | Show filter options with zero results |
| `expanded` | boolean | `false` | Always show groups expanded (non-collapsible) |
| `open` | string | `""` | Comma-separated filter names to start open |
| `sort` | string | `"default"` | Sort filter values: `default`, `alphabetical`, `count-desc`, `count-asc` |
| `auto-open-threshold` | number | `6` | Max values for auto-opening single groups (0 to disable) |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

### Auto-open Behavior

By default, filter groups are collapsible. If there is only one group, and it has options at or below the `auto-open-threshold`, it will open automatically.

Groups can also be opened by listing their keys in the `open` attribute.

### Filter Visibility

- Options with zero matching results are hidden by default (unless `show-empty` is set or the option is selected)
- If all options in a group are hidden, the entire group is hidden

## Faceted Search Mode


For catalog-style interfaces where users browse by filters without requiring a search term, enable faceted mode via [`<pagefind-config>`](/components/config/):

```html
<pagefind-config faceted preload></pagefind-config>
```

In faceted mode, all results are shown initially and update immediately when filters are changed. See the [config component documentation](/components/config/#faceted-search-mode) for details.
