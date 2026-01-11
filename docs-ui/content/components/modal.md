---
title: "<pagefind-modal>"
nav_title: "<pagefind-modal>"
nav_section: Components
weight: 54
---

A modal overlay container for search. Opens over the page content and traps focus until closed.

<div class="demo-box">
<pagefind-modal-trigger instance="modal-ref"></pagefind-modal-trigger>
<pagefind-modal instance="modal-ref"></pagefind-modal>
</div>

```html
<pagefind-modal-trigger></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `reset-on-close` | boolean | `false` | Clear the search input when modal closes |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Default Structure

When used without child elements, the modal auto-generates a default structure:

```html
<!-- This empty modal... -->
<pagefind-modal></pagefind-modal>

<!-- ...generates this structure: -->
<pagefind-modal>
  <pagefind-modal-header>
    <pagefind-input></pagefind-input>
  </pagefind-modal-header>
  <pagefind-modal-body>
    <pagefind-summary></pagefind-summary>
    <pagefind-results></pagefind-results>
  </pagefind-modal-body>
  <pagefind-modal-footer>
    <pagefind-keyboard-hints></pagefind-keyboard-hints>
  </pagefind-modal-footer>
</pagefind-modal>
```

If you want to customize the contents of your own modal, the above is a good starting point.

## Structure Components

The modal uses three structural components. These are optional if you're providing your own modal contents, but it's recommended to place your components within them.

### `<pagefind-modal-header>`

Fixed header area, typically contains the search input. Automatically includes a close button visible on mobile.

### `<pagefind-modal-body>`

Scrollable content area, typically contains results.

### `<pagefind-modal-footer>`

Optional fixed footer area.
