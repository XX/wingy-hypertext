//! Light/dark color scheme switching: the scheme the user picked (`light`, `dark`
//! or `auto` to follow the system) is kept in `localStorage`, and the dark one
//! is applied by toggling the `wa-dark` class on the document element.

use wasm_dom as dom;
use wasm_dom::event::EventListener;
use web_sys::MediaQueryList;
use wingy_hypertext::class::DARK;
use wingy_hypertext::component::color_scheme_selector::ColorScheme;

use crate::util::event;

/// The `localStorage` key holding the chosen scheme.
pub const STORAGE_KEY: &str = "color-scheme";

/// Resolves `Auto` against the system preference.
pub fn is_dark(color_scheme: ColorScheme) -> bool {
    match color_scheme {
        ColorScheme::Light => false,
        ColorScheme::Dark => true,
        ColorScheme::Auto => prefers_dark(),
    }
}

pub fn prefers_dark_query() -> Option<MediaQueryList> {
    dom::existing::window()
        .match_media("(prefers-color-scheme: dark)")
        .ok()
        .flatten()
}

/// Tells whether the system is set to a dark color scheme.
pub fn prefers_dark() -> bool {
    prefers_dark_query().is_some_and(|query| query.matches())
}

/// The scheme stored by [`set_color_scheme`], `Auto` when none is stored.
pub fn color_scheme() -> ColorScheme {
    dom::existing::local_storage()
        .and_then(|storage| storage.get_item(STORAGE_KEY).ok().flatten())
        .and_then(|value| ColorScheme::parse(&value))
        .unwrap_or_default()
}

/// Stores `scheme`, applies it to the document and dispatches a bubbling
/// `wg-color-scheme-change` event on the document element.
pub fn set_color_scheme(scheme: ColorScheme) {
    if let Some(storage) = dom::existing::local_storage() {
        storage.set_item(STORAGE_KEY, scheme.as_ref()).ok();
    }
    apply_color_scheme(scheme);

    event::dispatch(&dom::existing::document_element(), event::COLOR_SCHEME_CHANGE, true).ok();
}

/// Toggles the `wa-dark` class on the document element to match `scheme`.
pub fn apply_color_scheme(scheme: ColorScheme) {
    dom::existing::document_element()
        .class_list()
        .toggle_with_force(DARK, is_dark(scheme))
        .ok();
}

/// Applies the stored scheme and keeps an `auto` one in step with the system
/// preference.
pub fn listen_color_scheme() {
    apply_color_scheme(color_scheme());

    let Some(query) = prefers_dark_query() else {
        return;
    };

    query.add_steady_event_listener("change", |_| {
        let scheme = color_scheme();
        if scheme == ColorScheme::Auto {
            apply_color_scheme(scheme);
        }
    });
}
