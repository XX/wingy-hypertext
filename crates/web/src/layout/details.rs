//! A `Details` behavior: expanding and collapsing with a height animation,
//! the header's click and arrow-key handling, keeping a disabled details shut,
//! accordion groups that collapse with an animation too, and the cancelable
//! `wg-show`/`wg-hide` (plus `wg-after-show`/`wg-after-hide`) lifecycle
//! events. The state lives entirely in the DOM, matching the markup produced
//! by `wingy_hypertext::layout::details`.
//!
//! The height animation is the shared [`crate::util::collapse`] one.
//!
//! The logical state is the `open` class. The native `open` attribute follows
//! it, except that a collapsing details keeps the attribute until the
//! animation ends, so the content stays visible while it collapses.

use const_format::concatcp;
use js_sys::Reflect;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{AddEventListenerOptions, Element, Event, HtmlElement, KeyboardEvent};
use wingy_hypertext::class::{DETAILS, DETAILS_BODY, DETAILS_HEADER, OPEN};

pub use crate::util::class::{is_disabled, is_open};
use crate::util::collapse::{animate_body, bump_generation, reset_body};
use crate::util::convert::{bool_to_str, details};
use crate::util::event;
use crate::util::id::ensure_id;

/// The group name of a details collapsing on behalf of another one in its
/// group, put aside while it collapses (see [`close_others_with_same_name`]).
const SAVED_NAME: &str = "wgDetailsName";

/// Clicks on these elements inside the header are theirs, not the header's.
const INTERACTIVE: &str = "a, button, input, textarea, select";

pub fn all_details() -> impl Iterator<Item = Element> {
    dom::existing::select_all_elements(concatcp!('.', DETAILS))
}

pub fn header(details: &Element) -> Option<Element> {
    details.query_selector(concatcp!(":scope > .", DETAILS_HEADER)).ok()?
}

pub fn body(details: &Element) -> Option<HtmlElement> {
    details
        .query_selector(concatcp!(":scope > .", DETAILS_BODY))
        .ok()??
        .maybe_into_html()
}

/// The details whose header is `header`, or `None` for any other element.
fn details_of_header(header: &Element) -> Option<Element> {
    if !header.class_list().contains(DETAILS_HEADER) {
        return None;
    }

    header
        .parent_element()
        .filter(|details| details.class_list().contains(DETAILS))
}

/// Sets the logical state: the class the styles follow and the header's `aria-expanded`.
fn set_expanded(details: &Element, open: bool) {
    details.class_list().toggle_with_force(OPEN, open).ok();
    if let Some(header) = header(details) {
        header.set_attribute("aria-expanded", bool_to_str(open)).ok();
    }
}

/// Expands the details, animating it unless the `wg-show` event is canceled.
pub async fn show(details_el: Element) -> Option<()> {
    if is_open(&details_el) || is_disabled(&details_el) {
        return None;
    }

    let native = details(&details_el)?;
    let body = body(&details_el)?;

    // A canceled `wg-show` keeps the details collapsed, even when the browser
    // has already opened it (on a search in the page, say).
    if !event::dispatch_custom(&details_el, event::SHOW, true, true, &JsValue::NULL).unwrap_or(true) {
        native.set_open(false);
        return None;
    }

    close_others_with_same_name(&details_el);

    let generation = bump_generation(&details_el);
    set_expanded(&details_el, true);
    native.set_open(true);

    if !animate_body(&details_el, &body, true, generation).await {
        return None;
    }

    event::dispatch_custom(&details_el, event::AFTER_SHOW, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Collapses the details, animating it unless the `wg-hide` event is canceled.
pub async fn hide(details: Element) -> Option<()> {
    if is_disabled(&details) {
        return None;
    }

    collapse(details).await
}

/// Collapses the details even when it is disabled: a details in an accordion
/// group closes when another one in the group opens, as natively.
async fn collapse(details_el: Element) -> Option<()> {
    if !is_open(&details_el) {
        return None;
    }

    let native = details(&details_el)?;
    let body = body(&details_el)?;

    if !event::dispatch_custom(&details_el, event::HIDE, true, true, &JsValue::NULL).unwrap_or(true) {
        return None;
    }

    let generation = bump_generation(&details_el);
    set_expanded(&details_el, false);

    if !animate_body(&details_el, &body, false, generation).await {
        return None;
    }

    native.set_open(false);
    reset_body(&body);

    event::dispatch_custom(&details_el, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Collapses the other open details of the group, the way the browser does
/// for a native `name` group — but with an animation. The browser would close
/// them instantly as soon as this one opens, so each of them leaves the group
/// for the time it collapses.
fn close_others_with_same_name(details: &Element) {
    let Some(name) = details.get_attribute("name").filter(|name| !name.is_empty()) else {
        return;
    };

    for other in all_details() {
        if other == *details || !is_open(&other) || other.get_attribute("name").as_deref() != Some(name.as_str()) {
            continue;
        }

        Reflect::set(&other, &JsValue::from_str(SAVED_NAME), &JsValue::from_str(&name)).ok();
        other.remove_attribute("name").ok();

        spawn_local(async move {
            collapse(other.clone()).await;
            restore_name(&other);
        });
    }
}

fn restore_name(details: &Element) {
    let key = JsValue::from_str(SAVED_NAME);
    if let Some(name) = Reflect::get(details, &key).ok().and_then(|name| name.as_string()) {
        details.set_attribute("name", &name).ok();
        Reflect::delete_property(details, &key).ok();
    }
}

/// Expands or collapses the details, as the header does.
pub fn set_details_open(details: &Element, open: bool) {
    let details = details.clone();
    spawn_local(async move {
        if open {
            show(details).await;
        } else {
            hide(details).await;
        }
    });
}

pub fn toggle_details(details: &Element) {
    set_details_open(details, !is_open(details));
}

/// A click on the header toggles the details, unless it is meant for an
/// interactive element placed in the summary. The native toggle is canceled
/// to animate it instead.
pub fn handle_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let header = target.closest(concatcp!('.', DETAILS_HEADER)).ok()??;
    let details = details_of_header(&header)?;

    if let Some(interactive) = target.closest(INTERACTIVE).ok().flatten()
        && header.contains(Some(&interactive))
    {
        return None;
    }

    event.prevent_default();

    if !is_disabled(&details) {
        toggle_details(&details);
        if let Some(header) = header.maybe_into_html() {
            header.focus().ok();
        }
    }

    Some(())
}

/// [Enter] and [Space] toggle the details from its header, the up and left
/// arrows collapse it, the down and right arrows expand it.
pub fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    let header = event.target()?.maybe_into_element()?;
    let details = details_of_header(&header)?;

    match keyboard.key().as_str() {
        "Enter" | " " => {
            keyboard.prevent_default();
            toggle_details(&details);
        },
        "ArrowUp" | "ArrowLeft" => {
            keyboard.prevent_default();
            set_details_open(&details, false);
        },
        "ArrowDown" | "ArrowRight" => {
            keyboard.prevent_default();
            set_details_open(&details, true);
        },
        _ => {},
    }

    Some(())
}

/// Follows the browser when it opens or closes a details on its own: a search
/// in the page expands the details holding the match, a native group closes
/// the other details of the group, a script sets `open`.
pub fn handle_toggle(event: &Event) -> Option<()> {
    let details_el = event.target()?.maybe_into_element()?;
    if !details_el.class_list().contains(DETAILS) {
        return None;
    }

    let native = details(&details_el)?;
    let open = native.open();
    if open == is_open(&details_el) {
        return None;
    }

    // A disabled details stays as it is
    if is_disabled(&details_el) {
        native.set_open(!open);
        return None;
    }

    if open {
        spawn_local(async move {
            show(details_el).await;
        });
    } else {
        // Already collapsed, there is nothing to animate
        bump_generation(&details_el);
        set_expanded(&details_el, false);
        if let Some(body) = body(&details_el) {
            reset_body(&body);
        }

        if event::dispatch_custom(&details_el, event::HIDE, true, true, &JsValue::NULL).unwrap_or(true) {
            event::dispatch_custom(&details_el, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();
        } else {
            set_expanded(&details_el, true);
            native.set_open(true);
        }
    }

    Some(())
}

/// Links the header and the body to each other by id. The server does it when
/// the details has an id; otherwise the ids are generated here.
fn link_header_and_body(details: &Element) -> Option<()> {
    let header = header(details)?;
    let body = body(details)?;

    let header_id = ensure_id(&header, "details-header");
    let body_id = ensure_id(&body, "details-body");
    header.set_attribute("aria-controls", &body_id).ok();
    body.set_attribute("aria-labelledby", &header_id).ok();

    Some(())
}

/// Synchronizes every `.details` on the page with its markup: links the header
/// and the body, stops a stale animation and aligns the logical state with the
/// native one. Run it after every render.
pub fn init_details() {
    for details_el in all_details() {
        link_header_and_body(&details_el);
        restore_name(&details_el);

        if let Some(body) = body(&details_el) {
            reset_body(&body);
        }
        if let Some(native) = details(&details_el) {
            set_expanded(&details_el, native.open());
        }
    }
}

/// Installs the document-level listeners driving every details on the page.
pub fn listen_details() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_click(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });

    // `toggle` doesn't bubble, but it does capture
    let options = AddEventListenerOptions::new();
    options.set_capture(true);
    document.add_steady_event_listener_with_options(
        "toggle",
        |event| {
            handle_toggle(&event);
        },
        &options,
    );
}
