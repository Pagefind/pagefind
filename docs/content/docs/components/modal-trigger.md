---
title: "<pagefind-modal-trigger>"
nav_title: "<pagefind-modal-trigger>"
nav_section: Components
weight: 60
---

A button that opens the associated modal.

<div class="demo-box demo-box-attached">
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
| `placeholder` | string | Language dependent | Text shown on the trigger button |
| `shortcut` | string | `"mod+k"` | Keyboard shortcut to open modal |
| `hide-shortcut` | boolean | `false` | Hide the keyboard shortcut display |
| `compact` | boolean | `false` | Show only the search icon, no text |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Shortcut

By default, the trigger listens for `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux) to open the modal. The keyboard listener is attached at the document level, so the shortcut works from anywhere on the page.

### Customizing the Shortcut

You can customize the keyboard shortcut using the `shortcut` attribute:

```html
<!-- Single key (no modifier) -->
<pagefind-modal-trigger shortcut="/"></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>

<!-- With modifier -->
<pagefind-modal-trigger shortcut="mod+p"></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

Example with `/` key:

<div class="demo-box">
<pagefind-modal-trigger shortcut="/" instance="trigger-demo-slash"></pagefind-modal-trigger>
<pagefind-modal instance="trigger-demo-slash"></pagefind-modal>
</div>

Supported syntax:

- **Platform modifier:** `mod` (Ctrl on Windows/Linux, Cmd on Mac)
- **Explicit modifiers:** `ctrl`, `shift`, `alt`, `cmd`/`meta`
- **Keys:** Any single character (e.g. `k`, `/`) or key name
- **Case-insensitive:** Keys are normalized to lowercase

The shortcut display auto-detects the platform to show the correct modifier key (⌘ on Mac, Ctrl on other platforms).
