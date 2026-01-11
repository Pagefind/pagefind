---
title: "<pagefind-summary>"
nav_title: "<pagefind-summary>"
nav_section: Components
weight: 53
---

Displays search status and result count. Shows messages like "12 results for 'search term'".

<div class="demo-box" style="flex-direction: column; gap: 1rem;">
<pagefind-input instance="summary-demo"></pagefind-input>
<pagefind-summary instance="summary-demo"></pagefind-summary>
</div>

```html
<pagefind-summary></pagefind-summary>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `default-message` | string | `""` | Message shown before any search |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## States

The summary displays different messages based on search state:

| State | Display |
|-------|---------|
| No search | `default-message` attribute value |
| Searching | `Searching for {term}...` |
| Results found | `{count} result(s) for {term}` |
| Results found, only filtering | `{count} result(s)` |
| No results | `0 results for {term}` |
| Error | `Error: {message}` |
