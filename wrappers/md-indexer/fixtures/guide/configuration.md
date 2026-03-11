---
title: Configuration
description: How to configure the md-indexer
date: 2026-03-11
slug: guide/config
---

# Configuration

The indexer accepts several options.

## Options

All options are optional except for `glob` or `vault`.

### glob

A fast-glob pattern to match markdown files.

### vault

A directory path. Shorthand for `glob: "vault/**/*.md"`.

### urlBase

A prefix added to all computed URLs.

## See also

For details on how search results are scored, see [[Ranking and Weights]].
