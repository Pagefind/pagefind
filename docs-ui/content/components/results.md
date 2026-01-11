---
title: "<pagefind-results>"
nav_title: "<pagefind-results>"
nav_section: Components
weight: 52
---

Displays search results with automatic lazy loading. Results load their full data only when scrolled into view.

<div class="demo-box" style="flex-direction: column; gap: 1rem; height: 400px;">
<pagefind-input instance="results-demo"></pagefind-input>
<div class="results-scroll" style="flex: 1; overflow-y: auto; width: 100%; min-height: 0;">
<pagefind-results instance="results-demo"></pagefind-results>
</div>
<pagefind-keyboard-hints instance="results-demo"></pagefind-keyboard-hints>
</div>

```html
<pagefind-results></pagefind-results>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `show-images` | boolean | `false` | Show/hide result images |
| `hide-sub-results` | boolean | `false` | Hide anchor-based sub-results |
| `max-sub-results` | number | `3` | Maximum number of sub-results per result |
| `link-target` | string | — | Set `target` attribute on result links (e.g., `_blank` for new tab) |
| `instance` | string | `"default"` | Connect to a specific Pagefind instance |

## Keyboard Navigation

When results are focused, use these keys to navigate:

| Key | Action |
|-----|--------|
| `↓` / `↑` | Navigate between results |
| `Enter` | Follow the selected link |
| `Backspace` | Focus input and delete last character |
| `/` | Focus the search input |
| Any letter | Focus input and type the character |

## Custom Templates

Customize result rendering using `<script type="text/pagefind-template">` elements inside the component. Templates use a simple syntax with variables, conditionals, and loops.

### Basic Template

```html
<pagefind-results>
  <script type="text/pagefind-template">
    <li class="my-result">
      <h3>{{ meta.title }}</h3>
      <a href="{{ url | safeUrl }}">{{ url }}</a>
      <p>{{+ excerpt +}}</p>
    </li>
  </script>
</pagefind-results>
```

The full templating syntax can be seen in the [adequate-little-templates](https://github.com/bglw/adequate-little-templates?tab=readme-ov-file#syntax) package.

### Available Data

Templates receive this data structure:

| Field | Description |
|-------|-------------|
| `meta.title` | Page title |
| `meta.url` | Page URL |
| `meta.image` | Page image (if set) |
| `meta.image_alt` | Image alt text |
| `meta.*` | Any custom metadata |
| `excerpt` | Search excerpt with `<mark>` highlighting (use `{{+ excerpt +}}`) |
| `url` | Fallback URL |
| `sub_results` | Array of matching sections within the page |
| `options.*` | The attributes supplied to the `<pagefind-results>` element |

Each sub-result has: `title`, `url`, `excerpt`.

### Full Template Example

Here's the built-in template. Copy and customize it:

```html
<pagefind-results>
  <script type="text/pagefind-template">
    <li class="pf-result">
      <div class="pf-result-card">
        {{#if and(options.show_images, meta.image)}}
        <img class="pf-result-image" src="{{ meta.image }}" alt="{{ meta.image_alt | default(meta.title) }}">
        {{/if}}
        <div class="pf-result-content">
          <p class="pf-result-title">
            <a class="pf-result-link" href="{{ meta.url | default(url) | safeUrl }}"{{#if options.link_target}} target="{{ options.link_target }}"{{/if}}>{{ meta.title }}</a>
          </p>
          {{#if excerpt}}
          <p class="pf-result-excerpt">{{+ excerpt +}}</p>
          {{/if}}
        </div>
      </div>
      {{#if sub_results}}
      <ul class="pf-heading-chips">
        {{#each sub_results as sub}}
        <li class="pf-heading-chip">
          <a class="pf-heading-link" href="{{ sub.url | safeUrl }}">{{ sub.title }}</a>
          <p class="pf-heading-excerpt">{{+ sub.excerpt +}}</p>
        </li>
        {{/each}}
      </ul>
      {{/if}}
    </li>
  </script>
</pagefind-results>
```
