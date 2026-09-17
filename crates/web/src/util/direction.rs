use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use wasm_dom as dom;
use web_sys::{Element, MutationObserver, MutationObserverInit};

/// Tells whether `element` is laid out right-to-left. The direction is resolved
/// on the element itself rather than on the document, since `dir` is inherited
/// and may be switched on any container (a code example preview, for one).
pub fn is_rtl(element: &Element) -> bool {
    // `:dir()` is the cheap path; browsers without it throw, so fall back to
    // the computed `direction`.
    element.matches(":dir(rtl)").unwrap_or_else(|_| {
        dom::existing::window()
            .get_computed_style(element)
            .ok()
            .flatten()
            .and_then(|style| style.get_property_value("direction").ok())
            .is_some_and(|direction| direction == "rtl")
    })
}

/// Runs `on_change` whenever a `dir` attribute changes anywhere in the document,
/// so direction-dependent layout (like popup positions) can be recomputed.
pub fn observe_direction_changes(mut on_change: impl FnMut() + 'static) -> Option<()> {
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |_: js_sys::Array| on_change());
    let observer = MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;

    let options = MutationObserverInit::new();
    options.set_attributes(true);
    options.set_subtree(true);
    options.set_attribute_filter(&js_sys::Array::of1(&"dir".into()));
    observer
        .observe_with_options(&dom::existing::document_element(), &options)
        .ok()?;

    // The observer lives as long as the page does.
    callback.forget();

    Some(())
}
