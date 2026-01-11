---
title: "<pagefind-config>"
nav_title: "<pagefind-config>"
nav_section: Components
weight: 59
---

Declaratively configure a Pagefind instance. Optionally include this on the page to set options like bundle path.

```html
<pagefind-config bundle-path="/search/pagefind/"></pagefind-config>

<pagefind-input></pagefind-input>
<pagefind-results></pagefind-results>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `bundle-path` | string | auto-detected | Path to Pagefind bundle files |
| `base-url` | string | `""` | Base URL prepended to all result links |
| `excerpt-length` | number | `30` | Length of result excerpts (in words) |
| `instance` | string | `"default"` | Instance name to configure |
| `preload` | boolean | `false` | Load Pagefind immediately instead of on first search |
| `faceted` | boolean | `false` | Enable faceted search mode |

### Bundle Path Detection

By default, `bundle-path` is automatically detected from the location of the Pagefind script. If the script is loaded from `/assets/pagefind/pagefind-component-ui.js`, the bundle path defaults to `/assets/pagefind/`. If detection fails, it falls back to `/pagefind/`.

### Important: Attributes Are Read Once

Attributes are only read when the component connects to the DOM. Changing attributes dynamically after connection has no effect. For dynamic configuration, use the JavaScript API instead.

## Examples

### Custom Bundle Path

When your Pagefind files are in a non-standard location:

```html
<pagefind-config bundle-path="/docs/search/pagefind/"></pagefind-config>
```

### External Bundle

Load from another domain:

```html
<pagefind-config bundle-path="https://example.com/pagefind/"></pagefind-config>
```

### Preload

Load Pagefind immediately on page load (instead of waiting for first search):

```html
<pagefind-config preload></pagefind-config>
```

This improves perceived performance for search-only pages but adds initial page weight.

### Faceted Search Mode

Enable faceted/browse mode for catalog-style interfaces where users filter content without requiring a search term:

```html
<pagefind-config faceted preload></pagefind-config>
<pagefind-filter-dropdown filter="category"></pagefind-filter-dropdown>
<pagefind-results></pagefind-results>
```

When `faceted` is enabled:
- Searching with an empty term returns all results (instead of showing nothing)
- Results update immediately when filters are changed

Combine with `preload` to show all results immediately on page load. Without `preload`, results appear after the first user interaction (e.g., opening a filter dropdown).

### Named Instance

Configure a specific instance:

```html
<pagefind-config instance="docs" preload bundle-path="/docs/pagefind/"></pagefind-config>
<pagefind-config instance="blog" bundle-path="/blog/pagefind/"></pagefind-config>

<!-- Use each instance -->
<pagefind-input instance="docs" placeholder="Search docs..."></pagefind-input>
<pagefind-input instance="blog" placeholder="Search blog..."></pagefind-input>
```

## Programmatic Configuration

For complex configuration, use JavaScript instead of the `<pagefind-config>` element:

```html
<script type="module">
  import { configureInstance } from '/pagefind/pagefind-component-ui.js';

  configureInstance("default", {
    bundlePath: "/pagefind/",
    mergeIndex: [
      { bundlePath: "/other-site/pagefind/" }
    ],
    ranking: {
      termFrequency: 0.8
    }
  });
</script>

<pagefind-input></pagefind-input>
<pagefind-results></pagefind-results>
```

Either place this on the page as shown, or compile it into your existing JavaScript bundle.

See the [main Pagefind documentation](https://pagefind.app/docs/) for all available options.
