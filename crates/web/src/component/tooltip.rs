//! A `Tooltip` behavior: showing and hiding the tooltip with animations, the
//! `hover`, `focus`, `click` and `manual` triggers with their delays, dismissal
//! with [Escape], and wiring the anchor's `aria-labelledby`. The tooltip is
//! anchored with the popup helper (`helper::popup`), and the state lives
//! entirely in the DOM (classes and attributes), matching the markup produced
//! by `wingy_hypertext::component::tooltip`.

use std::cell::Cell;

use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Element, Event, KeyboardEvent, MouseEvent};

use crate::helper::popup;
use crate::util::animate::animate_with_class;
pub use crate::util::class::{is_disabled, is_open};
use crate::util::event;

const DEFAULT_TRIGGER: &str = "hover focus";
const DEFAULT_SHOW_DELAY: i32 = 150;
const DEFAULT_HIDE_DELAY: i32 = 0;

/// The elements that dismiss on [Escape] before the tooltips do: they are
/// opened above a tooltip, so the key belongs to them first. Web Awesome keeps
/// a stack of dismissibles for this; the markup already tells us which of them
/// are open.
const DISMISSIBLE_ABOVE: &str = ".dropdown.open, .drawer.open";

pub fn tooltips() -> impl Iterator<Item = Element> {
    dom::existing::select_all_elements(".tooltip")
}

pub fn open_tooltips() -> impl Iterator<Item = Element> {
    dom::existing::select_all_elements(".tooltip.open")
}

fn popup_host(tooltip: &Element) -> Option<Element> {
    tooltip.query_selector(":scope > .popup").ok()?
}

fn popup_body(tooltip: &Element) -> Option<Element> {
    tooltip.query_selector(":scope > .popup > .popup-body").ok()?
}

/// The element the tooltip describes, referenced by id.
pub fn anchor_of(tooltip: &Element) -> Option<Element> {
    let id = popup_host(tooltip)?.get_attribute("data-anchor")?;
    dom::existing::document().get_element_by_id(&id)
}

fn has_trigger(tooltip: &Element, trigger: &str) -> bool {
    tooltip
        .get_attribute("data-trigger")
        .unwrap_or_else(|| DEFAULT_TRIGGER.to_string())
        .split_whitespace()
        .any(|value| value == trigger)
}

fn delay(tooltip: &Element, name: &str, default: i32) -> i32 {
    tooltip
        .get_attribute(name)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

//
// Showing and hiding
//

/// Shows the tooltip, animating it in unless the `wg-show` event is canceled.
async fn show(tooltip: Element) -> Option<()> {
    if is_open(&tooltip) || is_disabled(&tooltip) {
        return None;
    }

    // A canceled `wg-show` keeps the tooltip hidden.
    if !event::dispatch_custom(&tooltip, event::SHOW, true, true, &JsValue::NULL).unwrap_or(true) {
        return None;
    }

    let host = popup_host(&tooltip)?;
    let body = popup_body(&tooltip)?;

    tooltip.class_list().add_1("open").ok();
    popup::set_popup_active(&host, true);

    animate_with_class(&body, "show-with-scale").await.ok();

    // The anchor may have moved while the animation was running
    if is_open(&tooltip) {
        popup::reposition(&host);
    }

    event::dispatch_custom(&tooltip, event::AFTER_SHOW, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Requests to hide the tooltip. Dispatches a cancelable `wg-hide` carrying the
/// `source` element that triggered the request; when canceled the tooltip stays
/// open.
async fn hide(tooltip: Element, source: Element) -> Option<()> {
    if !is_open(&tooltip) {
        return None;
    }

    let detail = Object::new();
    detail.set("source", source);

    if !event::dispatch_custom(&tooltip, event::HIDE, true, true, detail.as_ref()).unwrap_or(true) {
        return None;
    }

    let host = popup_host(&tooltip)?;
    let body = popup_body(&tooltip)?;

    tooltip.class_list().remove_1("open").ok();

    animate_with_class(&body, "hide-with-scale").await.ok();

    // A quick re-open may have started while the hide animation was running.
    if !is_open(&tooltip) {
        popup::set_popup_active(&host, false);
    }

    event::dispatch_custom(&tooltip, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Shows or hides the tooltip; `source` is reported to `wg-hide` listeners.
pub fn set_tooltip_open(tooltip: &Element, open: bool, source: &Element) {
    cancel_pending(tooltip);

    let tooltip = tooltip.clone();
    let source = source.clone();
    spawn_local(async move {
        if open {
            show(tooltip).await;
        } else {
            hide(tooltip, source).await;
        }
    });
}

//
// Delayed showing and hiding
//

/// Drops the pending show or hide of a tooltip, if any.
fn cancel_pending(tooltip: &Element) {
    if let Some(handle) = tooltip
        .get_attribute("data-timer")
        .and_then(|handle| handle.parse().ok())
    {
        dom::existing::window().clear_timeout_with_handle(handle);
        tooltip.remove_attribute("data-timer").ok();
    }
}

/// Shows or hides the tooltip after its show or hide delay. The pending
/// timeout is kept on the element, so a tooltip only ever has one.
fn set_tooltip_open_delayed(tooltip: &Element, open: bool, source: &Element) {
    cancel_pending(tooltip);

    let delay = if open {
        delay(tooltip, "data-show-delay", DEFAULT_SHOW_DELAY)
    } else {
        delay(tooltip, "data-hide-delay", DEFAULT_HIDE_DELAY)
    };
    if delay <= 0 {
        set_tooltip_open(tooltip, open, source);
        return;
    }

    let element = tooltip.clone();
    let source = source.clone();
    let handle = dom::set_timeout(
        move || {
            element.remove_attribute("data-timer").ok();
            set_tooltip_open(&element, open, &source);
        },
        delay,
    );

    if let Ok(handle) = handle {
        tooltip.set_attribute("data-timer", &handle.to_string()).ok();
    }
}

//
// Event handlers
//

/// The tooltips anchored to `target` or to one of its ancestors. Nothing keeps
/// an element from being described by several tooltips, and there are only ever
/// a handful of them on a page, so they are scanned instead of being indexed by
/// the anchor's id.
fn tooltips_of_anchor(target: &Element) -> impl Iterator<Item = Element> {
    let target = target.clone();
    tooltips().filter(move |tooltip| anchor_of(tooltip).is_some_and(|anchor| anchor.contains(Some(&target))))
}

/// Shows the tooltip of the hovered anchor after its show delay, and keeps a
/// tooltip the pointer moved onto open.
pub fn handle_mouseover(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;

    if let Some(tooltip) = target.closest(".tooltip").ok().flatten() {
        // Moving onto the tooltip itself cancels the pending hide
        if has_trigger(&tooltip, "hover") {
            cancel_pending(&tooltip);
        }
        return Some(());
    }

    for tooltip in tooltips_of_anchor(&target) {
        if has_trigger(&tooltip, "hover") {
            set_tooltip_open_delayed(&tooltip, true, &target);
        }
    }

    Some(())
}

/// Hides the tooltip after its hide delay once the pointer leaves both the
/// anchor and the tooltip.
pub fn handle_mouseout(event: &Event) -> Option<()> {
    let mouse: &MouseEvent = event.dyn_ref()?;
    let target = event.target()?.maybe_into_element()?;
    let related = mouse.related_target().and_then(|related| related.maybe_into_element());

    let hide_unless_still_hovered = |tooltip: &Element| {
        if !has_trigger(tooltip, "hover") {
            return;
        }

        // The pointer is still on the anchor or on the tooltip when it merely
        // moved between their children: `relatedTarget` is what it moved onto.
        if let Some(related) = &related {
            let stays_on_anchor = anchor_of(tooltip).is_some_and(|anchor| anchor.contains(Some(related)));
            if stays_on_anchor || tooltip.contains(Some(related)) {
                return;
            }
        }

        set_tooltip_open_delayed(tooltip, false, &target);
    };

    if let Some(tooltip) = target.closest(".tooltip").ok().flatten() {
        hide_unless_still_hovered(&tooltip);
        return Some(());
    }

    for tooltip in tooltips_of_anchor(&target) {
        hide_unless_still_hovered(&tooltip);
    }

    Some(())
}

/// Shows the tooltip of the focused anchor. `focusin` is used instead of
/// `focus`, which doesn't bubble up to the document.
pub fn handle_focusin(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;

    for tooltip in tooltips_of_anchor(&target) {
        if has_trigger(&tooltip, "focus") {
            set_tooltip_open(&tooltip, true, &target);
        }
    }

    Some(())
}

pub fn handle_focusout(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;

    for tooltip in tooltips_of_anchor(&target) {
        if has_trigger(&tooltip, "focus") {
            set_tooltip_open(&tooltip, false, &target);
        }
    }

    Some(())
}

/// Toggles the tooltip of the clicked anchor.
pub fn handle_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;

    for tooltip in tooltips_of_anchor(&target) {
        if has_trigger(&tooltip, "click") {
            set_tooltip_open(&tooltip, !is_open(&tooltip), &target);
        }
    }

    Some(())
}

/// Dismisses the top-most open tooltip on [Escape], whatever its trigger is.
pub fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    if keyboard.key() != "Escape" {
        return None;
    }

    // An overlay opened above the tooltip dismisses itself first
    if dom::existing::document()
        .query_selector(DISMISSIBLE_ABOVE)
        .ok()
        .flatten()
        .is_some()
    {
        return None;
    }

    let tooltip = open_tooltips().last()?;
    event.prevent_default();
    event.stop_propagation();
    set_tooltip_open(&tooltip, false, &tooltip.clone());

    Some(())
}

//
// Initialization
//

thread_local! {
    static NEXT_ID: Cell<u32> = const { Cell::new(0) };
}

/// The tooltip's id, generating one when the markup doesn't carry it: the
/// anchor refers to the tooltip by id to be labeled by it.
fn ensure_id(tooltip: &Element) -> String {
    let id = tooltip.id();
    if !id.is_empty() {
        return id;
    }

    let document = dom::existing::document();
    let id = NEXT_ID.with(|next| {
        loop {
            let id = format!("tooltip-{}", next.get());
            next.set(next.get() + 1);
            if document.get_element_by_id(&id).is_none() {
                return id;
            }
        }
    });

    tooltip.set_id(&id);
    id
}

/// Labels the anchor by the tooltip. Web Awesome uses `aria-labelledby` rather
/// than `aria-describedby` here: it is the one screen readers announce
/// consistently on the first focus of a control.
fn add_to_aria_labelledby(anchor: &Element, id: &str) {
    let labels = anchor.get_attribute("aria-labelledby").unwrap_or_default();
    if labels.split_whitespace().any(|label| label == id) {
        return;
    }

    let labels = if labels.trim().is_empty() {
        id.to_string()
    } else {
        format!("{} {id}", labels.trim())
    };
    anchor.set_attribute("aria-labelledby", &labels).ok();
}

/// Synchronizes every `.tooltip` on the page with its markup state: resets a
/// stale open tooltip, labels the anchor, and shows the tooltips rendered with
/// `data-open`. Run it after every render.
pub fn init_tooltips() {
    for tooltip in tooltips() {
        cancel_pending(&tooltip);
        tooltip.class_list().remove_1("open").ok();

        if let Some(body) = popup_body(&tooltip) {
            body.class_list().remove_2("show-with-scale", "hide-with-scale").ok();
        }
        if let Some(host) = popup_host(&tooltip) {
            popup::set_popup_active(&host, false);
        }

        let id = ensure_id(&tooltip);
        if let Some(anchor) = anchor_of(&tooltip) {
            add_to_aria_labelledby(&anchor, &id);
        }

        if tooltip.has_attribute("data-open") {
            set_tooltip_open(&tooltip, true, &tooltip.clone());
        }
    }
}

/// Installs the document-level listeners that drive all tooltips on the page.
pub fn listen_tooltips() {
    let document = dom::existing::document();

    document.add_steady_event_listener("mouseover", |event| {
        handle_mouseover(&event);
    });
    document.add_steady_event_listener("mouseout", |event| {
        handle_mouseout(&event);
    });
    document.add_steady_event_listener("focusin", |event| {
        handle_focusin(&event);
    });
    document.add_steady_event_listener("focusout", |event| {
        handle_focusout(&event);
    });
    document.add_steady_event_listener("click", |event| {
        handle_click(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });
}
