//! Port of `webassets/js/utils/animate.js`.

use std::cell::Cell;
use std::rc::Rc;

use js_sys::{Function, Object};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, KeyframeAnimationOptions};

use crate::dom::window;

/// Same as `el.animate()`, except it resolves without throwing when the animation is canceled.
pub async fn animate(el: &Element, keyframes: &JsValue, options: &KeyframeAnimationOptions) {
    let animation = el.animate_with_keyframe_animation_options(keyframes.dyn_ref::<Object>(), options);
    // The `finished` promise rejects with an `AbortError` when canceled; ignore it (Safari throws).
    if let Ok(finished) = animation.finished() {
        let _ = JsFuture::from(finished).await;
    }
}

/// Applies a class to animate an element, removing it once the animation finishes (or if there is none).
pub async fn animate_with_class(el: &Element, class_name: &str) {
    let class_list = el.class_list();
    if class_list.contains(class_name) {
        return;
    }
    let _ = class_list.add_1(class_name);

    let el = el.clone();
    let class_name = class_name.to_string();

    let promise = js_sys::Promise::new(&mut move |resolve, _reject| {
        let resolved = Rc::new(Cell::new(false));
        // Shared slot so `on_end` can unregister itself once it fires.
        let on_end_slot: Rc<std::cell::RefCell<Option<Function>>> = Rc::new(std::cell::RefCell::new(None));

        let on_end = {
            let resolved = resolved.clone();
            let el = el.clone();
            let class_name = class_name.clone();
            let on_end_slot = on_end_slot.clone();
            let resolve = resolve.clone();
            Closure::<dyn FnMut()>::new(move || {
                if resolved.get() {
                    return;
                }
                resolved.set(true);
                let _ = el.class_list().remove_1(&class_name);
                let _ = resolve.call0(&JsValue::NULL);
                if let Some(f) = on_end_slot.borrow().as_ref() {
                    let _ = el.remove_event_listener_with_callback("animationend", f);
                    let _ = el.remove_event_listener_with_callback("animationcancel", f);
                }
            })
        };
        let on_end_fn: Function = on_end.as_ref().unchecked_ref::<Function>().clone();
        on_end.forget();
        *on_end_slot.borrow_mut() = Some(on_end_fn.clone());

        let _ = el.add_event_listener_with_callback("animationend", &on_end_fn);
        let _ = el.add_event_listener_with_callback("animationcancel", &on_end_fn);

        // If there are no animations (or a 0ms one), end immediately on the next frame.
        let el_raf = el.clone();
        let resolved_raf = resolved.clone();
        let on_end_raf = on_end_fn.clone();
        let raf = Closure::once_into_js(move |_ts: f64| {
            if !resolved_raf.get() && el_raf.get_animations().length() == 0 {
                let _ = on_end_raf.call0(&JsValue::NULL);
            }
        });
        let _ = window().request_animation_frame(raf.as_ref().unchecked_ref());
    });

    let _ = JsFuture::from(promise).await;
}

/// Parses a CSS duration and returns the number of milliseconds.
pub fn parse_duration(duration: &str) -> f64 {
    let duration = duration.trim().to_lowercase();

    if duration.contains("ms") {
        return parse_float(&duration);
    }

    if duration.contains('s') {
        return parse_float(&duration) * 1000.0;
    }

    parse_float(&duration)
}

/// Tells whether the user has enabled the "reduced motion" setting.
pub fn prefers_reduced_motion() -> bool {
    window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}

/// Mimics JavaScript's `parseFloat`: reads a leading (optionally signed) decimal number, defaulting to 0.
pub fn parse_float(input: &str) -> f64 {
    let trimmed = input.trim_start();
    let bytes = trimmed.as_bytes();
    let mut end = 0;
    let mut seen_dot = false;

    if let Some(&first) = bytes.first()
        && (first == b'+' || first == b'-')
    {
        end += 1;
    }

    while let Some(&byte) = bytes.get(end) {
        match byte {
            b'0'..=b'9' => end += 1,
            b'.' if !seen_dot => {
                seen_dot = true;
                end += 1;
            },
            _ => break,
        }
    }

    trimmed[..end].parse::<f64>().unwrap_or(0.0)
}
