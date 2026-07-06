use std::cell::RefCell;
use std::rc::Rc;

use js_sys::{Array, Function, Object};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{AddEventListenerOptions, Animation, Element, Event, HtmlElement, MouseEvent, TouchEvent};

use crate::utils::animate::{linear_animate, prefers_reduced_motion};
use crate::utils::convert::{bool_to_str, parse_duration_millis, parse_float};

//
// Resizing previews
//

/// Begins a drag-to-resize gesture when a `.code-example-resizer` inside a `.code-example-preview` is pressed.
pub fn handle_resizer_drag(event: &Event) -> Option<()> {
    const PREVIEW_DRAGGING_CLASS: &str = "code-example-preview--dragging";

    let target = event.target()?.maybe_into_element()?;
    let _resizer = target.closest(".code-example-resizer").ok()??;
    let preview = target.closest(".code-example-preview").ok()??.maybe_into_html()?;

    let start_x = pointer_client_x(event);
    let start_width = parse_float(
        dom::existing::window()
            .get_computed_style(&preview)
            .ok()??
            .get_property_value("width")
            .unwrap_or_default(),
    );

    event.prevent_default();
    preview.class_list().add_1(PREVIEW_DRAGGING_CLASS).ok();

    let root = dom::document_element().ok()?;

    let drag_move = dom::event::js_function({
        let preview = preview.clone();
        move |event| {
            let width = start_width + pointer_page_x(&event) - start_x;
            preview.style().set_property("width", &format!("{width}px")).ok();
        }
    });

    // Слот нужен только для того, чтобы stop мог снять сам себя.
    let stop_slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));

    let drag_stop = dom::event::js_function({
        let preview = preview.clone();
        let root = root.clone();
        let drag_move = drag_move.clone();
        let stop_slot = stop_slot.clone();
        move |_| {
            preview.class_list().remove_1(PREVIEW_DRAGGING_CLASS).ok();

            root.remove_event_listener_with_callback("mousemove", &drag_move).ok();
            root.remove_event_listener_with_callback("touchmove", &drag_move).ok();
            if let Some(stop_fn) = stop_slot.borrow().as_ref() {
                root.remove_event_listener_with_callback("mouseup", stop_fn).ok();
                root.remove_event_listener_with_callback("touchend", stop_fn).ok();
            }
        }
    });

    *stop_slot.borrow_mut() = Some(drag_stop.clone());

    root.add_event_listener_with_callback("mousemove", &drag_move).ok();
    root.add_event_listener_with_callback("touchmove", &drag_move).ok();
    root.add_event_listener_with_callback("mouseup", &drag_stop).ok();
    root.add_event_listener_with_callback("touchend", &drag_stop).ok();

    Some(())
}

/// `event.clientX`, or the first changed touch's `pageX` for touch events (mirrors the JS gesture start).
fn pointer_client_x(event: &Event) -> f64 {
    first_touch_page_x(event).unwrap_or_else(|| {
        event
            .dyn_ref::<MouseEvent>()
            .map(|event| event.client_x())
            .unwrap_or(0.0)
    })
}

/// `event.pageX`, or the first changed touch's `pageX` for touch events (mirrors the JS drag move).
fn pointer_page_x(event: &Event) -> f64 {
    first_touch_page_x(event)
        .unwrap_or_else(|| event.dyn_ref::<MouseEvent>().map(|event| event.page_x()).unwrap_or(0.0))
}

fn first_touch_page_x(event: &Event) -> Option<f64> {
    event
        .dyn_ref::<TouchEvent>()
        .and_then(|event| event.changed_touches().get(0))
        .map(|touch| touch.page_x() as _)
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
    code_example
        .set_attribute("data-animation-generation", &generation.to_string())
        .ok();
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
    let style = dom::existing::window().get_computed_style(source).ok().flatten();
    let read = |name: &str| -> f64 {
        let raw = style
            .as_ref()
            .and_then(|style| style.get_property_value(name).ok())
            .unwrap_or_default();
        let trimmed = raw.trim();
        if trimmed.is_empty() {
            200.0
        } else {
            parse_duration_millis(trimmed)
        }
    };

    (
        read("--code-example-show-duration"),
        read("--code-example-hide-duration"),
    )
}

fn set_code_example_source_accessibility(source: &Element, open: bool) {
    if open {
        source.remove_attribute("aria-hidden").ok();
    } else {
        source.set_attribute("aria-hidden", "true").ok();
    }
}

fn set_code_example_source_collapsed(source_html: &HtmlElement, collapsed: bool) {
    if collapsed {
        source_html.style().set_property("height", "0").ok();
        source_html.style().set_property("opacity", "0").ok();
    } else {
        source_html.style().set_property("height", "auto").ok();
        source_html.style().remove_property("opacity").ok();
    }
}

fn reset_code_example_element(code_example: &Element) {
    if let Some(source) = code_example.query_selector(".code-example-source").ok().flatten() {
        cancel_source_animations(&source);
        source.class_list().remove_1("is-animating").ok();
    }

    if let Some(preview) = code_example.query_selector(".code-example-preview").ok().flatten() {
        preview.class_list().remove_1("is-dragging").ok();
        if let Some(html) = preview.dyn_ref::<HtmlElement>() {
            html.style().remove_property("width").ok();
        }
    }
}

/// Opens or closes a code example, animating the source panel unless reduced motion is requested.
pub async fn set_code_example_open(code_example: Element, toggle: Element, open: bool) -> Option<()> {
    let source = code_example.query_selector(".code-example-source").ok()??;

    let generation = bump_animation_generation(&code_example);
    cancel_source_animations(&source);
    source.class_list().remove_1("is-animating").ok();

    let source_html = source.maybe_as_html()?;

    if prefers_reduced_motion() || source.class_list().contains("no-animation") {
        toggle.set_attribute("aria-expanded", bool_to_str(open)).ok();
        code_example.class_list().toggle_with_force("open", open).ok();
        set_code_example_source_collapsed(source_html, !open);
        set_code_example_source_accessibility(&source, open);
        return None;
    }

    let (show_duration, hide_duration) = get_code_example_durations(&source);

    if open {
        toggle.set_attribute("aria-expanded", "true").ok();
        code_example.class_list().add_1("open").ok();
        set_code_example_source_accessibility(&source, true);
        source.class_list().add_1("is-animating").ok();
        source_html.style().set_property("height", "0").ok();
        source_html.style().set_property("opacity", "0").ok();

        dom::animation::next_frame().await.ok();

        let keyframes = Array::of2(
            &keyframe("0", "0"),
            &keyframe(&format!("{}px", source_html.scroll_height()), "1"),
        );
        linear_animate(&source, &keyframes, show_duration).await;

        if get_animation_generation(&code_example) != generation {
            return None;
        }

        source_html.style().set_property("height", "auto").ok();
        source_html.style().remove_property("opacity").ok();
        source.class_list().remove_1("is-animating").ok();
    } else {
        toggle.set_attribute("aria-expanded", "false").ok();
        source.class_list().add_1("is-animating").ok();
        // Remove `.open` before the animation so the chevron rotation and panel collapse run together.
        code_example.class_list().remove_1("open").ok();
        let start_height = source_html.scroll_height();
        source_html
            .style()
            .set_property("height", &format!("{start_height}px"))
            .ok();

        let keyframes = Array::of2(&keyframe(&format!("{start_height}px"), "1"), &keyframe("0", "0"));
        linear_animate(&source, &keyframes, hide_duration).await;

        if get_animation_generation(&code_example) != generation {
            return None;
        }

        set_code_example_source_collapsed(source_html, true);
        source.class_list().remove_1("is-animating").ok();
        set_code_example_source_accessibility(&source, false);
    }

    Some(())
}

/// Initializes all `.code-example` elements to match their current `open` state.
pub fn init_code_examples() {
    let examples = dom::existing::document()
        .query_selector_all(".code-example")
        .expect("querySelectorAll failed");
    for i in 0..examples.length() {
        let Some(code_example) = examples.get(i).and_then(|node| node.maybe_into_element()) else {
            continue;
        };
        let Some(source) = code_example.query_selector(".code-example-source").ok().flatten() else {
            continue;
        };

        reset_code_example_element(&code_example);

        let open = code_example.class_list().contains("open");
        if let Some(source_html) = source.maybe_as_html() {
            set_code_example_source_collapsed(source_html, !open);
        }
        set_code_example_source_accessibility(&source, open);
    }
}

/// Installs the document-level listeners that drive code-example toggling and preview resizing.
pub fn listen_code_examples() {
    let document = dom::existing::document();

    document.add_steady_event_listener("turbo:load", |_| init_code_examples());
    document.add_steady_event_listener("mousedown", |event| {
        handle_resizer_drag(&event);
    });

    let options = AddEventListenerOptions::new();
    options.set_passive(true);
    document.add_steady_event_listener_with_options(
        "touchstart",
        |event| {
            handle_resizer_drag(&event);
        },
        &options,
    );

    let click_handler = |event: Event| -> Option<()> {
        let target = event.target()?.maybe_into_element()?;
        let toggle = target.closest(".code-example-toggle").ok()??;
        let code_example = toggle.closest(".code-example").ok()??;
        let open = !code_example.class_list().contains("open");
        spawn_local(async move {
            set_code_example_open(code_example, toggle, open).await;
        });
        Some(())
    };
    document.add_steady_event_listener("click", move |event| {
        click_handler(event);
    });
}

fn keyframe(height: &str, opacity: &str) -> Object {
    let object = Object::new();
    object.set("height", height);
    object.set("opacity", opacity);
    object
}
