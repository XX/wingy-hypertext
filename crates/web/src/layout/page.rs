use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use wasm_dom as dom;
use wasm_dom::existing::access::CastToHtmlElement;
use web_sys::{Element, ResizeObserver, ResizeObserverEntry, ResizeObserverSize};

use crate::layout::drawer;
use crate::util::convert;

/// The width below which the page switches to its mobile layout, unless
/// `data-mobile-breakpoint` on the page says otherwise. Kept in step with
/// `wingy_hypertext::layout::page::DEFAULT_MOBILE_BREAKPOINT`.
const DEFAULT_MOBILE_BREAKPOINT: f64 = 920.0;

const DESKTOP: &str = "desktop";
const MOBILE: &str = "mobile";

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

/// Resolves `data-mobile-breakpoint` to pixels. Bare numbers and `px` are taken as
/// is; `rem` and `em` are resolved against the root font size, as in `wa-page`.
fn mobile_breakpoint(page: &Element) -> f64 {
    let Some(raw) = page.get_attribute("data-mobile-breakpoint") else {
        return DEFAULT_MOBILE_BREAKPOINT;
    };

    let raw = raw.trim();
    let value = convert::parse_float(raw);

    if raw.ends_with("rem") || raw.ends_with("em") {
        value * root_font_size()
    } else {
        value
    }
}

fn root_font_size() -> f64 {
    dom::existing::window()
        .get_computed_style(&dom::existing::document_element())
        .ok()
        .flatten()
        .and_then(|style| style.get_property_value("font-size").ok())
        .map(|size| convert::parse_float(&size))
        .filter(|size| *size > 0.0)
        .unwrap_or(16.0)
}

/// Moves the menu content into the navigation drawer, or back out of it.
///
/// The light-DOM stand-in for the slot reassignment `wa-page` does in its shadow
/// root: the navigation exists once, so it is relocated rather than duplicated.
/// That keeps ids unique and leaves listeners — htmx's included — attached, since
/// they live on the nodes being moved.
pub fn move_navigation(page: &Element, into_drawer: bool) -> Option<()> {
    let drawer_element = page.query_selector(".page-navigation-drawer").ok().flatten()?;
    let drawer_body = drawer_element.query_selector(".drawer-body").ok().flatten()?;
    let menu = page.query_selector(".page-menu").ok().flatten()?;

    let (from, to) = if into_drawer {
        (&menu, &drawer_body)
    } else {
        (&drawer_body, &menu)
    };

    while let Some(child) = from.first_child() {
        to.append_child(&child).ok()?;
    }

    // The drawer is modal, so leaving it open on desktop would lock the page behind
    // a panel that the layout no longer shows.
    if !into_drawer && drawer::is_open(&drawer_element) {
        drawer::close_drawer(drawer_element.clone(), drawer_element);
    }

    Some(())
}

/// Reflects the page's own width into the `view` attribute, and moves the
/// navigation to match.
///
/// The layout itself is handled by the container query the page renders, so this is
/// not what makes the page look right — it is what makes the navigation *be* in the
/// right place, which only script can do.
pub fn apply_view(page: &Element, width: f64) {
    let view = if width >= mobile_breakpoint(page) {
        DESKTOP
    } else {
        MOBILE
    };

    if page.get_attribute("view").as_deref() == Some(view) {
        return;
    }

    page.set_attribute("view", view).ok();
    move_navigation(page, view == MOBILE);
}

/// Watches the page element, the same box the container query measures, so the two
/// switch together.
pub fn observe_view(page: &Element) -> Option<()> {
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
        for entry in entries.iter() {
            let Ok(entry) = entry.dyn_into::<ResizeObserverEntry>() else {
                continue;
            };
            let Some(target) = entry.target().dyn_into::<Element>().ok() else {
                continue;
            };
            let Some(size) = entry
                .border_box_size()
                .get(0)
                .dyn_into::<ResizeObserverSize>()
                .ok()
                .map(|size| size.inline_size())
            else {
                continue;
            };

            apply_view(&target, size);
        }
    });

    let observer = ResizeObserver::new(callback.as_ref().unchecked_ref()).ok()?;
    observer.observe(page);

    // The observer has to outlive this call; it lives as long as the page does.
    callback.forget();

    Some(())
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
        observe_view(&page)?;
    }

    Some(())
}
