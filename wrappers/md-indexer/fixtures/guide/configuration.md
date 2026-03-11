---
title: Configuration
description: How to configure the md-indexer
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
