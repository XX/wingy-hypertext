//! The remove behavior of `Tag`: activating the remove button dispatches a
//! bubbling `wg-remove` event on the `.tag` element. The tag does not remove
//! itself — the consumer handles the event and decides.

use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Element, Event, EventInit};

/// The name of the event emitted when the tag's remove button is activated.
pub const REMOVE_EVENT: &str = "wg-remove";

pub fn dispatch_remove_tag(tag: &Element) {
    let init = EventInit::new();
    init.set_bubbles(true);
    if let Ok(event) = Event::new_with_event_init_dict(REMOVE_EVENT, &init) {
        tag.dispatch_event(&event).ok();
    }
}

pub fn handle_remove_tag(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let remove_button = target.closest(".tag-remove").ok()??;
    let tag = remove_button.closest(".tag").ok()??;

    dispatch_remove_tag(&tag);
    Some(())
}

/// Installs the document-level listener that emits `wg-remove` for every
/// removable tag on the page.
pub fn listen_remove_tags() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_remove_tag(&event);
    });
}
