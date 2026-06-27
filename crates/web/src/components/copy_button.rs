//! Port of `webassets/js/components/copy_button.js`.

use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::{JsFuture, spawn_local};
use web_sys::{Element, Event};

use crate::dom::{document, set_hidden, set_timeout, window};
use crate::utils::action::{ActionCtx, register_action};
use crate::utils::animate::animate_with_class;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Success,
    Error,
}

/// Copies a value to the clipboard in response to a copy-button click, then flashes a status icon.
///
/// `from` mirrors the JS `from` argument: when present it selects the source by id, optionally with a
/// `.property` or `[attribute]` suffix; when absent the parent element's text content is copied.
pub async fn handle_copy(event: Event, from: Option<String>) {
    let target = event
        .target()
        .and_then(|t| t.dyn_into::<Element>().ok())
        .and_then(|el| el.closest(".copy-button").ok().flatten());
    let Some(target) = target else {
        return;
    };

    let (value_to_copy, element): (Option<String>, Option<Element>) = match &from {
        Some(from) => {
            let is_property = from.contains('.');
            let is_attribute = from.contains('[') && from.contains(']');

            let (id, field) = if is_property {
                split_once_owned(from.trim(), '.')
            } else if is_attribute {
                split_once_owned(from.trim().trim_end_matches(']'), '[')
            } else {
                (from.clone(), String::new())
            };

            // The JS prototype reads `window.getElementById` (always undefined); the lookup belongs
            // on `document`, so the Rust port resolves the element the way the feature intends.
            let element = document().get_element_by_id(&id);
            let value = element
                .as_ref()
                .map(|el| read_value(el, is_property, is_attribute, &field))
                // When no element is found, the value falls back to the raw `from`, as in JS.
                .or_else(|| Some(from.clone()));
            (value, element)
        },
        None => {
            let element = target.parent_node().and_then(|n| n.dyn_into::<Element>().ok());
            let value = element.as_ref().map(|el| el.text_content().unwrap_or_default());
            (value, element)
        },
    };

    if element.is_none() {
        show_status(&target, Status::Error).await;
    }

    let value = value_to_copy.unwrap_or_default();
    if value.is_empty() {
        show_status(&target, Status::Error).await;
        return;
    }

    match JsFuture::from(window().navigator().clipboard().write_text(&value)).await {
        Ok(_) => show_status(&target, Status::Success).await,
        Err(_) => show_status(&target, Status::Error).await,
    }
}

fn read_value(element: &Element, is_property: bool, is_attribute: bool, field: &str) -> String {
    if is_attribute {
        element.get_attribute(field).unwrap_or_default()
    } else if is_property {
        Reflect::get(element, &JsValue::from_str(field))
            .ok()
            .and_then(|v| v.as_string())
            .unwrap_or_default()
    } else {
        element.text_content().unwrap_or_default()
    }
}

async fn show_status(target: &Element, status: Status) {
    let copy_icon = target.query_selector(".copy-button-copy").ok().flatten();
    let success_icon = target.query_selector(".copy-button-success").ok().flatten();
    let error_icon = target.query_selector(".copy-button-error").ok().flatten();

    let icon_to_show = if status == Status::Success {
        success_icon.clone()
    } else {
        error_icon.clone()
    };

    if let Some(copy_icon) = copy_icon.clone() {
        animate_with_class(&copy_icon, "hide").await;
        set_hidden(&copy_icon, true);
        if let Some(icon) = &icon_to_show {
            set_hidden(icon, false);
            animate_with_class(icon, "show").await;
        }
    }

    set_timeout(1000, move || {
        spawn_local(async move {
            if let Some(copy_icon) = copy_icon {
                if let Some(icon) = &icon_to_show {
                    animate_with_class(icon, "hide").await;
                    set_hidden(icon, true);
                }
                set_hidden(&copy_icon, false);
                animate_with_class(&copy_icon, "show").await;
            }
        });
    });
}

/// Registers the `copy` action with the action registry.
pub fn register_copy_action() {
    register_action("copy", |args: &JsValue, ctx: &ActionCtx| {
        let from = Reflect::get(args, &JsValue::from_str("from"))
            .ok()
            .and_then(|v| v.as_string())
            .filter(|s| !s.is_empty());
        let event = ctx.event.clone();
        spawn_local(async move {
            handle_copy(event, from).await;
        });
    });
}

/// `str::split_once` returning owned halves, with the whole input as the head when the delimiter is absent.
fn split_once_owned(input: &str, delimiter: char) -> (String, String) {
    match input.split_once(delimiter) {
        Some((head, tail)) => (head.to_string(), tail.to_string()),
        None => (input.to_string(), String::new()),
    }
}
