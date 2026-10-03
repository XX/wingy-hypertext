//! The two parts of a `Textarea` that have to be measured or counted in the
//! browser: the height of a field that follows its content (`resize=Auto`) and
//! the character count. Everything else — typing, resizing by hand, validation,
//! form submission — is native.

use const_format::concatcp;
use js_sys::Reflect;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::{JsCast, JsValue};
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{Element, Event, HtmlElement, HtmlTextAreaElement, ResizeObserver};
use wingy_hypertext::class::{CONTROL, RESIZE_AUTO, RESIZE_NONE, SIZE_ADJUSTER, TEXTAREA_COUNT, TEXTAREA_FIELD};

/// Marks a textarea whose control is already observed, so repeated `init` calls
/// don't stack observers. It is a property on the element object rather than an
/// attribute: htmx snapshots the markup for its history cache, and an attribute
/// would come back on a restored — and therefore unobserved — element.
const OBSERVED: &str = "wgTextareaObserved";

pub fn control_of(textarea: &Element) -> Option<HtmlTextAreaElement> {
    textarea
        .query_selector(concatcp!('.', CONTROL))
        .ok()??
        .dyn_into::<HtmlTextAreaElement>()
        .ok()
}

/// The element sharing the grid cell with the control. It holds the measured
/// height, so the field neither collapses while the control is measured nor
/// keeps the old height when the text shrinks.
pub fn size_adjuster(textarea: &Element) -> Option<HtmlElement> {
    let field = textarea.query_selector(concatcp!('.', TEXTAREA_FIELD)).ok()??;

    if let Some(adjuster) = field.query_selector(concatcp!('.', SIZE_ADJUSTER)).ok()? {
        return adjuster.maybe_into_html();
    }

    let adjuster = dom::existing::document().create_element("div").ok()?;
    adjuster.set_class_name(SIZE_ADJUSTER);
    adjuster.set_attribute("aria-hidden", "true").ok();
    field.append_child(&adjuster).ok()?;
    adjuster.maybe_into_html()
}

/// Fits the height of an `auto` textarea to its content, the way
/// `setTextareaHeight` does in `wa-textarea`: pin the current height, let the
/// control collapse to measure `scroll_height`, then apply it to both.
pub fn fit_height(textarea: &Element) -> Option<()> {
    if !textarea.class_list().contains(RESIZE_AUTO) {
        return None;
    }

    let control = control_of(textarea)?;
    let adjuster = size_adjuster(textarea)?;

    adjuster
        .style()
        .set_property("height", &format!("{}px", control.client_height()))
        .ok();
    control.style().set_property("height", "auto").ok();

    let height = control.scroll_height();
    control.style().set_property("height", &format!("{height}px")).ok();
    adjuster.style().set_property("height", &format!("{height}px")).ok();

    Some(())
}

/// Updates the character count: the number of characters entered, or how many
/// are left when the field carries a `maxlength`. The length is the one the
/// browser counts for `maxlength` — UTF-16 code units — so the two agree.
pub fn update_count(textarea: &Element) -> Option<()> {
    let count = textarea.query_selector(concatcp!('.', TEXTAREA_COUNT)).ok()??;
    let control = control_of(textarea)?;
    let length = control.value().encode_utf16().count();

    let text = match control.max_length() {
        maxlength if maxlength >= 0 => {
            format!("{} characters left", (maxlength as usize).saturating_sub(length))
        },
        _ => format!("{length} characters"),
    };
    count.set_text_content(Some(&text));

    Some(())
}

/// Mirrors the size of a manually resized control onto the box around it. The
/// browser resizes the control itself, so without this the resizer would move
/// while the visible field — the bordered box — stayed put (`setTextareaDimensions`
/// in `wa-textarea` does the same).
pub fn sync_field_size(textarea: &Element) -> Option<()> {
    let field = textarea
        .query_selector(concatcp!('.', TEXTAREA_FIELD))
        .ok()??
        .maybe_into_html()?;
    let control = control_of(textarea)?;

    // An `auto` field owns its height, and a fixed one has nothing to follow
    if textarea.class_list().contains(RESIZE_AUTO) || textarea.class_list().contains(RESIZE_NONE) {
        field.style().remove_property("width").ok();
        field.style().remove_property("height").ok();
        return Some(());
    }

    // The control sits inside the borders of the box, so the box is that much larger
    let style = dom::existing::window().get_computed_style(&field).ok()??;
    let border = |name: &str| -> f64 {
        style
            .get_property_value(name)
            .ok()
            .and_then(|value| value.trim_end_matches("px").parse().ok())
            .unwrap_or(0.0)
    };

    // The size that ends up on the box is the one the control really has, not the
    // one the drag asked for: a control dragged past its minimum keeps that
    // minimum, and mirroring the requested size would leave the box — and its
    // resizer — smaller than the field inside it.
    let rect = control.get_bounding_client_rect();

    for (property, size, borders) in [
        (
            "width",
            rect.width(),
            border("border-left-width") + border("border-right-width"),
        ),
        (
            "height",
            rect.height(),
            border("border-top-width") + border("border-bottom-width"),
        ),
    ] {
        // The control carries a size of its own only once it has been dragged
        if control
            .style()
            .get_property_value(property)
            .unwrap_or_default()
            .is_empty()
        {
            field.style().remove_property(property).ok();
            continue;
        }

        field
            .style()
            .set_property(property, &format!("{}px", size + borders))
            .ok();
    }

    Some(())
}

/// Watches a manually resizable control, so the box follows it while it is
/// being dragged rather than only after.
pub fn observe_resize(textarea: &Element) -> Option<()> {
    if Reflect::get(textarea, &JsValue::from_str(OBSERVED))
        .map(|marked| marked.is_truthy())
        .unwrap_or(false)
    {
        return Some(());
    }

    let control = control_of(textarea)?;
    let observed = textarea.clone();
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |_: js_sys::Array| {
        sync_field_size(&observed);
    });
    let observer = ResizeObserver::new(callback.as_ref().unchecked_ref()).ok()?;
    observer.observe(&control);

    // The observer lives as long as the textarea does
    callback.forget();
    Reflect::set(textarea, &JsValue::from_str(OBSERVED), &JsValue::TRUE).ok();

    Some(())
}

pub fn sync(textarea: &Element) {
    fit_height(textarea);
    update_count(textarea);
    sync_field_size(textarea);
}

/// Synchronizes every textarea on the page with its content. Run it after every
/// render: a swap brings new markup, and the height of an `auto` field is a
/// style rather than markup.
pub fn init_textareas() {
    for textarea in dom::existing::select_all_elements(".textarea") {
        observe_resize(&textarea);
        sync(&textarea);
    }
}

/// Installs the document-level listener that keeps the textareas in step while
/// they are typed into.
pub fn listen_textareas() {
    dom::existing::document().add_steady_event_listener("input", |event| {
        handle_input(&event);
    });
}

pub fn handle_input(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let textarea = target.closest(".textarea").ok()??;
    sync(&textarea);
    Some(())
}
