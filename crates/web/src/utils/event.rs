use wasm_dom::JsValue;
use web_sys::{Element, Event, EventInit};

pub const CLOSE: &str = "wg-close";
pub const REMOVE: &str = "wg-remove";

pub fn dispatch(element: &Element, event_type: &str, bubbles: bool) -> Result<bool, JsValue> {
    let init = EventInit::new();
    init.set_bubbles(bubbles);

    let event = Event::new_with_event_init_dict(event_type, &init)?;
    element.dispatch_event(&event)
}
