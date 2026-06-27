//! Port of `webassets/js/layouts/page.js`.

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

use crate::dom::{document, query_selector};

/// Publishes the page header height as the `--page-header-height` custom property.
pub fn set_page_header_height() {
    let header = query_selector(".page-header").or_else(|| query_selector(".page > header"));

    let Some(header) = header.and_then(|h| h.dyn_into::<HtmlElement>().ok()) else {
        return;
    };

    if let Some(root) = document()
        .document_element()
        .and_then(|e| e.dyn_into::<HtmlElement>().ok())
    {
        let _ = root
            .style()
            .set_property("--page-header-height", &format!("{}px", header.offset_height()));
    }
}

pub fn init_page() {
    set_page_header_height();
}

/// Initializes the `.page` element once, guarded by a `data-initialized` marker.
pub fn init_page_element() {
    let Some(page) = query_selector(".page") else {
        return;
    };

    if page.get_attribute("data-initialized").is_none() {
        let _ = page.set_attribute("data-initialized", "true");
        init_page();
    }
}
