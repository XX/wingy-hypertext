use futures::channel::oneshot;
use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use wasm_dom as dom;
use web_sys::{Element, KeyframeAnimationOptions};

/// Same as `element.animate()`, except it resolves without throwing when the animation is canceled.
pub async fn animate(element: &Element, keyframes: &JsValue, options: &KeyframeAnimationOptions) {
    let animation = element.animate_with_keyframe_animation_options(keyframes.dyn_ref::<Object>(), options);
    if let Ok(finished) = animation.finished() {
        // The `finished` promise rejects with an `AbortError` when canceled; ignore it.
        JsFuture::from(finished).await.ok();
    }
}

pub async fn linear_animate(element: &Element, keyframes: &JsValue, duration: f64) {
    let options = KeyframeAnimationOptions::new();
    options.set_duration(duration);
    options.set_easing("linear");

    animate(element, keyframes, &options).await
}

/// Applies a class to animate an element, removing it once the animation finishes (or if there is none).
pub async fn animate_with_class(element: &Element, class_name: &str) -> Result<(), JsValue> {
    let class_list = element.class_list();

    if class_list.contains(class_name) {
        return Ok(());
    }
    class_list.add_1(class_name)?;

    // Ждём один кадр, чтобы анимация (если она есть) успела стартовать
    // и попасть в getAnimations().
    dom::animation::next_frame().await.ok();

    // Если анимация есть (length != 0), то ждём конца анимации. Если ничего не анимируется
    // (нет анимации или 0ms), то мы просто снимаем класс ниже.
    if element.get_animations().length() != 0 {
        let (tx, rx) = oneshot::channel::<()>();
        let mut tx = Some(tx);

        let callback = dom::event::closure(move |_event| {
            if let Some(tx) = tx.take() {
                tx.send(()).ok();
            }
        });

        element.add_event_listener_with_callback("animationend", callback.as_ref().unchecked_ref())?;
        element.add_event_listener_with_callback("animationcancel", callback.as_ref().unchecked_ref())?;

        rx.await.ok();

        element.remove_event_listener_with_callback("animationend", callback.as_ref().unchecked_ref())?;
        element.remove_event_listener_with_callback("animationcancel", callback.as_ref().unchecked_ref())?;
    }

    class_list.remove_1(class_name)?;

    Ok(())
}

/// Tells whether the user has enabled the "reduced motion" setting.
pub fn prefers_reduced_motion() -> bool {
    dom::existing::window()
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false)
}
