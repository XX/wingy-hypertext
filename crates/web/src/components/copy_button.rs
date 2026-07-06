use wasm_bindgen_futures::{JsFuture, spawn_local};
use wasm_dom as dom;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{Element, Event};

use crate::utils::action::register_action;
use crate::utils::animate::animate_with_class;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Status {
    Success,
    Error,
}

/// Copies a value to the clipboard in response to a copy-button click, then flashes a status icon.
///
/// `from`: when present it selects the source by id, optionally with a  `.property` or `[attribute]`
/// suffix; when absent the parent element's text content is copied.
pub async fn handle_copy(event: &Event, from: Option<String>) -> Option<()> {
    let target = event.target()?.maybe_into_element()?.closest(".copy-button").ok()??;

    let (value_to_copy, element): (String, Option<Element>) = match from.as_deref() {
        Some(from) => {
            let is_property = from.contains('.');
            let is_attribute = from.contains('[') && from.contains(']');

            let trimmed_from = from.trim();
            let (id, field) = if is_property {
                trimmed_from.split_once('.').unwrap_or((trimmed_from, ""))
            } else if is_attribute {
                trimmed_from
                    .trim_end_matches(']')
                    .split_once('[')
                    .unwrap_or((trimmed_from, ""))
            } else {
                (trimmed_from, "")
            };

            let element = dom::existing::document().get_element_by_id(id);
            let value = if let Some(element) = &element {
                read_value(element, is_property, is_attribute, field).unwrap_or_default()
            } else {
                // When no element is found, the value falls back to the raw `from`.
                from.into()
            };
            (value, element)
        },
        None => {
            let element = target.parent_node().and_then(|node| node.maybe_into_element());
            let value = element
                .as_ref()
                .and_then(|element| element.text_content())
                .unwrap_or_default();
            (value, element)
        },
    };

    if element.is_none() {
        show_status(&target, Status::Error).await;
    }

    if value_to_copy.is_empty() {
        show_status(&target, Status::Error).await;
        return None;
    }

    if JsFuture::from(
        dom::existing::window()
            .navigator()
            .clipboard()
            .write_text(&value_to_copy),
    )
    .await
    .is_ok()
    {
        show_status(&target, Status::Success).await;
        Some(())
    } else {
        show_status(&target, Status::Error).await;
        None
    }
}

fn read_value(element: &Element, is_property: bool, is_attribute: bool, field: &str) -> Option<String> {
    if is_attribute {
        element.get_attribute(field)
    } else if is_property {
        element.get(field).as_string()
    } else {
        element.text_content()
    }
}

async fn show_status(target: &Element, status: Status) {
    let copy_icon = target.query_selector(".copy-button-copy").ok().flatten();
    let success_icon = target.query_selector(".copy-button-success").ok().flatten();
    let error_icon = target.query_selector(".copy-button-error").ok().flatten();

    let icon_to_show = if status == Status::Success {
        success_icon
    } else {
        error_icon
    };

    if let Some(copy_icon) = &copy_icon {
        animate_with_class(copy_icon, "hide").await.ok();
        copy_icon.as_html().set_hidden(true);
        if let Some(icon) = &icon_to_show {
            icon.as_html().set_hidden(false);
            animate_with_class(icon, "show").await.ok();
        }
    }

    dom::set_timeout(
        move || {
            spawn_local(async move {
                if let Some(copy_icon) = &copy_icon {
                    if let Some(icon) = &icon_to_show {
                        animate_with_class(icon, "hide").await.ok();
                        icon.as_html().set_hidden(true);
                    }
                    copy_icon.as_html().set_hidden(false);
                    animate_with_class(copy_icon, "show").await.ok();
                }
            });
        },
        1000,
    )
    .ok();
}

/// Registers the `copy` action with the action registry.
pub fn register_copy_action() {
    register_action("copy", |args, ctx| {
        let from = args.get("from").as_string().filter(|from| !from.is_empty());
        let event = ctx.event.clone();

        spawn_local(async move {
            handle_copy(&event, from).await;
        });
    });
}
