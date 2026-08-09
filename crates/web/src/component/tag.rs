//! The remove behavior of `Tag`: activating the remove button dispatches a
//! bubbling `wg-remove` event on the `.tag` element. The tag does not remove
//! itself — the consumer handles the event and decides.

use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::Event;

use crate::util::event;

pub fn handle_remove_tag(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let remove_button = target.closest(".tag-remove").ok()??;
    let tag = remove_button.closest(".tag").ok()??;

    event::dispatch(&tag, event::REMOVE, true).ok().map(drop)
}

/// Installs the document-level listener that emits `wg-remove` for every
/// removable tag on the page.
pub fn listen_remove_tags() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_remove_tag(&event);
    });
}
