use wasm_dom::JsValue;
use web_sys::{CustomEvent, CustomEventInit, Element, Event, EventInit};

pub const CLOSE: &str = "wg-close";
pub const REMOVE: &str = "wg-remove";
pub const SHOW: &str = "wg-show";
pub const HIDE: &str = "wg-hide";
pub const AFTER_SHOW: &str = "wg-after-show";
pub const AFTER_HIDE: &str = "wg-after-hide";
pub const SELECT: &str = "wg-select";
/// Dispatched by an accordion about one of its items, as `wa-accordion` does.
pub const EXPAND: &str = "wg-expand";
pub const AFTER_EXPAND: &str = "wg-after-expand";
pub const COLLAPSE: &str = "wg-collapse";
pub const AFTER_COLLAPSE: &str = "wg-after-collapse";
/// Reported by the slider while its value changes, and once it settles —
/// the native names of a form control, as `wa-slider` uses them.
pub const INPUT: &str = "input";
pub const CHANGE: &str = "change";
/// Reported by the rating while the pointer previews a value, as `wa-hover` of
/// `wa-rating`: the detail carries the `phase` (`start`, `move` or `end`) and
/// the `value`.
pub const HOVER: &str = "wg-hover";
pub const COLOR_SCHEME_CHANGE: &str = "wg-color-scheme-change";

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
