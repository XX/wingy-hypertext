# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

`wingy-hypertext` is a Rust component library that renders [Web Awesome](https://webawesome.com/) (`wa-*` class) styled HTML server-side via the [`hypertext`](https://github.com/vidhanio/hypertext) crate. Components are typed Rust structs implementing `hypertext::Renderable`. The repo also ships an `example-client` (compiled to WASM) and an `example-server` (static file server) that together form a live component gallery driven by htmx — but with htmx requests intercepted and answered by the WASM module instead of a backend.

## Workspace layout

- `crates/lib` (`wingy-hypertext`) — the component library. This is the product.
- `crates/macros` (`wingy-hypertext-macros`) — proc-macros (`#[derive(Props)]`, `#[derive(DynRenderable)]`, `#[const_str(...)]`, `htmx_rsx!`) used by the library.
- `crates/web` (`wingy-hypertext-web`) — Rust/WASM port of the gallery's client-side scripts (web-sys/js-sys); modules mirror `webassets/js` one-to-one. Consumed by `example-client`.
- `examples/client` (`example-client`) — `cdylib`/`rlib` compiled to `wasm32-unknown-unknown`; renders the gallery pages.
- `examples/server` (`example-server`) — serves `target/web` over `127.0.0.1:9080`.
- `webassets/` — shared CSS (`webassets/style/`) and JS (`webassets/js/`) for components/layouts; copied into `target/web` at build time.
- `examples/client/webassets/` — gallery-specific static assets (`index.html`, `main.js`, vendored htmx/highlight.js); also copied into `target/web`.
- `tmp/` — scratch (git-ignored), including `client_old/` legacy code. Not part of the build.

## Commands

The build is orchestrated by [`cargo-make`](https://github.com/sagiegurari/cargo-make) (`Makefile.toml`), because building the gallery requires compiling WASM, running `wasm-bindgen`, and copying static assets. Plain `cargo` works for the library and tests.

- `cargo make run` — full build of the example (WASM client + wasm-bindgen + static copy) then runs the server at http://127.0.0.1:9080.
- `cargo make watch` — rebuild the client on changes under `examples/client` (uses `watchexec`).
- `cargo make client` — build + wasm-bindgen the client and copy static files only (no server).
- Add `-p debug` to any task for an unoptimized debug build, e.g. `cargo make run -p debug` (release/`opt-level=s` is the default).

Tooling required for the example build: the `wasm32-unknown-unknown` target, `wasm-bindgen-cli`, `wasm-opt` (binaryen; release builds only), `watchexec` (for `watch`), and `cargo-make`.

### Lint & test (matches CI in `.github/workflows/lint-test.yml`)

- `cargo +nightly fmt --check` — formatting (config in `.rustfmt.toml`; nightly is required for the unstable options; `max_width = 120`).
- `cargo clippy --all-targets -- -D warnings` — lints; warnings are errors.
- `cargo test` — runs the unit tests (rendering snapshot-style assertions in `crates/lib/src/tests/`).
- Run a single test: `cargo test -p wingy-hypertext <test_name>`.
- `cargo make lint-test` runs all three in sequence.

## Architecture

### Component pattern

Every component/layout is a struct that follows the same recipe (see `crates/lib/src/components/button.rs` as the canonical example):

1. Derives `Default, AsRef, AsMut, Props`.
2. Is annotated `#[const_str(CLASS = BUTTON)]` to attach the base CSS class as an associated `const CLASS` — the value is a `&str` constant from `crate::class` (see Conventions), not a string literal.
3. Is annotated `#[props(builder)]` to get `builder()`/`build()`.
4. Embeds shared sub-structs as fields tagged `#[as_ref] #[as_mut]`: `CommonAttrs` (id/classes/styles), `Link` (href/target/...), `Action` (data-action/data-args). These `AsRef`/`AsMut` impls are what make the blanket trait impls (below) apply to the component.
5. Has an `Option<R: Renderable>` `children` field tagged `#[prop(convert)]`.
6. Derives `DynRenderable`, which generates the `Renderable` impl as a thin wrapper delegating to an inherent `fn render_to(&self, buffer, children: Option<&dyn Renderable>)` holding the actual render body (building the class line with `self.class_line_with(&[Self::CLASS, ...])` and emitting via `rsx! { ... }`). Every field typed `Option<P>` with `P` a type param is erased to `Option<&dyn Renderable>` and passed after the buffer in declaration order (e.g. `Callout` gets `icon` and `children`); fields marked `#[skip_render]` are excluded, and without erased fields the call is delegated as is. The delegate name is overridable with `#[render_to(name)]` on the struct. The body must render the passed arguments and never touch the erased fields through `self`, so LLVM can merge the per-child-type instantiations into one copy — critical for WASM binary size.

### Trait-based fluent setters

Capabilities are mixed in via traits with blanket impls keyed off `AsRef`/`AsMut` of the shared sub-structs:

- `CommonAttributeSetters`/`CommonAttributeGetters` (`attributes.rs`) — any `T: AsMut<CommonAttrs>` gets `.id()`, `.class()`, `.style()`, etc.
- `LinkSetters` (`link.rs`) — any `T: AsMut<Link>` gets `.href()`, `.target()`, `.download()`, `.rel()`.
- `ActionSetters` (`action.rs`) — any `T: AsMut<Action>` gets `.action()`, `.args()`.
- `VariantSetters`/`AppearanceSetters` (`variant.rs`, `appearance.rs`) — opt-in by implementing the marker traits `UseVariant`/`UseAppearance`; the enums (`Variant`, `Appearance`) derive strum `IntoStaticStr` with kebab-case serialization and expose `const` string forms via `#[strum(const_into_str)]`.
- `HtmxSetters` (`htmx.rs`) — any `T: AsMut<Htmx>` gets `.hx_get(..)`, `.hx_target(..)`, etc. `Htmx` stores htmx attributes as a *collection* (not a field per attribute) and exposes inherent accessors `hx_get(&self) -> Option<&str>` for reading them back.

### HTMX attributes

To attach htmx attributes to elements without hand-listing all ~40, a component's `render_to` uses `htmx_rsx!` instead of `rsx!` and writes a single `htmx=[self.htmx]` pseudo-attribute, which the macro expands into `hx-*=[(self.htmx).hx_*()]` for every attribute before forwarding to `hypertext::rsx!` (see `button.rs`). The component needs `HtmxAttributes` (from `hypertext::prelude`) in scope for rsx to accept the generated `hx-*` names. The canonical attribute list is **duplicated** and must stay in sync: the `htmx_attrs!` declarative-macro invocation in `crates/lib/src/htmx.rs` generates the `Htmx` type/accessors/setters, and `HTMX_ATTRS` in `crates/macros/src/htmx.rs` drives the `htmx_rsx!` expansion. `hx-on` (namespaced) and the deprecated `hx-vars` are intentionally excluded.

The `#[derive(Props)]` macro (`crates/macros/src/derive.rs`) generates per-field `.field(self, value) -> Self` (chainable) and `set_field(&mut self) -> &mut Self` setters. Field attributes change codegen: `#[prop(into)]` accepts `impl Into<T>`; `#[prop(impl_from)]` generates a `From<T>` impl for the whole struct; `#[prop(convert)]` produces a setter that changes the generic type parameter (used for `children: Option<R>` so the renderable child type is inferred from the call site); `#[prop(skip)]` omits setters. `Option<T>` fields auto-wrap the assigned value in `Some(...)`.

### Example gallery runtime (the unusual part)

The gallery has **no real backend**. Flow:

1. `examples/client/webassets/main.js` boots the WASM module, calls `wasm.render_root(path)` to render the full page shell, and inserts it into `#root`.
2. `examples/client/webassets/vendor/htmx/client_patch.js` monkey-patches `XMLHttpRequest` so htmx requests (anything not starting with `/api/`) are intercepted and answered synchronously by `wasm.request(url)` — see `examples/client/src/lib.rs`, which routes the path to the matching component overview (`badge`/`button`/`copy-button`/`input`).
3. htmx swaps the returned fragment into `.main-content`; `htmx:afterSettle` re-runs highlight.js and re-inits page/scroll JS.

So changing gallery content means editing the Rust in `examples/client/src/` (and rebuilding the WASM), not adding server routes. The server (`examples/server/src/main.rs`) only serves static files from `target/web`.

### Client-side behavior in Rust

The interactive client-side behavior (copy button, code-example expand/resize, action dispatch, page/scroll init) is implemented in Rust in `crates/web` (`wingy-hypertext-web`) via `web-sys`/`js-sys`, as a close port of the JavaScript modules under `webassets/js` (each Rust module mirrors one JS module). `main.js` wires the gallery up by calling thin `#[wasm_bindgen]` entry points re-exported from `examples/client/src/lib.rs` (e.g. `register_copy_action`, `listen_code_examples`, `init_page_element`) instead of importing the `webassets/js` modules; only third-party vendor scripts (htmx, highlight.js) and the htmx request interception (`client_patch.js`) stay on the JS side. The old `webassets/js` modules are retained as reference. The crate uses still-unstable web-sys Web Animations bindings, so the build needs `--cfg=web_sys_unstable_apis` — set workspace-wide in `.cargo/config.toml` and repeated in `Makefile.toml`'s `RUSTFLAGS` (which would otherwise override the config).

## Conventions

- CSS class names are not hardcoded as string literals. They live as `&str` constants in `crates/lib/src/class.rs` (e.g. `BUTTON`, `LABEL`, `HINT`, `STACK`, `HEADING_M`, and `wa-*` utility classes). Reference these constants both in `#[const_str(CLASS = …)]` on components and in `class=(…)` attributes throughout the gallery; add new entries there rather than inlining a literal. Because `class=` then takes constants, multi-class attributes are built by tupling with separators, e.g. `class=(STACK, " ", GAP_S)`.
- Rust edition 2024 across the workspace.
- The library is rendering-only and intentionally minimal — it produces Web Awesome-classed markup; the actual styling/behavior lives in `webassets/style` and `webassets/js` (and Web Awesome itself).
- Inline JS is deliberately avoided (see git history `feat: dont use inline JS`); behavior is wired through external modules in `webassets/js/` and gallery assets.
- Dual-licensed MIT OR Apache-2.0.
