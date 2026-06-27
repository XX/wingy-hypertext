//! Small DOM access helpers shared across the ported scripts.

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Document, Element, HtmlElement, Window};

pub fn window() -> Window {
    web_sys::window().expect("no global `window`")
}

pub fn document() -> Document {
    window().document().expect("no `document` on `window`")
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
        let cb = Closure::once_into_js(move |_ts: f64| {
            let _ = resolve.call0(&JsValue::NULL);
        });
        let _ = window().request_animation_frame(cb.as_ref().unchecked_ref());
    });
    let _ = JsFuture::from(promise).await;
}

/// `setTimeout(fn, ms)` for a one-shot callback.
pub fn set_timeout<F: FnOnce() + 'static>(ms: i32, f: F) {
    let cb = Closure::once_into_js(f);
    let _ = window().set_timeout_with_callback_and_timeout_and_arguments_0(cb.as_ref().unchecked_ref(), ms);
}
