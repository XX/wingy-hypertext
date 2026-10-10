//! Moving focus without moving the page.
//!
//! A plain `focus()` scrolls the page until the element is fully in view,
//! honoring `scroll-margin-top` — so focusing a control that is partly hidden
//! under the sticky header scrolls it down to the middle of the page. Web
//! Awesome focuses with `preventScroll` and scrolls only the popup's own
//! container where needed (`internal/scroll.ts`); these helpers do the same.

use wasm_bindgen::JsCast;
use web_sys::{Element, FocusOptions, HtmlElement, ScrollBehavior, ScrollToOptions};

/// Focuses `element` without scrolling the page.
pub fn focus_without_scroll(element: &HtmlElement) {
    let options = FocusOptions::new();
    options.set_prevent_scroll(true);
    element.focus_with_options(&options).ok();
}

/// Scrolls `container` vertically just enough to show `element`, leaving the
/// page and any other ancestor where they are (the `scrollIntoView` of
/// `internal/scroll.ts`, vertical only).
pub fn scroll_into_view_within(element: &Element, container: &Element) {
    let scroll_top = container.scroll_top();
    let offset_top =
        (element.get_bounding_client_rect().top() - container.get_bounding_client_rect().top()).round() + scroll_top;
    let container_height = f64::from(container.dyn_ref::<HtmlElement>().map_or(0, HtmlElement::offset_height));
    let element_height = f64::from(element.client_height());

    let top = if offset_top < scroll_top {
        offset_top
    } else if offset_top + element_height > scroll_top + container_height {
        offset_top - container_height + element_height
    } else {
        return;
    };

    let options = ScrollToOptions::new();
    options.set_top(top);
    options.set_behavior(ScrollBehavior::Auto);
    container.scroll_to_with_scroll_to_options(&options);
}
