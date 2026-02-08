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
| `shortcut` | string | `"ctrl+k"` | Keyboard shortcut to open modal |
| `hide-shortcut` | boolean | `false` | Hide the keyboard shortcut display |
| `compact` | boolean | `false` | Show only the search icon, no text |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Shortcut

By default, the trigger listens for `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux) to open the modal. The keyboard listener is attached at the document level, so the shortcut works from anywhere on the page.

### Customizing the Shortcut

You can customize the keyboard shortcut using the `shortcut` attribute:

```html
<!-- Single key (no modifier) -->
<pagefind-modal-trigger shortcut="slash"></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>

<!-- With Ctrl/Cmd modifier -->
<pagefind-modal-trigger shortcut="cmd+p"></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

Example with `/` key:

<div class="demo-box">
<pagefind-modal-trigger shortcut="slash" instance="trigger-demo-slash"></pagefind-modal-trigger>
<pagefind-modal instance="trigger-demo-slash"></pagefind-modal>
</div>

Supported syntax:

- **Single key:** `slash` (becomes `/`), or any single character key
- **With modifier:** `ctrl+k` or `cmd+k` (automatically uses Cmd on Mac, Ctrl on Windows/Linux)
- **Case-insensitive:** Keys are normalized to lowercase

The shortcut display auto-detects the platform to show the correct modifier key (⌘ on Mac, Ctrl on other platforms).
