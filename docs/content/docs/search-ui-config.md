---
title: "Component UI configuration"
weight: 3
---

To configure the Component UI, set options declaratively with the [`<pagefind-config>`](/docs/components/config/) element, or programmatically with `configureInstance()`.

Options shared with the search API, such as `bundlePath`, `baseUrl`, and `ranking`, are documented in [Search API config](/docs/search-config/).

## Component UI Options

### Preload

By default, Pagefind is only loaded when a search is first run, or when a search component is focused. Set `preload` to load it immediately instead.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config preload></pagefind-config>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    preload: true
});
```
{{< /tab >}}
{{< /tabs >}}

This improves the perceived performance of search-only pages, where a search is likely to happen, but adds initial page weight.

When writing [custom components](/docs/custom-components/#preloading), the same can be triggered with `instance.triggerLoad()`.

### Faceted search

Enable faceted search for catalog-style interfaces, where users browse and filter content without needing to enter a search term.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config faceted preload></pagefind-config>
<pagefind-filter-dropdown filter="category"></pagefind-filter-dropdown>
<pagefind-results></pagefind-results>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    faceted: true,
+    preload: true
});
```
{{< /tab >}}
{{< /tabs >}}

When `faceted` is enabled:

- Searching with an empty term returns all results, instead of showing nothing
- Results update immediately when filters are changed

Combine with `preload` to show all results as soon as the page loads. Without `preload`, results appear after the first interaction, such as opening a filter dropdown.

### Result sorting

Order results by a [sort key](/docs/sorts/) instead of by relevance, as `key:asc` or `key:desc`.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config sort="date:desc"></pagefind-config>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    sort: "date:desc"
});
```
{{< /tab >}}
{{< /tabs >}}

Sorting replaces relevancy ordering entirely, and pages that aren't tagged with the given sort key are excluded from the results. As with the [search API](/docs/js-api-sorting/), the sort can also be given as an object, `{ date: "desc" }`.

### Process term

Provide a function to transform the search term before it is passed to Pagefind.

The processed term is only used for the search query. The raw term is still shown in the search input, announced to screen readers, and emitted by the `search` event.

{{< diffcode >}}
```js
configureInstance("default", {
+    processTerm: (term) => term.replace(/aa/g, "ā")
});
```
{{< /diffcode >}}

As `processTerm` is a function, it can't be set using the `<pagefind-config>` element.
