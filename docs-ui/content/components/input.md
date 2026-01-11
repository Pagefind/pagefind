---
title: "<pagefind-input>"
nav_title: "<pagefind-input>"
nav_section: Components
weight: 51
---

A search input field with debouncing.

<div class="demo-box">
<pagefind-input instance="input-demo"></pagefind-input>
</div>

```html
<pagefind-input></pagefind-input>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `placeholder` | string | Language dependent | Placeholder text shown when input is empty |
| `debounce` | number | `300` | Milliseconds to wait after typing before searching |
| `autofocus` | boolean | `false` | Focus input when page loads |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Behavior

- Focusing the input begins loading the Pagefind search bundle
- When search is triggered by other components, this input's value is automatically updated
- When focused, the input registers keyboard shortcuts with the instance

## Keyboard Behavior

| Key | Action |
|-----|--------|
| `Escape` | Clear the input and reset search |
| `↓` | Move focus to the results list |
