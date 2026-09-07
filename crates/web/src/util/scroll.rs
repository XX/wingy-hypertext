//! Body scroll locking that doesn't shift the page.
//!
//! Hiding the document's overflow removes the scrollbar, and with nothing
//! reserving its width the content jumps sideways — visibly so in browsers
//! with classic scrollbars. The `.wa-scroll-lock` utility in
//! `webassets/style/common/utilities/scroll-lock.css` keeps the gutter;
//! this module measures the scrollbar and drives that class.
//!
//! `--wa-scroll-lock-gutter` is deliberately left unset, which keeps that
//! utility's `scrollbar-gutter` branch inert: a gutter on `<html>` sits outside
//! the root's content box, and therefore outside a modal's `::backdrop`, leaving
//! an undimmed stripe beside the drawer. The width is compensated with body
//! padding instead — see `webassets/style/utils/scroll_lock.css`, which pairs
//! with the `--wa-scroll-lock-size` set here.

use wasm_dom as dom;
use wasm_dom::existing::access::CastToHtmlElement;

/// Set on `<html>` while the lock is held; the utility stylesheet does the rest.
pub const SCROLL_LOCK: &str = "wa-scroll-lock";
pub const SIZE: &str = "--wa-scroll-lock-size";

/// The width of the document's scrollbar, in CSS pixels.
pub fn scrollbar_width() -> f64 {
    let document_width = f64::from(dom::existing::document_element().client_width());
    let inner_width = dom::existing::window()
        .inner_width()
        .ok()
        .and_then(|width| width.as_f64())
        .unwrap_or(document_width);

    (inner_width - document_width).abs()
}

/// Padding already on `<body>`, which the compensation replaces outright and so
/// has to carry over.
pub fn existing_body_padding() -> f64 {
    dom::existing::window()
        .get_computed_style(&dom::existing::body())
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("padding-right").ok())
        .and_then(|padding| padding.trim_end_matches("px").parse::<f64>().ok())
        .filter(|padding| padding.is_finite())
        .unwrap_or(0.0)
}

/// Locks or unlocks page scrolling.
///
/// The caller decides whether a lock is wanted — the state is derived from the
/// DOM, so there is no lock registry to keep in sync. Repeated calls are cheap:
/// the scrollbar is measured only when the lock is first taken, because once the
/// overflow is hidden the scrollbar is gone and would measure zero.
pub fn set_body_scroll_lock(locked: bool) {
    let root = dom::existing::document_element();
    let classes = root.class_list();

    if locked {
        if classes.contains(SCROLL_LOCK) {
            return;
        }

        // Measured before the class lands, while the scrollbar is still there.
        let width = scrollbar_width() + existing_body_padding();

        if let Some(html) = root.maybe_as_html() {
            html.style().set_property(SIZE, &format!("{width}px")).ok();
        }

        classes.add_1(SCROLL_LOCK).ok();
    } else {
        classes.remove_1(SCROLL_LOCK).ok();

        if let Some(html) = root.maybe_as_html() {
            html.style().remove_property(SIZE).ok();
        }
    }
}
