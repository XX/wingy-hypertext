use std::cell::Cell;
use std::rc::Rc;

use futures::channel::oneshot;
use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Element, Event, KeyframeAnimationOptions};

use crate::dom::window;

/// Same as `element.animate()`, except it resolves without throwing when the animation is canceled.
pub async fn animate(element: &Element, keyframes: &JsValue, options: &KeyframeAnimationOptions) {
    let animation = element.animate_with_keyframe_animation_options(keyframes.dyn_ref::<Object>(), options);
    if let Ok(finished) = animation.finished() {
        // The `finished` promise rejects with an `AbortError` when canceled; ignore it.
        JsFuture::from(finished).await.ok();
    }
}

/// Applies a class to animate an element, removing it once the animation finishes (or if there is none).
pub async fn animate_with_class(element: &Element, class_name: &str) -> Result<(), JsValue> {
    let class_list = element.class_list();
    if class_list.contains(class_name) {
        return Ok(());
    }
    class_list.add_1(class_name)?;

    let (tx, rx) = oneshot::channel::<()>();
    let sender = Rc::new(Cell::new(Some(tx)));

    let on_end: Rc<dyn Fn()> = {
        let element = element.clone();
        let class_name = class_name.to_string();
        let sender = sender.clone();

        Rc::new(move || {
            if let Some(tx) = sender.take() {
                element.class_list().remove_1(&class_name).ok();
                tx.send(()).ok();
            }
        })
    };

    let event_callback = Closure::<dyn FnMut(Event)>::new({
        let on_end = on_end.clone();
        move |_event| on_end()
    });

    // if there are no animations or animation is set to 0ms, end immediately
    let raf_callback = Closure::<dyn FnMut()>::new({
        let on_end = on_end.clone();
        let element = element.clone();
        move || {
            if element.get_animations().length() == 0 {
                on_end();
            }
        }
    });

    let window = window();

    element.add_event_listener_with_callback("animationend", event_callback.as_ref().unchecked_ref())?;
    element.add_event_listener_with_callback("animationcancel", event_callback.as_ref().unchecked_ref())?;
    let raf_handle = window.request_animation_frame(raf_callback.as_ref().unchecked_ref())?;

    rx.await.unwrap_throw();

    element.remove_event_listener_with_callback("animationend", event_callback.as_ref().unchecked_ref())?;
    element.remove_event_listener_with_callback("animationcancel", event_callback.as_ref().unchecked_ref())?;
    window.cancel_animation_frame(raf_handle)?;

    Ok(())
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
