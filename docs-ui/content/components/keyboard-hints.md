---
title: "<pagefind-keyboard-hints>"
nav_title: "<pagefind-keyboard-hints>"
nav_section: Components
weight: 54
---

Displays contextual keyboard shortcuts based on which component currently has focus.

<div class="demo-box" style="flex-direction: column; gap: 1rem;">
<pagefind-input placeholder="Click me!" instance="hints-demo"></pagefind-input>
<pagefind-keyboard-hints instance="hints-demo"></pagefind-keyboard-hints>
</div>

```html
<pagefind-keyboard-hints></pagefind-keyboard-hints>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## How It Works

Keyboard hints are contextual. Different components register their shortcuts when focused:

| Component Focus | Shortcuts Shown |
|----------------|-----------------|
| Input | `↓ navigate`, `esc clear` |
| Results | `↑↓ navigate`, `enter select`, `esc close` |
| Modal | `esc close` |

When no component has focus, the keyboard hints area is empty.

## Accessibility

This component is marked `aria-hidden="true"` because it's a visual aid for sighted keyboard users.

The Component UI shortcuts are either the expected controls for WAI-ARIA patterns, or they are optional alternatives to standard tab navigation patterns.

### Touch Devices

Keyboard hints are automatically hidden on touch-only devices.
