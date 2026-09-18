use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::existing::access::CastToHtmlElement;
use web_sys::HtmlInputElement;

/// Applies the indeterminate state of every checkbox rendered with
/// `data-indeterminate`. Run it after every render: a swap replaces the markup,
/// and the property doesn't survive it.
pub fn init_checkboxes() {
    for control in dom::existing::select_all_elements(".checkbox .control[data-indeterminate]") {
        if let Some(input) = control
            .maybe_into_html()
            .and_then(|html| html.dyn_into::<HtmlInputElement>().ok())
        {
            input.set_indeterminate(true);
        }
    }
}
