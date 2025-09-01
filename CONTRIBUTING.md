# Contributing

## Core facets:

### [Rust] The Pagefind indexing binary
This lives in the `pagefind` directory, and houses the code for indexing a built static site.

### [JavaScript] The Pagefind search interface
This lives in the `pagefind_web_js` directory.

### [Rust] The Pagefind WebAssembly
This lives in `pagefind_web`, and is what performs the actual search actions in the browser.

### [JavaScript] The Pagefind UI modules
These are the node packages in `pagefind_ui`, which are both published to NPM and compiled into the indexing binary.

### [JavaScript + Python] The wrapper modules
These live in `wrappers`, and are what provide the `npx` and `pip` binary Pagefind runners, as well as the Node and Python bindings for Pagefind.

## Extras:

### [Hugo] The Pagefind documentation
This lives in `docs`, and is the static site generating the content at https://pagefind.app

### [Rust] The Pagefind stemmer
This lives in `pagefind_stem`, and it's unlikely you'll need to touch this.

***

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [NodeJS](https://nodejs.org/en/download)
- [just](https://github.com/casey/just#installation) - Command runner for development tasks

NB: Contributing right now is certainly easier on macOS/Linux systems, but there are no hard blockers for contributing from Windows.
To do so, you will need to dig into the justfile and run the applicable commands for Windows.
If you're interested in contributing Windows variants for the build scripts and justfile commands, that would be lovely!

## Quick Start

We use [just](https://github.com/casey/just) to manage development commands. To get started:

```bash
# Install all dependencies and tooling
just install

# Build everything
just build

# Run tests
just test
```

Run `just` to see all available commands.

## Building the project

The project has multiple components that need to be built in order. The easiest way is to use:

```bash
# Build all supporting packages first
just build-deps

# Then build the main Pagefind binary
just build-main

# Or build everything at once
just build
```

The build process:
1. Builds the JavaScript API bindings (`pagefind_web_js`)
2. Builds the UI packages (`pagefind_ui/default` and `pagefind_ui/modular`)
3. Builds the playground (`pagefind_playground`)
4. Builds the WebAssembly package for all supported languages (this takes a while!)
5. Builds the main Pagefind binary with release optimizations

After building, you'll have a final Pagefind binary at `target/release/pagefind`.

Note: Pagefind currently runs _very_ slowly in a debug build, so we always build with `--release` for better runtime performance.

## Test suite

```bash
# Run all tests
just test
```

This runs:
- Rust unit tests for the main package
- WebAssembly tests
- JavaScript tests
- Integration tests using Toolproof

For most changes, integration tests are preferred over unit tests. The integration test files are in `pagefind/integration_tests` and are written for [Toolproof](https://toolproof.app/).

## Development commands

```bash
# Start UI development server (default UI)
just dev-ui

# Start UI development server (modular UI)
just dev-ui-modular

# Format code
just fmt

# Lint everything
just lint

# Test with the documentation site
just test-docs
```

## Manually testing

For the UI packages, use `just dev-ui` or `just dev-ui-modular` to start a development server with hot reload.

To test the main package with the documentation site:

```bash
just test-docs
```

This will:
- Build the Pagefind documentation site
- Run your local Pagefind binary on it
- Start a server where you can test:
  - Indexing by your local build of the binary
  - Searching with your local build of the WebAssembly
  - UI rendered by your local build of the Default UI

## Further Notes

TODOS:
- Devise and document a nice way to manually test the npx wrapper behaviour
- Devise and document a nice way to manually test the Node package interface
- Provide a better path for contributing from Windows machines
