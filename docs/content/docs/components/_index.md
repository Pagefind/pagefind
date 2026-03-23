---
title: "Component System"
nav_title: "Component System"
weight: 2
---

The Pagefind Component UI is a set of web components that communicate through shared instances. This allows you to compose search interfaces from individual pieces, or use the prebuilt components that bundle everything together.

## How Components Connect

Components on the same page automatically connect to each other. In this example, `pagefind-input` triggers searches, `pagefind-summary` shows the count, and `pagefind-results` displays the matches.

<div class="demo-box demo-box-attached demo-box-inline-search" style="flex-direction: column;">
<pagefind-input instance="components-inline" style="width: 100%;"></pagefind-input>
<pagefind-summary instance="components-inline" style="width: 100%;"></pagefind-summary>
<div class="results-scroll" style="max-height: 350px; overflow-y: auto; width: 100%;">
<pagefind-results instance="components-inline"></pagefind-results>
</div>
</div>

```html
<pagefind-input></pagefind-input>
<pagefind-summary></pagefind-summary>
<pagefind-results></pagefind-results>
```

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

- **[Modal](/docs/components/modal/)** — A dialog search with keyboard navigation. Supports custom inner structure
- **[Modal Trigger](/docs/components/modal-trigger/)** — A button that opens the modal
- **[Searchbox](/docs/components/searchbox/)** — A compact dropdown search

### Building Blocks

These components can be composed together for custom layouts:

- **[Input](/docs/components/input/)** — The search text field
- **[Results](/docs/components/results/)** — The list of search results
- **[Summary](/docs/components/summary/)** — Shows result count and search status
- **[Keyboard Hints](/docs/components/keyboard-hints/)** — Shows contextual keyboard shortcuts
- **[Filter Dropdown](/docs/components/filter-dropdown/)** — A dropdown to filter by a specific facet
- **[Filter Pane](/docs/components/filter-pane/)** — A sidebar-style filter panel with checkboxes

### Configuration

- **[Config](/docs/components/config/)** — Set options for the Component UI, or for Pagefind itself
