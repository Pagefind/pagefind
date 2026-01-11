---
title: "<pagefind-filter-dropdown>"
nav_title: "<pagefind-filter-dropdown>"
nav_section: Components
weight: 58
---

A dropdown selector for a single filter.

<div class="demo-box">
<pagefind-filter-dropdown filter="section" label="Section" instance="filter-dd-demo"></pagefind-filter-dropdown>
</div>

```html
<pagefind-filter-dropdown filter="section" label="Section"></pagefind-filter-dropdown>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `filter` | string | — | **Required.** The filter key to display |
| `label` | string | filter name | Text shown on the trigger button |
| `single-select` | boolean | `false` | Allow only one selection (closes after selecting) |
| `show-empty` | boolean | `false` | Show options with zero results |
| `wrap` | boolean | `false` | Allow text to wrap to multiple lines |
| `sort` | string | `"default"` | Sort filter values: `default`, `alphabetical`, `count-desc`, `count-asc` |
| `hide-clear` | boolean | `false` | Hide the adjacent "Clear" button |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Navigation

Full keyboard support following the ARIA Listbox pattern:

| Key | Action |
|-----|--------|
| `↓` / `↑` | Navigate through options |
| `Home` / `End` | Jump to first/last option |
| `Enter` / `Space` | Toggle selection |
| `Escape` | Close dropdown |
| `Tab` | Close and move focus |
| Type characters | Jump to matching option (type-ahead) |

### Type-Ahead Behavior

When you type characters while the dropdown is open:
- The component jumps to the first option that starts with the typed characters
- Matching is case-insensitive
- The type buffer clears after 500ms of inactivity

## Faceted Search Mode

For catalog-style interfaces where users browse by filters without requiring a search term, enable faceted mode via [`<pagefind-config>`](/components/config/):

```html
<pagefind-config faceted preload></pagefind-config>
```

In faceted mode, all results are shown initially and update immediately when filters are changed. See the [config component documentation](/components/config/#faceted-search-mode) for details.
