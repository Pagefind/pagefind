---
title: "<pagefind-searchbox>"
nav_title: "<pagefind-searchbox>"
nav_section: Components
weight: 70
---

An all-in-one search component combining input, results dropdown, and keyboard navigation. Results appear in a floating dropdown below the input and load their full data only when scrolled into view using the Intersection Observer API.

<div class="demo-box demo-box-attached">
<pagefind-searchbox instance="searchbox-ref"></pagefind-searchbox>
</div>

```html
<pagefind-searchbox></pagefind-searchbox>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `placeholder` | string | Language dependent | Placeholder text for the input |
| `debounce` | number | `150` | Milliseconds to wait after typing |
| `max-results` | number | unlimited | Limit number of results shown |
| `show-sub-results` | boolean | `false` | Show anchor-based sub-results in dropdown |
| `show-keyboard-hints` | boolean | `true` | Show keyboard navigation hints in footer |
| `autofocus` | boolean | `false` | Focus input on page load |
| `shortcut` | string | `"mod+k"` | Keyboard shortcut to focus the input |
| `hide-shortcut` | boolean | `false` | Hide the keyboard shortcut display |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Shortcut

By default, the searchbox listens for `Cmd+K` (Mac) or `Ctrl+K` (Windows/Linux) to focus the input, and displays the shortcut hint inside the input field.

```html
<!-- Default: Cmd+K / Ctrl+K -->
<pagefind-searchbox></pagefind-searchbox>

<!-- Custom shortcut -->
<pagefind-searchbox shortcut="/"></pagefind-searchbox>

<!-- Hide the visual shortcut hint -->
<pagefind-searchbox hide-shortcut></pagefind-searchbox>
```

The `shortcut` attribute accepts the same syntax as [`<pagefind-modal-trigger>`](/docs/components/modal-trigger/#customizing-the-shortcut).

## Keyboard Navigation

Full keyboard support is built in:

| Key | Action |
|-----|--------|
| `↓` / `↑` | Navigate through results |
| `Enter` | Go to selected result |
| `Shift+Enter` | Open result in new tab |
| `Ctrl+Enter` / `Cmd+Enter` | Open result in new tab |
| `Escape` | Close dropdown |
| `Tab` | Move focus away |

Both `Shift+Enter` and the platform modifier (`Ctrl` on Windows/Linux, `Cmd` on Mac) with `Enter` will open the result in a new tab.

## Custom Templates

Like `<pagefind-results>`, you can customize rendering with `<script type="text/pagefind-template">` elements. Templates use a simple syntax with variables, conditionals, and loops.

### Basic Template

```html
<pagefind-searchbox>
  <script type="text/pagefind-template">
    <a class="my-result" id="{{ aria.result_id }}" href="{{ meta.url | default(url) | safeUrl }}" role="option" aria-selected="false" aria-labelledby="{{ aria.title_id }}">
      <p id="{{ aria.title_id }}">{{ meta.title }}</p>
      {{#if excerpt}}
      <p>{{+ excerpt +}}</p>
      {{/if}}
    </a>
  </script>
</pagefind-searchbox>
```

The full templating syntax can be seen in the [adequate-little-templates](https://github.com/bglw/adequate-little-templates?tab=readme-ov-file#syntax) package.

### Available Data

| Field | Description |
|-------|-------------|
| `meta.title` | Page title |
| `meta.url` | Page URL |
| `meta.*` | Any custom metadata |
| `excerpt` | Search excerpt with `<mark>` highlighting (use `{{+ excerpt +}}`) |
| `url` | Fallback URL |
| `sub_results` | Array of matching sections (when `show-sub-results` is set) |
| `options.show_sub_results` | Whether `show-sub-results` is set on the searchbox |
| `aria.result_id` | Unique ID for ARIA attributes |
| `aria.title_id` | ID for the title element |
| `aria.excerpt_id` | ID for the excerpt element |

Each sub-result has: `title`, `url`, `excerpt`, and its own `aria` object.

### Full Template Example

Here's the built-in template. Copy and customize it:

```html
<pagefind-searchbox>
  <script type="text/pagefind-template">
    {{#if and(options.show_sub_results, sub_results)}}<div class="pf-searchbox-group" role="group" aria-label="{{ meta.title | default('Untitled') }}">{{/if}}
    <a class="pf-searchbox-result" id="{{ aria.result_id }}" href="{{ meta.url | default(url) | safeUrl }}" role="option" aria-selected="false" aria-labelledby="{{ aria.title_id }}"{{#if excerpt}} aria-describedby="{{ aria.excerpt_id }}"{{/if}}>
      <p class="pf-searchbox-result-title" id="{{ aria.title_id }}">{{ meta.title | default("Untitled") }}</p>
      {{#if excerpt}}
      <p class="pf-searchbox-result-excerpt" id="{{ aria.excerpt_id }}">{{+ excerpt +}}</p>
      {{/if}}
    </a>
    {{#if and(options.show_sub_results, sub_results)}}
    {{#each sub_results as sub}}
    <a class="pf-searchbox-result pf-searchbox-subresult" id="{{ sub.aria.result_id }}" href="{{ sub.url | safeUrl }}" role="option" aria-selected="false" aria-labelledby="{{ sub.aria.title_id }}"{{#if sub.excerpt}} aria-describedby="{{ sub.aria.excerpt_id }}"{{/if}}>
      <p class="pf-searchbox-result-title" id="{{ sub.aria.title_id }}">{{ sub.title | default("Section") }}</p>
      {{#if sub.excerpt}}
      <p class="pf-searchbox-result-excerpt" id="{{ sub.aria.excerpt_id }}">{{+ sub.excerpt +}}</p>
      {{/if}}
    </a>
    {{/each}}
    </div>{{/if}}
  </script>
</pagefind-searchbox>
```

### Template Requirements

The searchbox implements the [ARIA combobox](https://www.w3.org/WAI/ARIA/apg/patterns/combobox/) pattern, so custom templates must include specific attributes for keyboard navigation and screen readers to work. Without these, keyboard navigation will skip your results entirely.

- Each navigable item must be an `<a>` element with a valid `href`, `role="option"`, and `aria-selected="false"`
- Set `id="{{ aria.result_id }}"` (or `{{ sub.aria.result_id }}` for sub-results) so the input can track the active selection
- The root element of your template must have `role="option"` or `role="group"`
- Without sub-results, your template root is the `<a role="option">` itself
- With sub-results, your template root is a `role="group"` element wrapping the main result and its sub-results together, with each navigable item as a `role="option"` child (as shown in the full template above)
