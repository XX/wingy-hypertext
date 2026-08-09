use wasm_dom as dom;
use wasm_dom::existing::access::CastToHtmlElement;

/// Publishes the page header height as the `--page-header-height` custom property.
pub fn set_page_header_height() -> Option<()> {
    let document = dom::existing::document();

    let header = document
        .query_selector(".page-header")
        .ok()
        .flatten()
        .or_else(|| document.query_selector(".page > header").ok().flatten())?
        .into_html();

    document
        .document_element()?
        .into_html()
        .style()
        .set_property("--page-header-height", &format!("{}px", header.offset_height()))
        .ok()
}

pub fn init_page() -> Option<()> {
    set_page_header_height()
}

/// Initializes the `.page` element once, guarded by a `data-initialized` marker.
pub fn init_page_element() -> Option<()> {
    let page = dom::select_element(".page").ok()?;

    if page.get_attribute("data-initialized").is_none() {
        page.set_attribute("data-initialized", "true").ok()?;
        init_page()?;
    }

    Some(())
}
