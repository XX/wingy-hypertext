use wasm_dom::existing::access::CastToHtmlElement;
use wasm_dom::{self as dom};
use web_sys::{ScrollBehavior, ScrollToOptions, Window};

use crate::util::convert::parse_float;

/// Smoothly scrolls to the element referenced by the URL hash, or back to the top when there is none.
pub fn init_scroll_to_anchor() {
    let window = dom::existing::window();

    let hash = window.location().hash().unwrap_or_default();
    if hash.is_empty() {
        scroll_to(&window, 0.0);
        return;
    }

    let Ok(target) = dom::select_element(&hash) else {
        return;
    };

    let document_element = dom::existing::document_element().into_html();
    let offset = parse_float(
        document_element
            .style()
            .get_property_value("--page-header-height")
            .unwrap_or_default(),
    );

    let element_pos = target.get_bounding_client_rect().top();
    let to_pos = element_pos + window.page_y_offset().unwrap_or(0.0) - offset;

    scroll_to(&window, to_pos);
}

fn scroll_to(window: &Window, top: f64) {
    let options = ScrollToOptions::new();
    options.set_top(top);
    options.set_behavior(ScrollBehavior::Smooth);

    window.scroll_to_with_scroll_to_options(&options);
}
