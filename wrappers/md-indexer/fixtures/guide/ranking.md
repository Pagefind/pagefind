---
title: Ranking and Weights
description: How Pagefind scores and ranks search results
date: 2026-03-11
---

# Ranking and Weights

Pagefind scores each page using a BM25-based algorithm. Headings are weighted higher than body text by default, and every weight can be overridden.

## Element weights

These defaults are applied automatically during indexing.

| Element | Default weight |
|---------|---------------|
| `h1`    | 7             |
| `h2`    | 6             |
| `h3`    | 5             |
| `h4`    | 4             |
| `h5`    | 3             |
| `h6`    | 2             |
| All other elements (`p`, `li`, `td`, …) | 1 |

Weight 0 means the content is indexed (and can be searched) but contributes nothing to ranking.

### Overriding element weight

Add `data-pagefind-weight` to any HTML element to replace the default:

```html
<p data-pagefind-weight="3">This paragraph ranks like an h4.</p>
<aside data-pagefind-weight="0">Indexed but never boosted.</aside>
```

Accepted values are `0` to `10` (floats allowed). Values outside that range are clamped.

## Ranking algorithm

Pagefind uses a modified BM25 formula. The parameters below are tunable via the JavaScript API.

| Parameter | Default | Range | Effect |
|-----------|---------|-------|--------|
| `term_similarity` | `1.0` | 0 - ∞ | Bonus for terms whose length is close to the query term length. Higher = stronger preference for exact-length matches. |
| `page_length` | `0.75` | 0.0 - 1.0 | How much document length affects scoring. `0` = ignore length, `1` = full BM25 length normalisation. |
| `term_saturation` | `1.4` | 0.0 - 2.0 | Controls how quickly repeated occurrences stop adding score. Lower = saturation kicks in sooner. |
| `term_frequency` | `1.0` | 0.0 - 1.0 | Blend between raw term count (`0`) and BM25 frequency (`1`). |
| `diacritic_similarity` | `0.8` | 0 - ∞ | Extra boost when the query matches with diacritics (e.g. `café` vs `cafe`). |
| `meta_weights` (title) | `5.0` | any | Multiplier applied when a query term appears in a metadata field. Metadata coverage is squared so partial matches are penalised. |

### Configuring ranking parameters

Pass a `ranking` object when creating the Pagefind instance:

```js
const pagefind = await import("/pagefind/pagefind.js");

await pagefind.options({
    ranking: {
        termSimilarity: 1.0,
        pageLength: 0.75,
        termSaturation: 1.4,
        termFrequency: 1.0,
        diacriticSimilarity: 0.8,
    },
});
```

### Configuring metadata weights

Metadata weights control the boost given to matches in frontmatter fields (such as `title` or `description`):

```js
await pagefind.options({
    ranking: {
        pageLength: 0.75,
    },
    // not yet exposed via public API - set data-pagefind-meta manually for custom fields
});
```

The built-in `title` metadata field has a default weight of `5.0`. A page whose title contains the query term will score significantly higher than one where the term appears only in body text.
