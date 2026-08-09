use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::Event;

use crate::util::event;

pub fn handle_close_callout(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let close_element = target.closest(".close").ok()??;
    let callout = close_element.closest(".callout").ok()??;

    event::dispatch(&callout, event::CLOSE, true).ok().map(drop)
}

pub fn listen_close_callout() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_close_callout(&event);
    });
}
