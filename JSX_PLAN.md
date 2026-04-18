# JSX-Style Macro Design

## Goal

Replace verbose manual tree construction such as:

```rust
Element::new("main".to_owned(), {
    let mut props = Props {
        children: Some(vec![header, status, controls]),
        ..Default::default()
    };
    props.set_attribute("id", id);
    props.set_attribute("class", "demo-app");
    props
})
```

with a compile-time macro that feels closer to JSX:

```rust
rsx! {
    <main id={id} class="demo-app">
        <Header data-cy="demo-title" />
        <span data-cy="count-value">{count}</span>
        <div class="controls">
            <button type="button" data-cy="increment" on_click={increment}>
                {"Increment"}
            </button>
            <button type="button" data-cy="decrement" on_click={decrement}>
                {"Decrement"}
            </button>
        </div>
    </main>
}
```

## Recommendation

The best fit for this repository is a compile-time procedural macro in a companion crate.

Recommended shape:

- Keep `rust-fel` as the runtime crate.
- Add a new `rust-fel-macro` crate with `proc-macro = true`.
- Re-export the user-facing macro from `rust-fel`.

This follows the same overall architecture used by mature Rust UI libraries:

- Yew uses `html!`.
- Leptos uses `view!`.
- Dioxus uses `rsx!`.

All three solve the same problem at compile time instead of parsing HTML-like strings at runtime.

## Why a Proc Macro

This is the right direction for `rust-fel` for five reasons:

1. It gives real JSX-like syntax instead of string-based pseudo-HTML.
2. Errors happen at compile time with spans, not at runtime in the browser.
3. Events and expressions can remain typed Rust values.
4. It preserves your current runtime model of `Element` plus `Props`.
5. It avoids pushing more complexity into the current hand-written runtime parser in `src/rsx.rs`.

The existing parser in `src/rsx.rs` is useful as a lightweight helper, but it is not the right foundation for a primary authoring experience. It works on strings, not Rust tokens, so it cannot support a good component model, event closures, or strong diagnostics.

## Parser Choice

Use `rstml` for parsing the JSX-like syntax.

Recommended macro dependencies:

- `syn`
- `quote`
- `proc-macro2`
- `rstml`

Reasoning:

- `syn` and `quote` are the standard Rust proc-macro tools.
- `rstml` is designed for HTML/RSX-style token parsing and is already used in this problem space by Leptos.
- Writing a custom angle-bracket parser directly on top of `syn` would add avoidable parser work.

## Proposed User Syntax

### HTML Elements

Lowercase tags map to plain DOM elements.

```rust
rsx! {
    <main id={id} class="demo-app">
        <span data-cy="count-value">{count}</span>
    </main>
}
```

### Function Components

PascalCase tags map to Rust functions that return `Element`.

```rust
fn Status(props: StatusProps) -> Element {
    rsx! {
        <span data-cy="count-value">{props.count}</span>
    }
}

rsx! {
    <Status count={count} />
}
```

### Nested Children

Children should flow naturally through the syntax:

```rust
rsx! {
    <Panel title="Counter">
        <span>{count}</span>
        <button on_click={increment}>{"Increment"}</button>
    </Panel>
}
```

### Expressions

Rust expressions should be supported in braces.

```rust
rsx! {
    <span>{count.to_string()}</span>
}
```

### Self-Closing Tags

```rust
rsx! {
    <Header />
    <button type="button" />
}
```

## Component Model Choice

For the first implementation, support function components before trait-based stateful components.

Recommended phase 1 component form:

```rust
fn Button(props: ButtonProps) -> Element
```

Why this is the right first step:

- It matches the macro model used by Yew and Leptos more naturally than the current `Component` trait.
- It lowers directly to `Element` without inventing a second runtime node kind.
- It keeps the first macro implementation small enough to finish.

The current `Component` trait should remain for app-level stateful components and `App::mount(...)`.

### Why not support the current `Component` trait inline first

The existing trait is oriented around stateful rendering and `re_render(...)`, not around nested compile-time component invocation in an element tree. Inline `<MyStatefulComponent />` support would force design decisions about:

- ownership of state
- lifecycle
- rerender boundaries
- nested component identity

That is a larger runtime redesign than the syntax work itself.

Phase 1 should avoid that.

## Runtime Lowering Model

The macro should lower into ordinary `Element` plus `Props` construction.

### Element Lowering

This input:

```rust
rsx! {
    <main id={id} class="demo-app">
        <span>{count}</span>
    </main>
}
```

should expand to something conceptually like:

```rust
{
    let child_0 = {
        let mut props = rust_fel::Props {
            text: Some(count.to_string()),
            ..Default::default()
        };
        rust_fel::Element::new("span".to_owned(), props)
    };

    let mut props = rust_fel::Props {
        children: Some(vec![child_0]),
        ..Default::default()
    };
    props.set_attribute("id", id);
    props.set_attribute("class", "demo-app");
    rust_fel::Element::new("main".to_owned(), props)
}
```

The exact generated tokens can be improved later, but the semantic target should remain the existing runtime API.

### Text Lowering

Text children should lower into your current `TEXT_ELEMENT` representation or an internal helper that creates it.

Recommendation:

- add a small hidden runtime helper such as `__private::text(value)`
- do not emit raw `TEXT_ELEMENT` strings directly from every macro expansion site

This keeps generated code smaller and gives one place to evolve text-node behavior.

### Attribute Lowering

Attributes should map into `Props` as follows:

- string-like attributes go into `props.set_attribute(name, value)`
- `on_click={handler}` maps to `props.on_click = Some(handler)`
- `children` are synthesized from nested nodes, not provided as a plain HTML attribute

This keeps your generic attribute refactor intact.

## Syntax Rules

### Tag Kinds

- lowercase identifier: HTML element
- identifier path with uppercase last segment: component
- path-qualified component names should be allowed later, for example `<ui::Button />`

### Supported in the first version

- one root node per macro call
- nested children
- self-closing tags
- string literal attributes
- `{expr}` attribute values
- `{expr}` child expressions
- text nodes
- `on_click={...}` event binding

### Defer until later

- fragments
- `if` and `for` control-flow sugar inside the macro
- prop spreads such as `..props`
- keyed nodes
- special directives
- support for the current trait-based stateful `Component` values as inline nodes

Keeping phase 1 tight matters more than feature completeness.

## Proposed Public API

### Re-export

Expose the macro from `rust-fel`:

```rust
pub use rust_fel_macro::rsx;
```

### Optional helpers

Add internal helpers for macro output stability:

- `rust_fel::__private::element(tag, props)`
- `rust_fel::__private::text(value)`
- `rust_fel::__private::set_attr(props, name, value)`

These helpers should be `#[doc(hidden)]` and not part of the stable public design story.

## Cargo Layout

Because a normal library crate cannot also be a proc-macro crate, add a sibling crate.

Recommended repo shape:

```text
rust-fel/
  Cargo.toml
  src/
  rust-fel-macro/
    Cargo.toml
    src/lib.rs
```

The root `Cargo.toml` can remain the package manifest for `rust-fel` and also define a workspace.

Recommended top-level shape:

```toml
[workspace]
members = [".", "rust-fel-macro"]

[package]
name = "rust-fel"
...
```

And in `rust-fel`:

```toml
[dependencies]
rust-fel-macro = { path = "rust-fel-macro" }
```

The macro crate should not depend on the runtime crate at runtime. It only needs to generate tokens that refer to `rust_fel`.

## Implementation Phases

### Phase 1: HTML Elements Only

Goal:

- `rsx! { <main class="demo-app"><span>{count}</span></main> }`

Scope:

- lowercase tags only
- text nodes
- generic attributes
- `on_click`
- nested children

This phase proves the parser and code generator without introducing a component model yet.

### Phase 2: Function Components

Goal:

- `<Header />`
- `<Status count={count} />`
- `<Panel><span>{count}</span></Panel>`

Scope:

- PascalCase tags call Rust functions
- children are packed into a component props struct
- minimal typed props story

Recommendation:

- start with manually written props structs
- add a derive macro later if needed

### Phase 3: Ergonomic Features

Potential additions:

- fragments
- `..props` spread support
- shorthand attributes
- control flow inside macro
- path-qualified components
- better diagnostics for invalid tag nesting and unsupported props

### Phase 4: Revisit Stateful Components

Only after the macro is stable should the crate consider deeper integration with the existing `Component` trait.

That work likely needs runtime design changes, not just syntax changes.

## Suggested Props Story for Function Components

The cleanest first model is an explicit props struct:

```rust
struct PanelProps {
    title: String,
    children: Vec<Element>,
}

fn Panel(props: PanelProps) -> Element {
    rsx! {
        <section class="panel">
            <h2>{props.title}</h2>
        </section>
    }
}
```

Then the macro lowers:

```rust
rsx! {
    <Panel title="Counter">
        <span>{count}</span>
    </Panel>
}
```

into something conceptually like:

```rust
Panel(PanelProps {
    title: "Counter".to_owned(),
    children: vec![...],
})
```

This is simple, explicit, and easy to debug.

## Recommended Testing Strategy

Use three layers of tests.

### Unit Tests in the Macro Crate

Test parser-to-token generation for:

- lowercase elements
- nested text
- attribute lowering
- event lowering
- self-closing tags
- invalid syntax errors

### `trybuild` Compile Tests

Add compile-pass and compile-fail coverage for macro errors and diagnostics.

This is important because proc macros succeed or fail at compile time, not at runtime.

### Runtime Integration Tests in `rust-fel`

Use the existing Rust tests and Playwright harness to ensure macro output behaves the same as hand-written `Element::new(...)` trees.

## Initial Non-Goals

To keep the project scoped correctly, the first implementation should not try to solve:

- React-style reconciliation keys
- a full typed HTML attribute model
- nested trait-component state management
- server-side rendering
- hot reload
- full JSX parity

The first goal is only to make authoring tree construction ergonomic.

## Concrete First Milestone

The first milestone should be to rewrite the Playwright demo from manual `Element::new(...)` calls to:

```rust
rsx! {
    <main id={id} class="demo-app">
        <h1 data-cy="demo-title">{"rust-fel Playwright demo"}</h1>
        <span data-cy="count-value">{count}</span>
        <div class="controls">
            <button type="button" data-cy="increment" on_click={increment}>
                {"Increment"}
            </button>
            <button type="button" data-cy="decrement" on_click={decrement}>
                {"Decrement"}
            </button>
        </div>
    </main>
}
```

If that compiles, renders, and passes the existing Playwright test suite, the macro direction is validated.

## Summary

The best approach for `rust-fel` is:

1. Add a companion proc-macro crate.
2. Parse JSX-like syntax at compile time with `rstml`.
3. Lower macro output into the existing `Element` plus `Props` runtime.
4. Support lowercase elements first.
5. Add function components next.
6. Delay integration with the current stateful `Component` trait until later.

That path gives the syntax improvement you want without forcing a full runtime rewrite up front.
