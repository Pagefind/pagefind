---
title: "Custom Components"
nav_title: "Custom Components"
nav_section: Customization
weight: 92
---

You can create your own components that integrate with the Pagefind Component UI system. Custom components can listen to search events, trigger searches, and coordinate with the built-in components.

Before deciding to build a custom component, though, do check out how much you can customise the official components. For example, the [modal component](/components/modal/) can have its internal structure replaced, and both the [results component](/components/results/) and the [searchbox component](/components/searchbox/) can be supplied custom templates for each result.

## Basic Structure

Custom components connect to Pagefind through an Instance. The instance coordinates all components that share the same `instance` attribute, handling search state, filters, and events.

```javascript
import { getInstanceManager } from '@pagefind/component-ui';

// Get the instance (creates one if it doesn't exist)
const manager = getInstanceManager();
const instance = manager.getInstance('default');

// Listen to search events
instance.on('results', (searchResult) => {
  console.log(`Found ${searchResult.results.length} results`);
});

// Trigger a search
instance.triggerSearch('hello world');
```

## Available Events

Subscribe to events using `instance.on(event, callback)`:

| Event | Callback Arguments | Description |
|-------|-------------------|-------------|
| `search` | `(term, filters)` | Fired when a search is triggered |
| `loading` | none | Fired when search starts loading |
| `results` | `(searchResult)` | Fired when results are ready |
| `filters` | `({ available, total })` | Fired when filter counts update |
| `error` | `(error)` | Fired on search errors |
| `translations` | `(translations, direction)` | Fired when language changes |

## Triggering Searches

```javascript
// Search with current filters
instance.triggerSearch('search term');

// Search with specific filters
instance.triggerSearchWithFilters('search term', {
  category: ['blog', 'docs']
});

// Update just the filters (re-runs current search)
instance.triggerFilters({ category: ['blog'] });

// Update a single filter
instance.triggerFilter('category', ['blog']);
```

## Example: Custom Results Counter

Here's a simple custom element that displays a result count:

```javascript
import { getInstanceManager } from '@pagefind/component-ui';

class MyResultsCounter extends HTMLElement {
  connectedCallback() {
    const instanceName = this.getAttribute('instance') || 'default';
    const manager = getInstanceManager();
    const instance = manager.getInstance(instanceName);

    instance.on('results', (searchResult) => {
      const count = searchResult.results?.length ?? 0;
      this.textContent = `${count} results found`;
    });

    instance.on('loading', () => {
      this.textContent = 'Searching...';
    });
  }
}

customElements.define('my-results-counter', MyResultsCounter);
```

Usage:
```html
<pagefind-input></pagefind-input>
<my-results-counter></my-results-counter>
<pagefind-results></pagefind-results>
```

## Example: Custom Search Trigger

A button that searches for a preset term:

```javascript
import { getInstanceManager } from '@pagefind/component-ui';

class QuickSearchButton extends HTMLElement {
  connectedCallback() {
    const term = this.getAttribute('term') || '';
    const instanceName = this.getAttribute('instance') || 'default';

    this.addEventListener('click', () => {
      const instance = getInstanceManager().getInstance(instanceName);
      instance.triggerSearch(term);
    });
  }
}

customElements.define('quick-search-button', QuickSearchButton);
```

Usage:
```html
<quick-search-button term="getting started">Quick Start</quick-search-button>
```

## Working with Results Data

The `results` event provides a `PagefindSearchResult` object:

```javascript
instance.on('results', async (searchResult) => {
  // searchResult.results is an array of raw results
  for (const rawResult of searchResult.results) {
    // Load the full data for this result
    const data = await rawResult.data();

    console.log(data.url);        // Page URL
    console.log(data.meta.title); // Page title
    console.log(data.excerpt);    // Search excerpt with <mark> tags
    console.log(data.sub_results); // Matching sections within the page
  }
});
```

### Sub-results Helpers

The instance provides a helper for working with sub-results:

```javascript
// Get sub-results for display, excluding the root and limiting count
const subResults = instance.getDisplaySubResults(resultData, 3);
```

## Accessing Translations

Use the instance's translation system for internationalized text. See the [English translation file](https://github.com/CloudCannon/pagefind/blob/main/pagefind_ui/translations/en.json) for available keys.

```javascript
// Get a translated string
const text = instance.translate('zero_results', {
  SEARCH_TERM: 'hello'
});

// Override the detected language
instance.setLanguage('fr');

// Add custom translation overrides
instance.setTranslations({
  'placeholder': 'Search documentation...',
  'zero_results': 'Nothing found for [SEARCH_TERM]'
});
```

## Announcing to Screen Readers

For accessible custom components, use the instance's announcer:

```javascript
// Announce using a translation key
instance.announce('zero_results', { SEARCH_TERM: term }, 'assertive');

// Announce raw text
instance.announceRaw('5 new results loaded', 'polite');
```

## Registering Components

Most components should register with the instance. This enables features like keyboard navigation between components and ARIA reconciliation.

```javascript
class MyCustomInput extends HTMLElement {
  connectedCallback() {
    const instance = getInstanceManager().getInstance('default');

    // Register as an input component with capabilities
    instance.registerInput(this, {
      keyboardNavigation: true  // Participates in arrow-key navigation
    });
  }
}
```

Registration methods:
- `registerInput(component, capabilities)` — Search inputs
- `registerResults(component, capabilities)` — Results displays
- `registerSummary(component, capabilities)` — Result summaries
- `registerFilter(component, capabilities)` — Filter controls
- `registerSort(component, capabilities)` — Sort controls
- `registerUtility(component, subtype, capabilities)` — Utility components (e.g., keyboard hints)

## Capabilities

Capabilities tell the instance what your component can do:

| Capability | Description |
|------------|-------------|
| `keyboardNavigation` | Component participates in arrow-key focus management. Inputs with this capability are targets for `focusPreviousInput()`. Results with this capability are targets for `focusNextResults()`. |
| `announcements` | Component handles its own screen reader announcements. When set, the instance won't make fallback announcements for search results. |

```javascript
instance.registerResults(this, {
  keyboardNavigation: true,
  announcements: true
});
```

Query components by capability:
```javascript
// Get all inputs that support keyboard navigation
const inputs = instance.getInputs('keyboardNavigation');

// Get all results components
const results = instance.getResults();
```

## Keyboard Shortcuts

Register keyboard shortcuts that appear in `<pagefind-keyboard-hints>`:

```javascript
// Register a shortcut (appears in hints UI)
instance.registerShortcut(
  { label: '↓', description: 'navigate' },
  this  // owner element
);

// Remove a specific shortcut
instance.deregisterShortcut('↓', this);

// Remove all shortcuts from this component
instance.deregisterAllShortcuts(this);

// Get all active shortcuts
const shortcuts = instance.getActiveShortcuts();
```

## Focus Management

The instance provides helpers for keyboard navigation between components. These functions find components based on DOM tab order relative to the passed element, considering only components with the `keyboardNavigation` capability.

```javascript
// Find the next results component (in tab order) after this element,
// then focus its first result link
instance.focusNextResults(this);

// Find the previous input component (in tab order) before this element,
// then focus it
instance.focusPreviousInput(this);

// Find the previous input, append a character to its value,
// focus it, and dispatch an input event (triggers search)
instance.focusInputAndType(this, 'a');

// Find the previous input, remove the last character from its value,
// focus it, and dispatch an input event (triggers search)
instance.focusInputAndDelete(this);
```

For example, these are used by `<pagefind-results>` to allow typing while focused on a result link — pressing a letter or backspace redirects to the input. By registering your own components as an input or a result type, you can participate in this behavior.

## ARIA Reconciliation

When your component's ARIA relationships need updating (e.g., after rendering), call `reconcileAria`:

```javascript
// Trigger ARIA reconciliation on all components
instance.reconcileAria();
```

Implement a `reconcileAria()` method on your component to respond:

```javascript
class MyComponent extends HTMLElement {
  reconcileAria() {
    // Update aria-controls, aria-labelledby, etc.
  }
}
```

For example, this is used by `<pagefind-modal>` and `<pagefind-modal-trigger>` to ensure their ARIA relationships are correct, regardless of what order they instantiate in.

## Accessing Instance State

Read current search state directly from the instance:

```javascript
instance.searchTerm        // Current search term
instance.searchFilters     // Current filter selections
instance.searchResult      // Last search result
instance.availableFilters  // Filter counts for current search
instance.totalFilters      // Filter counts for all content
instance.direction         // 'ltr' or 'rtl'
instance.faceted           // Whether faceted mode is enabled
```

## Generating Unique IDs

Use the instance's ID generator for unique, collision-free IDs:

```javascript
const id = instance.generateId('my-prefix');
// Returns something like: "my-prefix-abc-def"
```

These are predominantly used for elements that need to reference each other by ID for ARIA relationships.

## Preloading

Trigger the Pagefind bundle to load before the user searches:

```javascript
await instance.triggerLoad();
```

## Using Without npm

If you're not using a bundler, access the instance manager from the global:

```html
<script type="module">
  // After pagefind-component-ui.js loads, PagefindComponents is available
  const manager = window.PagefindComponents.getInstanceManager();
  const instance = manager.getInstance('default');

  instance.on('results', (results) => {
    console.log('Got results:', results);
  });
</script>
```
