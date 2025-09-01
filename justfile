#!/usr/bin/env just --justfile

# Development Justfile for Pagefind
# Run `just` to see available commands

set shell := ["bash", "-euc"]

# Default recipe - show available commands
default:
    @just --list

# Install all dependencies
install:
    @command -v rustup >/dev/null 2>&1 || { echo "Error: Rust is required but not installed. Please install Rust first."; exit 1; }
    @command -v node >/dev/null 2>&1 || { echo "Error: Node.js is required but not installed. Please install Node.js first."; exit 1; }
    @command -v python3 >/dev/null 2>&1 || { echo "Error: Python is required but not installed. Please install Python first."; exit 1; }
    cd pagefind_web_js && npm i
    cd pagefind_ui/default && npm i
    cd pagefind_ui/modular && npm i
    cd pagefind_playground && npm i
    cd wrappers/node && npm i
    cd wrappers/python && (python3 -m poetry install 2>/dev/null || (python3 -m pip install --user poetry && python3 -m poetry install))
    # Install Python linting tools - using python -m to avoid PATH issues
    python3 -m pip install --user mypy ruff==0.12.11
    rustup target add wasm32-unknown-unknown
    rustup toolchain install nightly
    rustup component add rust-src --toolchain nightly
    cargo install wasm-pack --version 0.10.3

# Build everything
build: build-deps build-main

# Build all supporting packages (run before building main package)
build-deps: build-wasm build-web-js build-ui build-playground

# Build WebAssembly (all language variants - this takes a while!)
build-wasm:
    cd pagefind_web && ./local_build.sh

# Build the JavaScript API bindings
build-web-js:
    cd pagefind_web_js && npm run build-coupled

# Build both UI packages
build-ui:
    cd pagefind_ui/default && npm run build
    cd pagefind_ui/modular && npm run build

# Build the playground
build-playground:
    cd pagefind_playground && npm run build

# Build the main Pagefind binary (release mode)
build-main:
    cd pagefind && cargo build --release --features extended

# Run integration tests
test:
    cd pagefind && cargo test --release --lib
    cd pagefind && cargo test --release --lib --features extended
    cd pagefind_web && cargo test
    cd pagefind_web_js && npm test
    npx -y toolproof@latest

# Format code
fmt:
    cargo +nightly fmt
    cd wrappers/python && python3 -m ruff format

# Lint everything
lint:
    cargo clippy --all
    cd wrappers/python && python3 -m mypy src scripts
    cd wrappers/python && python3 -m ruff check

# Start UI development server (default UI)
dev-ui:
    cd pagefind_ui/default && npm start

# Start UI development server (modular UI)
dev-ui-modular:
    cd pagefind_ui/modular && npm start

# Test with the documentation site
test-docs:
    @command -v hugo >/dev/null 2>&1 || { echo "Error: Hugo is required but not installed. Please install Hugo first."; exit 1; }
    cd docs && rm -rf ./public
    cd docs && npm i
    cd docs && hugo
    ./target/release/pagefind -s docs/public --serve
