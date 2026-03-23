---
title: Pagefind Component UI
nav_title: Home
weight: 1
new_ui: true
---

Use one of the prebuilt search experiences, or assemble your own. Either way, you can customize most things.
These components follow best practices for WAI-ARIA components, and are designed to be hard to misuse. Additionally, all components translate both visible and assistive text automatically to match the language of the website.

## Quick Start

{{< tabs >}}
{{< tab "Script tag" "sync-ui-scripttag" >}}
After running Pagefind, include the stylesheet and script from your output directory:

```html
<link href="/pagefind/pagefind-component-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-component-ui.js" type="module"></script>
```
{{< /tab >}}
{{< tab "Import" "sync-ui-import" >}}
Install the package:

```bash
npm install @pagefind/component-ui
```

Import the CSS and components in your JavaScript:

```javascript
import '@pagefind/component-ui';
import '@pagefind/component-ui/css';
```
{{< /tab >}}
{{< /tabs >}}

Then, choose your search style. The Component UI offers two "prebuilt" drop-in options, or you can go in any direction using the components individually.

## Modal Search

The most universal search UI that doesn't need tight integration with your site.

<div class="demo-box demo-box-attached">
<pagefind-modal-trigger instance="home-modal"></pagefind-modal-trigger>
<pagefind-modal instance="home-modal"></pagefind-modal>
</div>

```html
<pagefind-modal-trigger></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

If you want to tweak things, you can build up the entirety of the modal internals using the rest of the Component UI. Like most of the component UI, you can also hook your own components in.

[Explore the modal →](/docs/components/modal/)

## Searchbox

Another common search experience is the search dropdown, often placed in a navigation.

<div class="demo-box demo-box-attached">
<pagefind-searchbox instance="home-searchbox"></pagefind-searchbox>
</div>

```html
<pagefind-searchbox></pagefind-searchbox>
```

Since this is a unified component, it isn't built out of subcomponents. It still offers a lot of options, including custom result templates within the dropdown.

[Explore the searchbox →](/docs/components/searchbox/)

## Build Your Own

Include your search directly in the page with individual components, or build these into your own larger search UI. These components communicate with each other wherever they're placed:

<div class="demo-grid">
<div class="demo-box demo-box-input">
<pagefind-input instance="home-inline"></pagefind-input>
</div>
<div class="demo-box demo-box-summary">
<pagefind-summary default-message="👋" instance="home-inline"></pagefind-summary>
</div>
<div class="demo-box demo-box-hints">
<pagefind-keyboard-hints instance="home-inline"></pagefind-keyboard-hints>
</div>
<div class="demo-box demo-box-filter">
<pagefind-filter-dropdown filter="section" label="Section" instance="home-inline"></pagefind-filter-dropdown>
</div>
<div class="demo-box demo-box-results">
<div class="results-scroll">
<pagefind-results instance="home-inline"></pagefind-results>
</div>
</div>
</div>
<pagefind-config instance="home-inline"></pagefind-config>

[Explore the component system →](/docs/components/)
