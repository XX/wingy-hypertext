//! Rust ports of the gallery's client-side scripts (`webassets/js`), compiled to WebAssembly.
//!
//! Each module mirrors one JavaScript module one-to-one; behavior is kept as close to the original
//! prototype as the web-sys bindings allow. Web Awesome's own styling/behavior and third-party
//! vendor scripts (htmx, highlight.js) are out of scope and stay on the JavaScript side.

pub mod components;
pub mod dom;
pub mod layouts;
pub mod utils;

/// One-time wiring that replaces the imperative setup the gallery's `main.js` used to perform.
///
/// Registers the copy action, the click-action delegate, and the code-example listeners, then runs
/// the initial code-example initialization.
pub fn init_runtime() {
    components::copy_button::register_copy_action();
    utils::action::listen_click_actions();
    init_code_examples();
    layouts::code_example::listen_code_examples();
}

/// Re-initialization run after every htmx settle: refreshes page metrics and the scroll anchor.
pub fn reinit() {
    layouts::page::init_page_element();
    components::head::init_scroll_to_anchor();
}

pub use components::copy_button::register_copy_action;
pub use components::head::init_scroll_to_anchor;
pub use layouts::code_example::{init_code_examples, listen_code_examples};
pub use layouts::page::init_page_element;
pub use utils::action::listen_click_actions;
