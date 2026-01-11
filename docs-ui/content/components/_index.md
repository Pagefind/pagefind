---
title: "Component System"
nav_title: "Component System"
weight: 2
---

The Pagefind Component UI is a set of web components that communicate through shared instances. This allows you to compose search interfaces from individual pieces, or use the prebuilt components that bundle everything together.

## How Components Connect

Components on the same page automatically connect to each other.

```html
<pagefind-input></pagefind-input>
<pagefind-summary></pagefind-summary>
<pagefind-results></pagefind-results>
```

These three components will work together automatically — `pagefind-input` triggers searches, `pagefind-summary` shows the count, and `pagefind-results` displays the matches.

<div class="demo-box">
<div class="inline-search-demo">
<pagefind-input instance="components-inline"></pagefind-input>
<pagefind-summary instance="components-inline"></pagefind-summary>
<pagefind-results instance="components-inline"></pagefind-results>
</div>
</div>

## Named Instances

Use the `instance` attribute to create separate, independent search interfaces on the same page:

```html
<!-- Documentation search -->
<pagefind-input instance="docs" placeholder="Search docs..."></pagefind-input>
<pagefind-results instance="docs"></pagefind-results>

<!-- Blog search -->
<pagefind-input instance="blog" placeholder="Search blog..."></pagefind-input>
<pagefind-results instance="blog"></pagefind-results>
```

Components with the same `instance` value share state. Components with different instance values are completely independent — they maintain separate search terms, filters, and results.

## Component Categories

### Full Experiences

These components provide complete, ready-to-use search interfaces:

- **[Modal](/components/modal/)** — A dialog search with keyboard navigation. Supports custom inner structure.
- **[Modal Trigger](/components/modal-trigger/)** — A button that opens the modal
- **[Searchbox](/components/searchbox/)** — A compact dropdown search. Supports [custom result templates](/components/searchbox/#custom-templates).

### Building Blocks

These components can be composed together for custom layouts:

- **[Input](/components/input/)** — The search text field
- **[Results](/components/results/)** — The list of search results
- **[Summary](/components/summary/)** — Shows result count and search status
- **[Keyboard Hints](/components/keyboard-hints/)** — Shows contextual keyboard shortcuts
- **[Filter Dropdown](/components/filter-dropdown/)** — A dropdown to filter by a specific facet
- **[Filter Pane](/components/filter-pane/)** — A sidebar-style filter panel with checkboxes

### Configuration

- **[Config](/components/config/)** — Set options for the Component UI, or for Pagefind itself
