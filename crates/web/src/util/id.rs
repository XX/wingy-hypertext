use std::cell::Cell;

use wasm_dom as dom;
use web_sys::Element;

thread_local! {
    static NEXT_ID: Cell<u32> = const { Cell::new(0) };
}

/// Gives an element an id unique on the page, unless it already has one, and
/// returns the id. Used where the markup refers to an element by id (ARIA
/// relations) but the server could not render one.
pub fn ensure_id(element: &Element, prefix: &str) -> String {
    let id = element.id();
    if !id.is_empty() {
        return id;
    }

    let document = dom::existing::document();
    let id = NEXT_ID.with(|next| {
        loop {
            let id = format!("{prefix}-{}", next.get());
            next.set(next.get() + 1);
            if document.get_element_by_id(&id).is_none() {
                return id;
            }
        }
    });

    element.set_id(&id);
    id
}
