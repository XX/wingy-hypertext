//! The color scheme selector of the top app bar: an icon button opening a menu
//! with the light, dark and system schemes, like the one in the Web Awesome docs.

use const_format::concatcp;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Event};
use wingy_hypertext::class::{CHECKED, COLOR_SCHEME_SELECTOR, DROPDOWN_ITEM};
use wingy_hypertext::component::color_scheme_selector::ColorScheme;

use crate::util::color_scheme::{color_scheme, listen_color_scheme, set_color_scheme};
use crate::util::event;

/// Applies the stored color scheme, checks the matching item of the selector
/// and switches the scheme on selection.
pub fn listen_color_scheme_selector() {
    listen_color_scheme();
    sync_selector(color_scheme());

    dom::existing::document().add_steady_event_listener(event::SELECT, |event| {
        handle_select(&event);
    });
}

/// Checks the item of the stored scheme in selectors added after
/// [`listen_color_scheme_selector`], e.g. by an htmx swap.
pub fn init_color_scheme_selectors() {
    sync_selector(color_scheme());
}

fn handle_select(event: &Event) -> Option<()> {
    let dropdown = event.target()?.maybe_into_element()?;
    if !dropdown.class_list().contains(COLOR_SCHEME_SELECTOR) {
        return None;
    }

    let custom: &CustomEvent = event.dyn_ref()?;
    let value = custom.detail().get("value").as_string()?;
    let scheme = ColorScheme::parse(&value)?;

    set_color_scheme(scheme);
    sync_selector(scheme);

    Some(())
}

/// Checks the item of `scheme` only: selecting a checkable item toggles it, so
/// the selected one is re-checked and the others are unchecked.
fn sync_selector(scheme: ColorScheme) {
    const SELECTOR: &str = concatcp!(
        '.',
        COLOR_SCHEME_SELECTOR,
        " .",
        DROPDOWN_ITEM,
        "[role=menuitemcheckbox]"
    );

    for item in dom::existing::select_all_elements(SELECTOR) {
        let checked = item.get_attribute("data-value").as_deref() == Some(scheme.as_ref());
        item.class_list().toggle_with_force(CHECKED, checked).ok();
        item.set_attribute("aria-checked", if checked { "true" } else { "false" })
            .ok();
    }
}
