//! The behavior shared by the layouts rendered as a native modal `<dialog>`
//! (`Drawer`, `Dialog`): showing and hiding with animations, declarative
//! `data-<kind>="open <id>"` / `data-<kind>="close"` triggers, [Escape],
//! the native `cancel` request and light-dismiss handling, body scroll
//! locking, focus restoring, and the cancelable `wg-show`/`wg-hide` (plus
//! `wg-after-show`/`wg-after-hide`) lifecycle events.
//!
//! The logical state is the `open` class: it is set when the modal is shown
//! and dropped as soon as hiding starts, while the native `open` stays until
//! the hide animation ends.
//!
//! Modals of different kinds may be open on top of each other (a dialog
//! opened from a drawer), so [Escape] and the native `cancel` close the
//! top-most one only. The order is kept in a stack of the modals in the order
//! they were opened, like Web Awesome's `dismissible-stack`.

use std::cell::RefCell;

use const_format::concatcp;
use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{AddEventListenerOptions, Element, Event, HtmlElement, KeyboardEvent, MouseEvent};
use wingy_hypertext::class::{DIALOG, DRAWER, OPEN};

use crate::util::animate::animate_with_class;
use crate::util::class::is_open;
use crate::util::convert::dialog;
use crate::util::id::ensure_id;
use crate::util::{event, scroll};

/// Matches the open modals of every kind: page scrolling stays locked while any of them is open.
pub const OPEN_MODALS: &str = concatcp!('.', DRAWER, '.', OPEN, ", .", DIALOG, '.', OPEN);

/// A kind of modal layout: its class and the attribute of its declarative triggers.
pub struct ModalKind {
    /// The class of the `<dialog>` element, e.g. `drawer`.
    pub class: &'static str,
    /// Matches the modals of this kind, e.g. `.drawer`.
    pub selector: &'static str,
    /// The attribute of the declarative triggers, e.g. `data-drawer`.
    pub trigger: &'static str,
    /// Matches the declarative triggers, e.g. `[data-drawer]`.
    pub trigger_selector: &'static str,
    /// Matches the modal's own title, which labels it.
    pub title_selector: &'static str,
}

/// An open modal and the element focused before it opened, which gets the focus back on close.
struct Entry {
    modal: Element,
    trigger: Option<HtmlElement>,
}

thread_local! {
    static STACK: RefCell<Vec<Entry>> = const { RefCell::new(Vec::new()) };
}

fn push(modal: &Element, trigger: Option<HtmlElement>) {
    STACK.with_borrow_mut(|stack| {
        stack.retain(|entry| entry.modal != *modal);
        stack.push(Entry {
            modal: modal.clone(),
            trigger,
        });
    });
}

fn take(modal: &Element) -> Option<Entry> {
    STACK.with_borrow_mut(|stack| {
        let index = stack.iter().position(|entry| entry.modal == *modal)?;
        Some(stack.remove(index))
    })
}

/// Forgets the modals that are closed or no longer on the page, e.g. swapped
/// out by htmx while open.
fn prune() {
    STACK.with_borrow_mut(|stack| {
        stack.retain(|entry| entry.modal.is_connected() && is_open(&entry.modal));
    });
}

/// The modal opened last among the open ones, of any kind.
pub fn top_modal() -> Option<Element> {
    prune();
    STACK
        .with_borrow(|stack| stack.last().map(|entry| entry.modal.clone()))
        // Shown in some other way than `show`: fall back to the document order
        .or_else(|| dom::existing::select_all_elements(OPEN_MODALS).last())
}

/// Whether the native `<dialog>` is shown, including while its hide animation runs.
pub fn is_shown(modal: &Element) -> bool {
    dialog(modal).map(|dialog| dialog.open()).unwrap_or(false)
}

/// Locks or unlocks page scrolling depending on whether any modal is open.
pub fn update_body_scroll_lock() {
    let has_open = dom::existing::document()
        .query_selector(OPEN_MODALS)
        .ok()
        .flatten()
        .is_some();

    scroll::set_body_scroll_lock(has_open);
}

/// Shows the modal, animating it in unless the `wg-show` event is canceled.
pub async fn show(modal: Element) -> Option<()> {
    let dialog = dialog(&modal)?;
    if dialog.open() {
        return None;
    }

    // A canceled `wg-show` keeps the modal closed.
    if !event::dispatch_custom(&modal, event::SHOW, true, true, &JsValue::NULL).unwrap_or(true) {
        return None;
    }

    let trigger = dom::existing::document()
        .active_element()
        .and_then(|element| element.maybe_into_html());

    dialog.show_modal().ok();
    modal.class_list().add_1(OPEN).ok();
    push(&modal, trigger);
    update_body_scroll_lock();

    // Move focus to an element marked `autofocus`, or the modal itself.
    if let Some(autofocus) = modal
        .query_selector("[autofocus]")
        .ok()
        .flatten()
        .and_then(|element| element.maybe_into_html())
    {
        autofocus.focus().ok();
    } else if let Some(html) = modal.maybe_as_html() {
        html.focus().ok();
    }

    animate_with_class(&modal, "show").await.ok();

    event::dispatch_custom(&modal, event::AFTER_SHOW, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Requests to close the modal. Dispatches a cancelable `wg-hide` carrying the
/// `source` element that triggered the request; when canceled the modal stays
/// open and pulses instead.
pub async fn request_close(modal: Element, source: Element) -> Option<()> {
    let dialog = dialog(&modal)?;

    // Not open, or already hiding.
    if !dialog.open() || !is_open(&modal) {
        return None;
    }

    let detail = Object::new();
    detail.set("source", source.clone());

    if !event::dispatch_custom(&modal, event::HIDE, true, true, detail.as_ref()).unwrap_or(true) {
        // Closing was prevented: draw attention to the modal.
        animate_with_class(&modal, "pulse").await.ok();
        return None;
    }

    // Remove `.open` before the animation so the backdrop fades out with the panel.
    modal.class_list().remove_1(OPEN).ok();
    let entry = take(&modal);

    animate_with_class(&modal, "hide").await.ok();

    dialog.close();
    update_body_scroll_lock();
    restore_focus(entry);

    event::dispatch_custom(&modal, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Gives the focus back to the element focused before the modal opened.
fn restore_focus(entry: Option<Entry>) {
    if let Some(trigger) = entry.and_then(|entry| entry.trigger)
        && trigger.is_connected()
    {
        trigger.focus().ok();
    }
}

pub fn open_modal(modal: Element) {
    spawn_local(async move {
        show(modal).await;
    });
}

pub fn close_modal(modal: Element, source: Element) {
    spawn_local(async move {
        request_close(modal, source).await;
    });
}

/// Resolves a trigger of this kind: `"open <id>"` opens the modal with that
/// id, `"close"` closes the enclosing modal.
fn handle_trigger_click(kind: &ModalKind, event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let trigger = target.closest(kind.trigger_selector).ok()??;

    let value = trigger.get_attribute(kind.trigger).unwrap_or_default();
    let mut tokens = value.split_whitespace();

    match tokens.next() {
        Some("open") => {
            if let Some(id) = tokens.next()
                && let Some(modal) = dom::existing::document().get_element_by_id(id)
                && modal.class_list().contains(kind.class)
            {
                open_modal(modal);
            }
        },
        Some("close") => {
            if let Some(modal) = trigger.closest(kind.selector).ok().flatten() {
                close_modal(modal, trigger);
            }
        },
        _ => {},
    }

    Some(())
}

/// A pointer going down on the backdrop closes a light-dismiss modal, and
/// pulses any other one. The backdrop belongs to the `<dialog>` element, so it
/// is told apart from the modal's own box (padding, scrollbar) by the position.
fn handle_backdrop_pointerdown(kind: &ModalKind, event: &Event) -> Option<()> {
    let mouse: &MouseEvent = event.dyn_ref()?;
    let modal = event.target()?.maybe_into_element()?;

    if !modal.class_list().contains(kind.class) || !is_open(&modal) {
        return None;
    }

    let rect = modal.get_bounding_client_rect();
    let (x, y) = (mouse.client_x(), mouse.client_y());
    let inside = x >= rect.left() && x <= rect.right() && y >= rect.top() && y <= rect.bottom();
    if inside {
        return None;
    }

    if modal.has_attribute("data-light-dismiss") {
        close_modal(modal.clone(), modal);
    } else {
        spawn_local(async move {
            animate_with_class(&modal, "pulse").await.ok();
        });
    }

    Some(())
}

/// [Escape] closes the top-most open modal when it is of this kind. A key
/// already handled by something inside the modal (a dropdown, a select, a
/// tooltip) is left alone: those cancel the event when they close on it.
fn handle_keydown(kind: &ModalKind, event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    if keyboard.key() != "Escape" || keyboard.default_prevented() {
        return None;
    }

    let modal = top_modal()?;
    if !modal.class_list().contains(kind.class) {
        return None;
    }

    // Canceling the key also keeps the browser from closing the `<dialog>` itself.
    keyboard.prevent_default();
    keyboard.stop_propagation();
    close_modal(modal.clone(), modal);

    Some(())
}

/// The browser requests to close the modal on its own, e.g. with the Android
/// back gesture: turn it into an animated close with the `wg-hide` event.
fn handle_cancel(kind: &ModalKind, event: &Event) -> Option<()> {
    let modal = event.target()?.maybe_into_element()?;
    if !modal.class_list().contains(kind.class) {
        return None;
    }

    event.prevent_default();

    if top_modal().is_some_and(|top| top == modal) {
        close_modal(modal.clone(), modal);
    }

    Some(())
}

/// The browser may close the modal without a cancelable `cancel` (it doesn't
/// let a page keep a dialog open forever): sync the state with that.
fn handle_close(kind: &ModalKind, event: &Event) -> Option<()> {
    let modal = event.target()?.maybe_into_element()?;
    if !modal.class_list().contains(kind.class) || !is_open(&modal) {
        return None;
    }

    modal.class_list().remove_1(OPEN).ok();
    let entry = take(&modal);
    update_body_scroll_lock();
    restore_focus(entry);

    event::dispatch_custom(&modal, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Labels the modal by its title, unless the markup labels it already.
fn label_by_title(kind: &ModalKind, modal: &Element) -> Option<()> {
    if modal.has_attribute("aria-labelledby") || modal.has_attribute("aria-label") {
        return None;
    }

    let title = modal.query_selector(kind.title_selector).ok()??;
    let id = ensure_id(&title, &format!("{}-title", kind.class));
    modal.set_attribute("aria-labelledby", &id).ok();

    Some(())
}

/// Labels the modals of this kind by their titles and shows the ones rendered
/// with the `data-open` attribute. Run it after every render.
pub fn init_modals(kind: &ModalKind) {
    // A swap may have removed an open modal along with the lock it held.
    prune();
    update_body_scroll_lock();

    let modals: Vec<_> = dom::existing::select_all_elements(kind.selector).collect();
    for modal in &modals {
        label_by_title(kind, modal);
    }

    spawn_local(async move {
        for modal in modals {
            if modal.has_attribute("data-open") && !is_shown(&modal) {
                show(modal).await;
            }
        }
    });
}

/// Installs the document-level listeners driving the modals of this kind:
/// declarative open/close, light dismiss, [Escape] and the native close requests.
pub fn listen_modals(kind: &'static ModalKind) {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_trigger_click(kind, &event);
    });
    document.add_steady_event_listener("pointerdown", |event| {
        handle_backdrop_pointerdown(kind, &event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(kind, &event);
    });

    // `cancel` and `close` don't bubble, but they do capture.
    let options = AddEventListenerOptions::new();
    options.set_capture(true);
    document.add_steady_event_listener_with_options(
        "cancel",
        |event| {
            handle_cancel(kind, &event);
        },
        &options,
    );
    document.add_steady_event_listener_with_options(
        "close",
        |event| {
            handle_close(kind, &event);
        },
        &options,
    );
}
