---
title: "Configuring the Pagefind search in the browser"
nav_title: "Search API config"
nav_section: References
weight: 60
---

The behaviour of the Pagefind search API can be configured in the browser.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
If using the Component UI, options can be set declaratively with the `<pagefind-config>` element:

```html
+<pagefind-config base-url="/"></pagefind-config>

<pagefind-input></pagefind-input>
<pagefind-results></pagefind-results>
```

See the full list of available attributes in the [`<pagefind-config>` reference](/docs/components/config/).
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
Options can be passed through to Pagefind via `configureInstance`. Import the function based on how you installed the Component UI:

```js
// If installed via a package manager:
import { configureInstance } from '@pagefind/component-ui';
// If loaded via script tag:
import { configureInstance } from '/pagefind/pagefind-component-ui.js';

+configureInstance("default", {
+    baseUrl: "/",
+    // ... more search options
+});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
If interfacing with Pagefind directly, options can be passed via awaiting `pagefind.options()`:

```js
const pagefind = await import("/pagefind/pagefind.js");
+await pagefind.options({
+    baseUrl: "/",
+    // ... more search options
+});
```
{{< /tab >}}
{{< /tabs >}}

## Available options

### Base URL

Defaults to "/". If hosting a site on a subpath, `baseUrl` can be provided, and will be appended to the front of all search result URLs.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config base-url="/docs/"></pagefind-config>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    baseUrl: "/docs/"
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    baseUrl: "/docs/"
});
```
{{< /tab >}}
{{< /tabs >}}

### Bundle path

Overrides the bundle directory. In most cases this should be automatically detected by the import URL. Set this if search isn't working and you are seeing a console warning that this path could not be detected.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config bundle-path="/subpath/pagefind/"></pagefind-config>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    bundlePath: "/subpath/pagefind/"
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    bundlePath: "/subpath/pagefind/"
});
```
{{< /tab >}}
{{< /tabs >}}

### Excerpt length

Set the maximum length for generated excerpts. Defaults to `30`.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
```html
<pagefind-config excerpt-length="15"></pagefind-config>
```
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    excerptLength: 15
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    excerptLength: 15
});
```
{{< /tab >}}
{{< /tabs >}}

### Highlight query parameter

If set, Pagefind will add the search term as a query parameter under the same name.

If using the [Pagefind highlight script](/docs/highlighting/), make sure this is configured to match.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
Not available as a declarative attribute. Use `configureInstance` instead.
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    highlightParam: "highlight"
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    highlightParam: "highlight"
});
```
{{< /tab >}}
{{< /tabs >}}

### Exact Diacritics

Defaults to `false`. When set to `true`, diacritics (accents such as àéö) are treated as fully distinct characters.

By default, Pagefind normalizes diacritics so that searching for "cafe" will match pages containing "café" and vice versa. When diacritics are normalized, exact matches are still preferred via the [`ranking.diacriticSimilarity`](/docs/ranking/#configuring-diacritic-similarity) parameter.

When `exactDiacritics` is set to `true`:
- Searching for "café" will only match pages containing "café"
- Searching for "cafe" will only match pages containing "cafe"

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
Not available as a declarative attribute. Use `configureInstance` instead.
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    exactDiacritics: true
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    exactDiacritics: true
});
```
{{< /tab >}}
{{< /tabs >}}

### Ranking

See [customize ranking](/docs/ranking/)

### Index weight

See [multisite search > weighting](/docs/multisite/#changing-the-weighting-of-individual-indexes)

### Merge filter

See [multisite search > filtering](/docs/multisite/#filtering-results-by-index)

### Disable web worker

Defaults to `false`. If set to `true`, forces Pagefind to run all search operations on the main thread instead of using a web worker.

By default, Pagefind will attempt to use a web worker for search operations when available, which helps keep the main thread responsive during searches. If web workers are not supported or fail to initialize, Pagefind will automatically fall back to running on the main thread.

{{< tabs >}}
{{< tab "UI (declarative)" "sync-cfg-declarative" >}}
Not available as a declarative attribute. Use `configureInstance` instead.
{{< /tab >}}
{{< tab "UI (programmatic)" "sync-cfg-programmatic" >}}
```js
configureInstance("default", {
+    noWorker: true
});
```
{{< /tab >}}
{{< tab "Search API" "sync-cfg-searchapi" >}}
```js
await pagefind.options({
+    noWorker: true
});
```
{{< /tab >}}
{{< /tabs >}}
