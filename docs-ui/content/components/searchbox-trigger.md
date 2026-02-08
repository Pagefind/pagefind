---
title: "<pagefind-searchbox-trigger>"
nav_title: "<pagefind-searchbox-trigger>"
nav_section: Components
weight: 57
---

A keyboard shortcut listener that focuses the associated searchbox. By default, displays the keyboard shortcut hint inside the searchbox input.

<div class="demo-box">
<pagefind-searchbox-trigger instance="trigger-demo"></pagefind-searchbox-trigger>
<pagefind-searchbox instance="trigger-demo"></pagefind-searchbox>
</div>

```html
<pagefind-searchbox-trigger></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `shortcut` | string | `"ctrl+k"` | Keyboard shortcut to focus the searchbox |
| `hide-shortcut` | boolean | `false` | Hide the keyboard shortcut display |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Shortcut

By default, the trigger listens for `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux) to focus the searchbox. The keyboard listener is attached at the document level, so the shortcut works from anywhere on the page.

### Platform-Independent Shortcuts

The `ctrl` modifier is **automatically platform-aware**:
- `shortcut="ctrl+k"` → Uses `Ctrl+K` on Windows/Linux, `Cmd+K` on Mac

This makes it easy to write cross-platform shortcuts without detecting the user's OS.

### Customizing the Shortcut

```html
<!-- Single key (no modifier) -->
<pagefind-searchbox-trigger shortcut="slash"></pagefind-searchbox-trigger>

<!-- Platform-independent (recommended) -->
<pagefind-searchbox-trigger shortcut="ctrl+k"></pagefind-searchbox-trigger>

<!-- Explicit Cmd on all platforms -->
<pagefind-searchbox-trigger shortcut="cmd+k"></pagefind-searchbox-trigger>
```

Supported syntax:

- **Single key:** `slash` (becomes `/`), or any single character key
- **With modifier:** `ctrl+k` or `cmd+k` (automatically uses Cmd on Mac, Ctrl on Windows/Linux)
- **Case-insensitive:** Keys are normalized to lowercase

## Usage with Searchbox

The trigger automatically connects to searchboxes in the same Pagefind instance:

```html
<pagefind-searchbox-trigger></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>
```

For multiple instances, use the `instance` attribute:

```html
<pagefind-searchbox-trigger instance="my-search"></pagefind-searchbox-trigger>
<pagefind-searchbox instance="my-search"></pagefind-searchbox>
```

## Visual Display

By default, the trigger displays the keyboard shortcut hint inside the searchbox input field on the right side. This provides a visual cue to users about the available keyboard shortcut.

### Hiding the Shortcut Display

To hide the visual shortcut hint while keeping the keyboard functionality:

```html
<pagefind-searchbox-trigger hide-shortcut></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>
```

Example with hidden shortcut and the slash key:

<div class="demo-box">
<pagefind-searchbox-trigger hide-shortcut shortcut="slash" instance="trigger-demo-hidden"></pagefind-searchbox-trigger>
<pagefind-searchbox instance="trigger-demo-hidden"></pagefind-searchbox>
</div>

## Examples

```html
<!-- Default: Shows Ctrl K (or Cmd K on Mac) inside the searchbox -->
<pagefind-searchbox-trigger></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>

<!-- Focus on slash key (shows / inside the searchbox) -->
<pagefind-searchbox-trigger shortcut="slash"></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>

<!-- Hide the visual hint but keep the keyboard shortcut -->
<pagefind-searchbox-trigger hide-shortcut></pagefind-searchbox-trigger>
<pagefind-searchbox></pagefind-searchbox>
```
