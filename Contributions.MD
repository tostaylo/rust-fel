# Contributing to rust-fel

This repository can still be built and tested locally, but it does not build on current Rust stable toolchains because it depends on `wasm-bindgen 0.2.67`.

At the time of writing, `cargo check` with Rust `1.93.1` fails with:

```text
older versions of the `wasm-bindgen` crate are incompatible with current versions of Rust; please update to `wasm-bindgen` v0.2.88
```

Until the dependency versions are updated, use an older Rust toolchain for local contribution work.

## Local setup

Install a compatible toolchain:

```sh
rustup toolchain install 1.56.1
rustup component add --toolchain 1.56.1 rustfmt clippy
```

You can either run commands with the toolchain explicitly:

```sh
cargo +1.56.1 check
cargo +1.56.1 test
```

Or set a local override while working in this repository:

```sh
rustup override set 1.56.1
cargo check
cargo test
```

If you set an override and want to remove it later:

```sh
rustup override unset
```

## Build

Use this command to confirm the crate still compiles:

```sh
cargo +1.56.1 check
```

This project is a library crate, so there is no standalone binary to run from this repository.

## Test

Run the full local test suite with:

```sh
cargo +1.56.1 test
```

This runs the unit tests in the crate and the doc tests that are enabled in the source. Browser testing is not required for this local workflow.

Verified locally with Rust `1.56.1`:

```text
3 unit tests passed
1 doc test passed
9 doc tests ignored
```

## Formatting and linting

The repository CI workflow checks formatting and Clippy in addition to build and test. To match that locally, run:

```sh
cargo +1.56.1 fmt --all -- --check
cargo +1.56.1 clippy -- -D warnings
```

If you want to apply formatting instead of checking it:

```sh
cargo +1.56.1 fmt --all
```

## Expected warning

The current test run emits one warning in `src/rsx.rs` for an unnecessary trailing semicolon. That warning does not prevent local tests from passing.

## Recommended contribution flow

1. Install Rust `1.56.1` if you do not already have it.
2. Run `cargo +1.56.1 check` before making changes.
3. Make your changes.
4. Run `cargo +1.56.1 test`.
5. Run `cargo +1.56.1 fmt --all -- --check`.
6. Run `cargo +1.56.1 clippy -- -D warnings`.

If you later update the `wasm-bindgen` and `web-sys` dependencies to modern versions, this document should be updated to remove the legacy toolchain requirement.
