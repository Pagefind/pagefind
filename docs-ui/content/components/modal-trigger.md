---
title: "<pagefind-modal-trigger>"
nav_title: "<pagefind-modal-trigger>"
nav_section: Components
weight: 55
---

A button that opens the associated modal.

<div class="demo-box">
<pagefind-modal-trigger instance="trigger-demo"></pagefind-modal-trigger>
<pagefind-modal instance="trigger-demo"></pagefind-modal>
</div>

```html
<pagefind-modal-trigger></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `placeholder` | string | `"Search"` | Text shown on the trigger button |
| `shortcut` | string | `"k"` | Key that opens modal (with Cmd/Ctrl) |
| `hide-shortcut` | boolean | `false` | Hide the keyboard shortcut display |
| `compact` | boolean | `false` | Show only the search icon, no text |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Shortcut

By default, the trigger listens for `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux) to open the modal. The keyboard listener is attached at the document level, so the shortcut works from anywhere on the page.

The shortcut display auto-detects the platform to show the correct modifier key (⌘ on Mac, Ctrl on other platforms). Shortcuts are case-insensitive.
