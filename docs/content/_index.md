---
title: Pagefind
nav_title: Home
weight: 1
---
Pagefind is a fully static search library that aims to perform well on large sites, while using as little of your users' bandwidth as possible, and without hosting any infrastructure.

Pagefind works with any static HTML output. All static site generators and website frameworks are supported, as long as the built HTML contains the content of the website. 

After indexing, Pagefind adds a static search bundle to your built files, which exposes a JavaScript search API that can be used anywhere on your site. Pagefind also provides prebuilt UI components that can be used with no configuration.

The goal of Pagefind is that websites with tens of thousands of pages should be searchable by someone in their browser, while consuming as little bandwidth as possible. Pagefind's search index is split into chunks, so that searching in the browser only ever needs to load a small subset of the search index. Pagefind can run a full-text search on a 10,000 page site with a total network payload under 300kB, including the Pagefind library itself. For most sites, this will be closer to 100kB.

## Highlights

- Prebuilt drop-in web components for search interfaces
- Zero-config support for multilingual websites
- Return results for each section of a page
- Tag results with custom metadata attributes
- Fine-grained configuration for the relevance of your content
- Sort results by relevance or by custom metadata
- Rich filtering engine for knowledge bases or faceted search
- Index anything (PDFs? JSON files? subtitles?) with the NodeJS and Python indexing libraries
- Search across multiple domains

## Pagefind demos

To see Pagefind on larger sites, try searching [MDN](https://mdn.pagefind.app/), [Godot](https://godot.pagefind.app/), or [XKCD](https://xkcd.pagefind.app/).

## Development Sponsor

Pagefind is an independent open source project, originally created and developed at [CloudCannon](https://cloudcannon.com/) 💙
