# Contributing to rust-fel

This repository now builds, tests, formats, and passes Clippy on current Rust stable toolchains.

## Local setup

Install Rust stable and the standard formatting and linting components:

```sh
rustup toolchain install stable
rustup component add rustfmt clippy
```

If you want this repository to always use stable while you work in it, set a local override:

```sh
rustup override set stable
```

If you set an override and want to remove it later:

```sh
rustup override unset
```

## Build

Use this command to confirm the crate still compiles:

```sh
cargo check
```

This project is a library crate, so there is no standalone binary to run from this repository.

## Test

From the repository root, run the full local test suite with:

```sh
cargo test
```

This runs the unit tests in the crate and the doc tests that are enabled in the source. Browser testing is not required for this local workflow.

If you want to run only the crate's unit tests:

```sh
cargo test --lib
```

If you want to run only the documentation tests:

```sh
cargo test --doc
```

Verified locally on current stable:

```text
3 unit tests passed
10 doc tests passed
0 doc tests ignored
```

## Browser E2E tests

This repository also includes a Playwright-based browser harness under `playwright-e2e/` for testing real HTML, JavaScript module loading, wasm startup, and DOM updates.

Prerequisites:

```sh
rustup target add wasm32-unknown-unknown
command -v wasm-pack
node --version
npm --version
```

From the repository root, install the browser test dependencies:

```sh
cd playwright-e2e
npm install
npm run install:browsers
```

Run the full browser end-to-end suite with:

```sh
cd playwright-e2e
npm test
```

That command:

1. Builds the demo wasm app in `playwright-e2e/demo`.
2. Serves the static test page from `playwright-e2e/site`.
3. Runs Playwright assertions against the real browser DOM.

If you want to watch the test run in a visible browser:

```sh
cd playwright-e2e
npm run test:headed
```

## Formatting and linting

The repository CI workflow checks formatting and Clippy in addition to build and test. To match that locally, run:

```sh
cargo fmt --all -- --check
cargo clippy -- -D warnings
```

If you want to apply formatting instead of checking it:

```sh
cargo fmt --all
```

## Publish to crates.io

This repository is already configured as a publishable crate, and it currently passes both:

```sh
cargo package
cargo publish --dry-run
```

One important constraint: `rust-fel 0.1.2` already exists on crates.io, so you cannot publish this repository again with the current version number.

Before publishing a new release:

1. Update the version in `Cargo.toml`.
2. Keep the `html_root_url` in `src/lib.rs` in sync with that same version.
3. Run the local quality checks:

```sh
cargo check
cargo test
cargo fmt --all -- --check
cargo clippy -- -D warnings
cd playwright-e2e && npm test
cd ..
```

4. Verify the package contents and publish flow:

```sh
cargo package
cargo publish --dry-run
```

5. Authenticate with crates.io if needed:

```sh
cargo login
```

6. Publish the new version:

```sh
cargo publish
```

Useful notes:

- `cargo publish --dry-run` already succeeds locally for this repository.
- `cargo publish` will fail if the version in `Cargo.toml` has already been published.
- `README.md` is included as the crate readme because `Cargo.toml` sets `readme = "./README.md"`.
- `LICENSE.txt` is included in the package and matches the crate's `license = "MIT"` metadata.

## Recommended contribution flow

1. Install Rust stable if you do not already have it.
2. Run `cargo check` before making changes.
3. Make your changes.
4. Run `cargo test`.
5. Run `cargo fmt --all -- --check`.
6. Run `cargo clippy -- -D warnings`.
7. If your changes affect DOM behavior, run `cd playwright-e2e && npm test`.
