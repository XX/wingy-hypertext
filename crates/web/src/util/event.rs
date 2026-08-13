use wasm_dom::JsValue;
use web_sys::{CustomEvent, CustomEventInit, Element, Event, EventInit};

pub const CLOSE: &str = "wg-close";
pub const REMOVE: &str = "wg-remove";
pub const SHOW: &str = "wg-show";
pub const HIDE: &str = "wg-hide";
pub const AFTER_SHOW: &str = "wg-after-show";
pub const AFTER_HIDE: &str = "wg-after-hide";
pub const SELECT: &str = "wg-select";

pub fn dispatch(element: &Element, event_type: &str, bubbles: bool) -> Result<bool, JsValue> {
    let init = EventInit::new();
    init.set_bubbles(bubbles);

    let event = Event::new_with_event_init_dict(event_type, &init)?;
    element.dispatch_event(&event)
}

/// Dispatches a `CustomEvent` carrying `detail`. Returns `true` when the event
/// was *not* canceled (no listener called `preventDefault`), matching the sense
/// of `EventTarget.dispatchEvent`.
pub fn dispatch_custom(
    element: &Element,
    event_type: &str,
    bubbles: bool,
    cancelable: bool,
    detail: &JsValue,
) -> Result<bool, JsValue> {
    let init = CustomEventInit::new();
    init.set_bubbles(bubbles);
    init.set_cancelable(cancelable);
    init.set_detail(detail);

    let event = CustomEvent::new_with_event_init_dict(event_type, &init)?;
    element.dispatch_event(&event)
}
