# Modernization Plan

- [x] Snapshot current baseline
  - Verified the legacy dependency stack builds and tests with Rust 1.56.1.
  - Confirmed current stable fails because `wasm-bindgen 0.2.67` is too old.
- [x] Upgrade wasm dependencies
  - Updated `wasm-bindgen` to `0.2.117`.
  - Updated `web-sys` to `0.3.94`.
  - Refreshed `Cargo.lock`.
- [x] Modernize Rust edition
  - Moved the crate from edition 2018 to edition 2021.
  - Left `rust-version` unset because an MSRV policy was not explicitly established during this pass.
- [x] Fix compiler and clippy fallout
  - Removed legacy warnings and Clippy violations in the parser, props, and render paths.
  - Confirmed the crate compiles cleanly on current stable.
- [x] Refresh CI workflow
  - Replaced the outdated `actions-rs` workflow with current GitHub Actions steps using `actions/checkout`, `dtolnay/rust-toolchain`, and `Swatinem/rust-cache`.
- [x] Update contributor docs
  - Rewrote the local setup guidance around current stable.
  - Removed the legacy toolchain workaround.
- [x] Verify stable build matrix
  - Passed `cargo check`, `cargo test`, `cargo fmt --all -- --check`, and `cargo clippy -- -D warnings` on current stable.

# Playwright E2E Plan

- [x] Define the browser test harness
  - Added a dedicated `playwright-e2e/` subdirectory for browser integration testing instead of mixing Node tooling into the Rust crate root.
  - Used that directory to host Playwright config, package metadata, and a tiny static app shell.

- [x] Add a runnable wasm demo target
  - Created a minimal Rust demo app that depends on this crate by path and mounts a predictable DOM tree into a known root element.
  - Exposed observable behavior through text content, attributes, and button interactions so browser assertions can verify real rendering and updates.

- [x] Choose the wasm build and serve workflow
  - Used a simple local flow based on `wasm-pack build demo --target web` plus a static `http-server` instance managed by Playwright.
  - Kept the tooling local to the test directory so contributors can run it without an existing frontend stack.

- [x] Add Playwright project structure
  - Added `package.json`, `package-lock.json`, Playwright configuration, and test scripts in the new test directory.
  - Added setup steps for browser installation and local server orchestration required by the tests.

- [x] Write end-to-end browser assertions
  - Verified the initial HTML rendered by the wasm app matches expectations.
  - Verified interaction flows such as clicking a button update real DOM text or attributes.
  - Kept assertions user-visible so the tests reflect actual browser behavior.

- [x] Integrate the workflow into repo docs
  - Documented how to install Node dependencies, build the wasm demo, and run Playwright locally.
  - Added a short explanation of what these tests cover that unit tests and doctests do not.

- [ ] Decide whether to wire browser tests into CI
  - Kept the first pass runnable locally.
  - A separate CI job for Playwright and wasm integration tests is still optional future work.
