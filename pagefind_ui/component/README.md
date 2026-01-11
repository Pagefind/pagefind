# @pagefind/component-ui

Modular web components for Pagefind search.

## Installation

### Via npm

```bash
npm install @pagefind/component-ui
```

Import the components and styles:

```javascript
import '@pagefind/component-ui';
import '@pagefind/component-ui/css';
```

If your bundler doesn't support the `/css` export, import the CSS file directly:

```javascript
import '@pagefind/component-ui/css/pagefind-component-ui.css';
```

### Via Pagefind's bundled files

After running Pagefind, include the generated files from your output directory:

```html
<link href="/pagefind/pagefind-component-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-component-ui.js" type="module"></script>
```

## Quick Start

Add a modal search to your site:

```html
<pagefind-modal-trigger></pagefind-modal-trigger>
<pagefind-modal></pagefind-modal>
```

Or use a searchbox dropdown:

```html
<pagefind-searchbox></pagefind-searchbox>
```

Or build your own layout with individual components:

```html
<pagefind-input></pagefind-input>
<pagefind-summary></pagefind-summary>
<pagefind-results></pagefind-results>
```

## Documentation

For full documentation, component references, styling guides, and examples, visit:

**https://ui.pagefind.app/**

## License

MIT
