---
title: "<pagefind-config>"
nav_title: "<pagefind-config>"
nav_section: Components
weight: 100
---

Declaratively configure a Pagefind instance. Optionally include this on the page to set options like bundle path.

This element is one of the two ways to configure the Component UI, the other being `configureInstance()`.

```html
<pagefind-config bundle-path="/search/pagefind/"></pagefind-config>

<pagefind-input></pagefind-input>
<pagefind-results></pagefind-results>
```

## Attributes

| Attribute | Type | Default | Description |
|-----------|------|---------|-------------|
| `bundle-path` | string | auto-detected | Path to Pagefind bundle files |
| `base-url` | string | `"/"` | Base URL prepended to all result links |
| `excerpt-length` | number | `30` | Length of result excerpts (in words) |
| `excerpt-ellipsis` | string | — | String appended to truncated excerpts; see [Excerpt ellipsis](/docs/search-config/#excerpt-ellipsis) |
| `lang` | string | auto-detected | Override the detected language for stemming and translations |
| `instance` | string | `"default"` | Instance name to configure |
| `preload` | boolean | `false` | Load Pagefind immediately instead of on first search |
| `faceted` | boolean | `false` | Enable faceted search mode |
| `highlight-param` | string | — | Add the search term as a query parameter under this key for use with the [highlight script](/docs/highlighting/) |
| `exact-diacritics` | boolean | `false` | Treat diacritics as distinct characters instead of normalizing them |
| `prefix-matching` | string | `"all"` | Which words of a search may match longer words: `"all"`, `"last"`, or `"none"`. See [prefix matching](/docs/search-config/#prefix-matching) |
| `meta-cache-tag` | string | — | Replace the default cache-busting timestamp with a fixed string for offline/PWA support |
| `no-worker` | boolean | `false` | Force Pagefind to run on the main thread instead of a web worker |
| `sort` | string | — | Order results by a [sort key](/docs/sorts/) rather than by relevance, as `key:asc` or `key:desc` |

The `preload`, `faceted`, and `sort` attributes are specific to the Component UI, and are covered in [Component UI configuration](/docs/search-ui-config/), along with their `configureInstance()` equivalents.

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

Options that can't be expressed as attributes, such as functions, or objects like `ranking` and `mergeIndex`, can be set with `configureInstance()` instead. Most attributes above have a camelCase equivalent, such as `bundlePath` for `bundle-path`.

See [Search API config](/docs/search-config/) for how to call `configureInstance()`, and [Component UI configuration](/docs/search-ui-config/) for the options specific to the Component UI.
