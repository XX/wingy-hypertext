//! A Rust port of the `wa-drawer` behavior: opening and closing the drawer as
//! a native modal `<dialog>` with slide-in/out animations, declarative
//! `data-drawer="open <id>"` / `data-drawer="close"` triggers, [Escape] and
//! light-dismiss handling, body scroll locking, and the cancelable
//! `wg-show`/`wg-hide` (plus `wg-after-show`/`wg-after-hide`) lifecycle events.
//! The state lives entirely in the DOM, matching the markup produced by
//! `wingy_hypertext::layouts::drawer`.

use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{Element, Event, HtmlDialogElement, KeyboardEvent};

use crate::utils::animate::animate_with_class;
use crate::utils::event;

fn dialog(drawer: &Element) -> Option<HtmlDialogElement> {
    drawer.clone().dyn_into::<HtmlDialogElement>().ok()
}

fn is_open(drawer: &Element) -> bool {
    dialog(drawer).map(|dialog| dialog.open()).unwrap_or(false)
}

/// Locks or unlocks page scrolling depending on whether any drawer is open.
fn update_body_scroll_lock() {
    let has_open = dom::existing::document()
        .query_selector(".drawer.open")
        .ok()
        .flatten()
        .is_some();

    let body = dom::existing::body();
    if has_open {
        body.style().set_property("overflow", "hidden").ok();
    } else {
        body.style().remove_property("overflow").ok();
    }
}

/// Shows the drawer, animating it in unless the `wg-show` event is canceled.
async fn show(drawer: Element) -> Option<()> {
    let dialog = dialog(&drawer)?;
    if dialog.open() {
        return None;
    }

    // A canceled `wg-show` keeps the drawer closed.
    if !event::dispatch_custom(&drawer, event::SHOW, true, true, &JsValue::NULL).unwrap_or(true) {
        return None;
    }

    dialog.show_modal().ok();
    drawer.class_list().add_1("open").ok();
    update_body_scroll_lock();

    // Move focus to an element marked `autofocus`, or the drawer itself.
    if let Some(autofocus) = drawer
        .query_selector("[autofocus]")
        .ok()
        .flatten()
        .and_then(|element| element.maybe_into_html())
    {
        autofocus.focus().ok();
    } else if let Some(html) = drawer.maybe_as_html() {
        html.focus().ok();
    }

    animate_with_class(&drawer, "show").await.ok();

    event::dispatch_custom(&drawer, event::AFTER_SHOW, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Requests to close the drawer. Dispatches a cancelable `wg-hide` carrying the
/// `source` element that triggered the request; when canceled the drawer stays
/// open and pulses instead.
async fn request_close(drawer: Element, source: Element) -> Option<()> {
    let dialog = dialog(&drawer)?;
    if !dialog.open() {
        return None;
    }

    let detail = Object::new();
    detail.set("source", source.clone());

    if !event::dispatch_custom(&drawer, event::HIDE, true, true, detail.as_ref()).unwrap_or(true) {
        // Closing was prevented: draw attention to the drawer.
        animate_with_class(&drawer, "pulse").await.ok();
        return None;
    }

    // Remove `.open` before the animation so the backdrop fades out with the panel.
    drawer.class_list().remove_1("open").ok();
    animate_with_class(&drawer, "hide").await.ok();

    dialog.close();
    update_body_scroll_lock();

    event::dispatch_custom(&drawer, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

fn open_drawer(drawer: Element) {
    spawn_local(async move {
        show(drawer).await;
    });
}

fn close_drawer(drawer: Element, source: Element) {
    spawn_local(async move {
        request_close(drawer, source).await;
    });
}

/// Resolves a `data-drawer` value: `"open <id>"` opens the drawer with that id,
/// `"close"` closes the enclosing drawer.
fn handle_drawer_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;

    if let Some(trigger) = target.closest("[data-drawer]").ok().flatten() {
        let value = trigger.get_attribute("data-drawer").unwrap_or_default();
        let mut tokens = value.split_whitespace();

        match tokens.next() {
            Some("open") => {
                if let Some(id) = tokens.next()
                    && let Some(drawer) = dom::existing::document().get_element_by_id(id)
                {
                    open_drawer(drawer);
                }
                return Some(());
            },
            Some("close") => {
                if let Some(drawer) = trigger.closest(".drawer").ok().flatten() {
                    close_drawer(drawer, trigger);
                }
                return Some(());
            },
            _ => {},
        }
    }

    // A click on the drawer element itself (not its content) is a backdrop click.
    if target.class_list().contains("drawer") && is_open(&target) {
        if target.has_attribute("data-light-dismiss") {
            close_drawer(target.clone(), target);
        } else {
            let drawer = target.clone();
            spawn_local(async move {
                animate_with_class(&drawer, "pulse").await.ok();
            });
        }
    }

    Some(())
}

/// [Escape] closes the top-most open drawer.
fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    if keyboard.key() != "Escape" {
        return None;
    }

    let drawers = dom::existing::document().query_selector_all(".drawer.open").ok()?;
    let drawer = (0..drawers.length())
        .rev()
        .find_map(|i| drawers.get(i).and_then(|node| node.maybe_into_element()))?;

    keyboard.prevent_default();
    keyboard.stop_propagation();
    close_drawer(drawer.clone(), drawer);

    Some(())
}

/// Shows any drawer rendered with the `data-open` attribute. Run after every render.
pub fn init_drawers() {
    let Ok(drawers) = dom::existing::document().query_selector_all(".drawer[data-open]") else {
        return;
    };
    for i in 0..drawers.length() {
        if let Some(drawer) = drawers.get(i).and_then(|node| node.maybe_into_element())
            && !is_open(&drawer)
        {
            open_drawer(drawer);
        }
    }
}

/// Installs the document-level listeners driving declarative open/close and
/// [Escape] dismissal for every drawer on the page.
pub fn listen_drawers() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_drawer_click(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });
}
