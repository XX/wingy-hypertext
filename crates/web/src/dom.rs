//! Small DOM access helpers.

use futures::channel::oneshot;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Document, Element, HtmlElement, Window};

pub fn window() -> Window {
    web_sys::window().expect_throw("no global `window`")
}

pub fn document() -> Document {
    window().document().expect_throw("no `document` on `window`")
}

pub fn body() -> HtmlElement {
    document().body().expect_throw("no `body` on `document`")
}

/// `document.querySelector(selector)`, swallowing invalid-selector errors.
pub fn query_selector(selector: &str) -> Option<Element> {
    document().query_selector(selector).ok().flatten()
}

/// Sets the `hidden` property on an element, mirroring `el.hidden = value` in JS.
pub fn set_hidden(element: &Element, hidden: bool) {
    if let Some(html) = element.dyn_ref::<HtmlElement>() {
        html.set_hidden(hidden);
    }
}

/// Awaits a single animation frame, like `await new Promise(r => requestAnimationFrame(r))`.
pub async fn next_animation_frame() {
    let (tx, rx) = oneshot::channel::<()>();
    let callback = Closure::once_into_js(move |_ts: f64| {
        tx.send(()).ok();
    });
    window().request_animation_frame(callback.unchecked_ref()).ok();

    rx.await.ok();
}

/// `setTimeout(fn, ms)` for a one-shot callback.
pub fn set_timeout<F: FnOnce() + 'static>(ms: i32, func: F) {
    let callback = Closure::once_into_js(func);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), ms)
        .ok();
}
