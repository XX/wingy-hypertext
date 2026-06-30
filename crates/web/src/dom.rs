//! Small DOM access helpers.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
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
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        let callback = Closure::once_into_js(move |_ts: f64| {
            resolve.call0(&JsValue::NULL).ok();
        });
        window().request_animation_frame(callback.as_ref().unchecked_ref()).ok();
    });

    JsFuture::from(promise).await.ok();
}

/// `setTimeout(fn, ms)` for a one-shot callback.
pub fn set_timeout<F: FnOnce() + 'static>(ms: i32, func: F) {
    let callback = Closure::once_into_js(func);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(callback.as_ref().unchecked_ref(), ms)
        .ok();
}
