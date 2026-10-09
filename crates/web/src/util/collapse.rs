//! The height animation of a collapsible body, shared by `Details` and
//! `Accordion`: the body grows from nothing to its content height while fading
//! in, and back. The height can't be animated from or to `auto`, so the scroll
//! height stands in for it, and the body is left to its natural size after.
//!
//! Durations come from the `--show-duration`/`--hide-duration` custom
//! properties and the timing function from `--easing` (linear when unset).
//! A generation counter on the owner element tells a finished animation
//! whether a newer expand/collapse has started in the meantime.

use js_sys::{Array, Object, Reflect};
use wasm_bindgen::{JsCast, JsValue};
use wasm_dom as dom;
use wasm_dom::existing::JsObjectAccess;
use web_sys::{Animation, Element, HtmlElement, KeyframeAnimationOptions};

use crate::util::animate::{animate, prefers_reduced_motion};
use crate::util::convert::parse_duration_style;

/// The generation of the latest expand/collapse of an element. It is a
/// property on the element object rather than an attribute, so htmx doesn't
/// snapshot it into the history cache.
const GENERATION: &str = "wgCollapseGeneration";

/// Set on the body while it animates: the styles clip its overflow then.
pub const ANIMATING: &str = "animating";

pub fn generation(owner: &Element) -> u32 {
    Reflect::get(owner, &JsValue::from_str(GENERATION))
        .ok()
        .and_then(|value| value.as_f64())
        .map(|value| value as u32)
        .unwrap_or(0)
}

/// Starts a new expand/collapse of the owner, outdating any running one.
pub fn bump_generation(owner: &Element) -> u32 {
    let generation = generation(owner) + 1;
    Reflect::set(owner, &JsValue::from_str(GENERATION), &JsValue::from(generation)).ok();
    generation
}

struct Timing {
    show_duration: f64,
    hide_duration: f64,
    easing: String,
}

fn timing(body: &HtmlElement) -> Timing {
    let style = dom::existing::window().get_computed_style(body).ok().flatten();

    let duration = |name| {
        style
            .as_ref()
            .and_then(|style| parse_duration_style(style, name))
            .unwrap_or(0.0)
    };
    let easing = style
        .as_ref()
        .and_then(|style| style.get_property_value("--easing").ok())
        .map(|easing| easing.trim().to_string())
        .filter(|easing| !easing.is_empty())
        .unwrap_or_else(|| "linear".to_string());

    Timing {
        show_duration: duration("--show-duration"),
        hide_duration: duration("--hide-duration"),
        easing,
    }
}

fn keyframe(height: &str, opacity: &str) -> Object {
    let object = Object::new();
    object.set("height", height);
    object.set("opacity", opacity);
    object
}

/// Stops a running expand/collapse, leaving the body to its natural size.
pub fn reset_body(body: &HtmlElement) {
    let animations = body.get_animations();
    for i in 0..animations.length() {
        if let Ok(animation) = animations.get(i).dyn_into::<Animation>() {
            animation.cancel();
        }
    }

    body.class_list().remove_1(ANIMATING).ok();
    body.style().remove_property("height").ok();
    body.style().remove_property("opacity").ok();
}

/// Animates the body from or to the collapsed state. Returns `false` when a
/// newer expand/collapse of the owner has started in the meantime and owns
/// the body now.
///
/// When collapsing, the collapsed state is set under the animation, so the
/// body doesn't flash back to its full height before the caller hides it.
pub async fn animate_body(owner: &Element, body: &HtmlElement, open: bool, generation: u32) -> bool {
    reset_body(body);

    if prefers_reduced_motion() {
        return true;
    }

    let timing = timing(body);
    body.class_list().add_1(ANIMATING).ok();

    let height = format!("{}px", body.scroll_height());
    let (keyframes, duration) = if open {
        (
            Array::of2(&keyframe("0", "0"), &keyframe(&height, "1")),
            timing.show_duration,
        )
    } else {
        body.style().set_property("height", "0").ok();
        body.style().set_property("opacity", "0").ok();

        (
            Array::of2(&keyframe(&height, "1"), &keyframe("0", "0")),
            timing.hide_duration,
        )
    };

    let options = KeyframeAnimationOptions::new();
    options.set_duration(duration);
    options.set_easing(&timing.easing);
    animate(body, &keyframes, &options).await;

    if self::generation(owner) != generation {
        return false;
    }

    body.class_list().remove_1(ANIMATING).ok();
    true
}
