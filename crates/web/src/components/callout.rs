use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Element, Event};

pub fn close_callout(callout: &Element) -> Option<()> {
    callout.remove();
    Some(())
}

pub fn handle_close_callout(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let close_element = target.closest(".close").ok()??;
    let callout = close_element.closest(".callout").ok()??;

    close_callout(&callout)
}

pub fn listen_close_callout() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_close_callout(&event);
    });
}
