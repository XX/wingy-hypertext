//! Port of `webassets/js/layouts/code_example.js`.

use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Function, Object, Reflect};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::{AddEventListenerOptions, Animation, Element, Event, HtmlElement, KeyframeAnimationOptions};

use crate::dom::{document, next_animation_frame, window};
use crate::utils::animate::{animate, parse_duration, parse_float, prefers_reduced_motion};

//
// Resizing previews
//

/// Begins a drag-to-resize gesture when a `.code-example-resizer` inside a `.code-example-preview` is pressed.
pub fn handle_resizer_drag(event: &Event) {
    let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
        return;
    };
    let resizer = target.closest(".code-example-resizer").ok().flatten();
    let preview = target.closest(".code-example-preview").ok().flatten();
    let (Some(_resizer), Some(preview)) = (resizer, preview) else {
        return;
    };
    let Some(preview) = preview.dyn_into::<HtmlElement>().ok() else {
        return;
    };

    let start_x = pointer_client_x(event);
    let start_width = parse_float(&computed_width(&preview));

    event.prevent_default();
    let _ = preview.class_list().add_1("code-example-preview--dragging");

    let Some(doc_el) = document().document_element() else {
        return;
    };

    let move_slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));
    let stop_slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));

    let drag_move = {
        let preview = preview.clone();
        Closure::<dyn FnMut(Event)>::new(move |event: Event| {
            let width = start_width + pointer_page_x(&event) - start_x;
            let _ = preview.style().set_property("width", &format!("{width}px"));
        })
    };
    let drag_move_fn: Function = drag_move.as_ref().unchecked_ref::<Function>().clone();
    drag_move.forget();
    *move_slot.borrow_mut() = Some(drag_move_fn.clone());

    let drag_stop = {
        let preview = preview.clone();
        let doc_el = doc_el.clone();
        let move_slot = move_slot.clone();
        let stop_slot = stop_slot.clone();
        Closure::<dyn FnMut()>::new(move || {
            let _ = preview.class_list().remove_1("code-example-preview--dragging");
            if let Some(move_fn) = move_slot.borrow().as_ref() {
                let _ = doc_el.remove_event_listener_with_callback("mousemove", move_fn);
                let _ = doc_el.remove_event_listener_with_callback("touchmove", move_fn);
            }
            if let Some(stop_fn) = stop_slot.borrow().as_ref() {
                let _ = doc_el.remove_event_listener_with_callback("mouseup", stop_fn);
                let _ = doc_el.remove_event_listener_with_callback("touchend", stop_fn);
            }
        })
    };
    let drag_stop_fn: Function = drag_stop.as_ref().unchecked_ref::<Function>().clone();
    drag_stop.forget();
    *stop_slot.borrow_mut() = Some(drag_stop_fn.clone());

    let _ = doc_el.add_event_listener_with_callback("mousemove", &drag_move_fn);
    let _ = doc_el.add_event_listener_with_callback("touchmove", &drag_move_fn);
    let _ = doc_el.add_event_listener_with_callback("mouseup", &drag_stop_fn);
    let _ = doc_el.add_event_listener_with_callback("touchend", &drag_stop_fn);
}

/// `event.clientX`, or the first changed touch's `pageX` for touch events (mirrors the JS gesture start).
fn pointer_client_x(event: &Event) -> f64 {
    if let Some(touch) = event.dyn_ref::<web_sys::TouchEvent>()
        && let Some(t) = touch.changed_touches().get(0)
    {
        return t.page_x() as f64;
    }
    event
        .dyn_ref::<web_sys::MouseEvent>()
        .map(|m| m.client_x())
        .unwrap_or(0.0)
}

/// `event.pageX`, or the first changed touch's `pageX` for touch events (mirrors the JS drag move).
fn pointer_page_x(event: &Event) -> f64 {
    if let Some(touch) = event.dyn_ref::<web_sys::TouchEvent>()
        && let Some(t) = touch.changed_touches().get(0)
    {
        return t.page_x() as f64;
    }
    event
        .dyn_ref::<web_sys::MouseEvent>()
        .map(|m| m.page_x())
        .unwrap_or(0.0)
}

fn computed_width(element: &Element) -> String {
    window()
        .get_computed_style(element)
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("width").ok())
        .unwrap_or_default()
}

//
// Code example open animation
//

fn get_animation_generation(code_example: &Element) -> u32 {
    code_example
        .get_attribute("data-animation-generation")
        .and_then(|value| value.parse().ok())
        .unwrap_or(0)
}

fn bump_animation_generation(code_example: &Element) -> u32 {
    let generation = get_animation_generation(code_example) + 1;
    let _ = code_example.set_attribute("data-animation-generation", &generation.to_string());
    generation
}

fn cancel_source_animations(source: &Element) {
    let animations = source.get_animations();
    for i in 0..animations.length() {
        if let Ok(animation) = animations.get(i).dyn_into::<Animation>() {
            animation.cancel();
        }
    }
}

fn get_code_example_durations(source: &Element) -> (f64, f64) {
    let style = window().get_computed_style(source).ok().flatten();
    let read = |name: &str| -> f64 {
        let raw = style
            .as_ref()
            .and_then(|s| s.get_property_value(name).ok())
            .unwrap_or_default();
        let trimmed = raw.trim();
        let value = if trimmed.is_empty() { "200ms" } else { trimmed };
        parse_duration(value)
    };
    (
        read("--code-example-show-duration"),
        read("--code-example-hide-duration"),
    )
}

fn set_code_example_source_accessibility(source: &Element, open: bool) {
    if open {
        let _ = source.remove_attribute("aria-hidden");
    } else {
        let _ = source.set_attribute("aria-hidden", "true");
    }
}

fn set_code_example_source_collapsed(source: &Element, collapsed: bool) {
    let Some(html) = source.dyn_ref::<HtmlElement>() else {
        return;
    };
    if collapsed {
        let _ = html.style().set_property("height", "0");
        let _ = html.style().set_property("opacity", "0");
    } else {
        let _ = html.style().set_property("height", "auto");
        let _ = html.style().remove_property("opacity");
    }
}

fn reset_code_example_element(code_example: &Element) {
    if let Some(source) = code_example.query_selector(".code-example-source").ok().flatten() {
        cancel_source_animations(&source);
        let _ = source.class_list().remove_1("is-animating");
    }

    if let Some(preview) = code_example.query_selector(".code-example-preview").ok().flatten() {
        let _ = preview.class_list().remove_1("is-dragging");
        if let Some(html) = preview.dyn_ref::<HtmlElement>() {
            let _ = html.style().remove_property("width");
        }
    }
}

/// Opens or closes a code example, animating the source panel unless reduced motion is requested.
pub async fn set_code_example_open(code_example: Element, toggle: Element, open: bool) {
    let Some(source) = code_example.query_selector(".code-example-source").ok().flatten() else {
        return;
    };

    let generation = bump_animation_generation(&code_example);
    cancel_source_animations(&source);
    let _ = source.class_list().remove_1("is-animating");

    if prefers_reduced_motion() || source.class_list().contains("no-animation") {
        let _ = toggle.set_attribute("aria-expanded", bool_str(open));
        let _ = code_example.class_list().toggle_with_force("open", open);
        set_code_example_source_collapsed(&source, !open);
        set_code_example_source_accessibility(&source, open);
        return;
    }

    let (show_duration, hide_duration) = get_code_example_durations(&source);
    let Some(source_html) = source.dyn_ref::<HtmlElement>().cloned() else {
        return;
    };

    if open {
        let _ = toggle.set_attribute("aria-expanded", "true");
        let _ = code_example.class_list().add_1("open");
        set_code_example_source_accessibility(&source, true);
        let _ = source.class_list().add_1("is-animating");
        let _ = source_html.style().set_property("height", "0");
        let _ = source_html.style().set_property("opacity", "0");

        next_animation_frame().await;

        let keyframes = Array::of2(
            &keyframe("0", "0"),
            &keyframe(&format!("{}px", source_html.scroll_height()), "1"),
        );
        animate(&source, &keyframes, &options(show_duration)).await;

        if get_animation_generation(&code_example) != generation {
            return;
        }

        let _ = source_html.style().set_property("height", "auto");
        let _ = source_html.style().remove_property("opacity");
        let _ = source.class_list().remove_1("is-animating");
    } else {
        let _ = toggle.set_attribute("aria-expanded", "false");
        let _ = source.class_list().add_1("is-animating");
        // Remove `.open` before the animation so the chevron rotation and panel collapse run together.
        let _ = code_example.class_list().remove_1("open");
        let start_height = source_html.scroll_height();
        let _ = source_html.style().set_property("height", &format!("{start_height}px"));

        let keyframes = Array::of2(&keyframe(&format!("{start_height}px"), "1"), &keyframe("0", "0"));
        animate(&source, &keyframes, &options(hide_duration)).await;

        if get_animation_generation(&code_example) != generation {
            return;
        }

        set_code_example_source_collapsed(&source, true);
        let _ = source.class_list().remove_1("is-animating");
        set_code_example_source_accessibility(&source, false);
    }
}

/// Initializes all `.code-example` elements to match their current `open` state.
pub fn init_code_examples() {
    let examples = document()
        .query_selector_all(".code-example")
        .expect("querySelectorAll failed");
    for i in 0..examples.length() {
        let Some(code_example) = examples.get(i).and_then(|n| n.dyn_into::<Element>().ok()) else {
            continue;
        };
        let Some(source) = code_example.query_selector(".code-example-source").ok().flatten() else {
            continue;
        };

        reset_code_example_element(&code_example);

        let open = code_example.class_list().contains("open");
        set_code_example_source_collapsed(&source, !open);
        set_code_example_source_accessibility(&source, open);
    }
}

/// Installs the document-level listeners that drive code-example toggling and preview resizing.
pub fn listen_code_examples() {
    let doc = document();

    add_listener(&doc, "turbo:load", |_event: Event| init_code_examples());
    add_listener(&doc, "mousedown", |event: Event| handle_resizer_drag(&event));

    // touchstart is passive, like the JS prototype.
    let touchstart = Closure::<dyn FnMut(Event)>::new(|event: Event| handle_resizer_drag(&event));
    let options = AddEventListenerOptions::new();
    options.set_passive(true);
    let _ = doc.add_event_listener_with_callback_and_add_event_listener_options(
        "touchstart",
        touchstart.as_ref().unchecked_ref(),
        &options,
    );
    touchstart.forget();

    add_listener(&doc, "click", |event: Event| {
        let Some(target) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
            return;
        };
        let Some(toggle) = target.closest(".code-example-toggle").ok().flatten() else {
            return;
        };
        let Some(code_example) = toggle.closest(".code-example").ok().flatten() else {
            return;
        };
        let open = !code_example.class_list().contains("open");
        spawn_local(async move {
            set_code_example_open(code_example, toggle, open).await;
        });
    });
}

fn add_listener(target: &web_sys::EventTarget, event: &str, handler: impl FnMut(Event) + 'static) {
    let cb = Closure::<dyn FnMut(Event)>::new(handler);
    let _ = target.add_event_listener_with_callback(event, cb.as_ref().unchecked_ref());
    cb.forget();
}

fn keyframe(height: &str, opacity: &str) -> Object {
    let object = Object::new();
    let _ = Reflect::set(&object, &JsValue::from_str("height"), &JsValue::from_str(height));
    let _ = Reflect::set(&object, &JsValue::from_str("opacity"), &JsValue::from_str(opacity));
    object
}

fn options(duration: f64) -> KeyframeAnimationOptions {
    let options = KeyframeAnimationOptions::new();
    options.set_duration(duration);
    options.set_easing("linear");
    options
}

fn bool_str(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}
