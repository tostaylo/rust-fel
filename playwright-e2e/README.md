# Playwright E2E Tests

This directory contains a browser test harness for `rust-fel`.

It uses:

- a small wasm demo crate in `demo/`
- a static browser shell in `site/`
- Playwright tests in `tests/`

The goal is to verify real browser behavior, not just Rust unit tests or doctests. These tests exercise:

- wasm startup in the browser
- JavaScript module loading
- HTML rendering into the real DOM
- DOM updates after user interaction

## Prerequisites

You need these tools installed:

```sh
rustup target add wasm32-unknown-unknown
wasm-pack --version
node --version
npm --version
```

## One-time setup

From this directory:

```sh
cd playwright-e2e
npm install
npm run install:browsers
```

## Run the tests

Run the full browser suite with:

```sh
cd playwright-e2e
npm test
```

That command will:

1. Build the demo wasm app with `wasm-pack`.
2. Serve the static site from `site/`.
3. Run the Playwright tests in `tests/`.

## Run the tests in a visible browser

```sh
cd playwright-e2e
npm run test:headed
```

## Build only the wasm demo

If you want to verify the wasm build without running Playwright:

```sh
cd playwright-e2e
npm run build:wasm
```

The generated browser package is written to `site/pkg/`.

## Serve the site manually

If you want to inspect the page outside the Playwright runner:

```sh
cd playwright-e2e
npm run serve
```

Then open `http://127.0.0.1:4173` in a browser.

## Current test coverage

The current suite checks:

- the wasm app renders its initial title and counter state
- clicking increment updates the visible count
- clicking decrement updates the visible count

## File layout

- `demo/`: Rust wasm crate used only for browser integration testing
- `site/`: static HTML and JS shell that loads the wasm bundle
- `tests/`: Playwright specs
- `playwright.config.js`: Playwright runner configuration
- `package.json`: scripts for build, serve, and test
