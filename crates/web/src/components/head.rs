//! Port of `webassets/js/components/head.js`.

use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, ScrollBehavior, ScrollToOptions};

use crate::dom::{document, window};
use crate::utils::animate::parse_float;

/// Smoothly scrolls to the element referenced by the URL hash, or back to the top when there is none.
pub fn init_scroll_to_anchor() {
    let win = window();
    let hash = win.location().hash().unwrap_or_default();

    if hash.is_empty() {
        scroll_to_top();
        return;
    }

    let Some(target) = document().query_selector(&hash).ok().flatten() else {
        return;
    };

    let offset = document()
        .document_element()
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
        .map(|root| {
            root.style()
                .get_property_value("--page-header-height")
                .unwrap_or_default()
        })
        .map(|value| parse_float(&value))
        .unwrap_or(0.0);

    let element_pos = target.get_bounding_client_rect().top();
    let to_pos = element_pos + win.page_y_offset().unwrap_or(0.0) - offset;

    let options = ScrollToOptions::new();
    options.set_top(to_pos);
    options.set_behavior(ScrollBehavior::Smooth);
    win.scroll_to_with_scroll_to_options(&options);
}

fn scroll_to_top() {
    let options = ScrollToOptions::new();
    options.set_top(0.0);
    options.set_behavior(ScrollBehavior::Smooth);
    window().scroll_to_with_scroll_to_options(&options);
}
