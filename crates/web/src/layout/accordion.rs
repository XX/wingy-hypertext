//! An `Accordion` behavior: expanding and collapsing its items with a height
//! animation, the `multiple`/`single`/`single-collapsible` modes, moving the
//! focus between the item triggers with the arrows, [Home] and [End], and the
//! cancelable `wg-expand`/`wg-collapse` (plus `wg-after-expand`/
//! `wg-after-collapse`) events, dispatched on the accordion with the item in
//! `detail.item`. The state lives entirely in the DOM, matching the markup
//! produced by `wingy_hypertext::layout::accordion`.
//!
//! The logical state is the `expanded` class on the item. A collapsed panel is
//! `hidden="until-found"`, set only once the collapse animation ends; a search
//! in the page reveals it on its own, which expands the item (`beforematch`).
//! The height animation is the shared [`crate::util::collapse`] one.

use const_format::concatcp;
use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{AddEventListenerOptions, Element, Event, HtmlElement, KeyboardEvent};
use wingy_hypertext::class::{
    ACCORDION, ACCORDION_ITEM, ACCORDION_ITEM_HEADING, ACCORDION_ITEM_PANEL, ACCORDION_ITEM_TRIGGER, EXPANDED,
};

pub use crate::util::class::is_disabled;
use crate::util::collapse::{animate_body, bump_generation, reset_body};
use crate::util::convert::bool_to_str;
use crate::util::event;
use crate::util::id::ensure_id;

/// The value of `hidden` on a collapsed panel.
const UNTIL_FOUND: &str = "until-found";

const ITEMS: &str = concatcp!(":scope > .", ACCORDION_ITEM);
const TRIGGER: &str = concatcp!(
    ":scope > .",
    ACCORDION_ITEM_HEADING,
    " > .",
    ACCORDION_ITEM_TRIGGER,
    ", :scope > .",
    ACCORDION_ITEM_TRIGGER,
);
const PANEL: &str = concatcp!(":scope > .", ACCORDION_ITEM_PANEL);

/// How the items of an accordion can be expanded, read from its `data-mode`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Mode {
    Multiple,
    Single,
    SingleCollapsible,
}

impl Mode {
    pub fn of(accordion: &Element) -> Self {
        match accordion.get_attribute("data-mode").as_deref() {
            Some("single") => Self::Single,
            Some("single-collapsible") => Self::SingleCollapsible,
            _ => Self::Multiple,
        }
    }

    pub fn is_single(self) -> bool {
        self != Self::Multiple
    }
}

pub fn all_items() -> impl Iterator<Item = Element> {
    dom::existing::select_all_elements(concatcp!('.', ACCORDION_ITEM))
}

/// The accordion's own items, not the ones of an accordion nested in them.
pub fn items(accordion: &Element) -> Vec<Element> {
    dom::existing::select_all_elements_from(accordion, ITEMS).collect()
}

/// The accordion the item belongs to.
pub fn accordion_of(item: &Element) -> Option<Element> {
    item.parent_element()
        .filter(|accordion| accordion.class_list().contains(ACCORDION))
}

pub fn trigger(item: &Element) -> Option<HtmlElement> {
    item.query_selector(TRIGGER).ok()??.maybe_into_html()
}

pub fn panel(item: &Element) -> Option<HtmlElement> {
    item.query_selector(PANEL).ok()??.maybe_into_html()
}

/// The item whose trigger is `trigger`, or `None` for any other element.
fn item_of_trigger(trigger: &Element) -> Option<Element> {
    if !trigger.class_list().contains(ACCORDION_ITEM_TRIGGER) {
        return None;
    }

    let parent = trigger.parent_element()?;
    let item = if parent.class_list().contains(ACCORDION_ITEM_HEADING) {
        parent.parent_element()?
    } else {
        parent
    };

    item.class_list().contains(ACCORDION_ITEM).then_some(item)
}

pub fn is_expanded(item: &Element) -> bool {
    item.class_list().contains(EXPANDED)
}

/// Sets the logical state: the class the styles follow and the trigger's `aria-expanded`.
fn set_expanded(item: &Element, expanded: bool) {
    item.class_list().toggle_with_force(EXPANDED, expanded).ok();
    if let Some(trigger) = trigger(item) {
        trigger.set_attribute("aria-expanded", bool_to_str(expanded)).ok();
    }
}

fn set_panel_hidden(panel: &HtmlElement, hidden: bool) {
    if hidden {
        panel.set_attribute("hidden", UNTIL_FOUND).ok();
    } else {
        panel.remove_attribute("hidden").ok();
    }
}

/// Dispatches an accordion event about the item on its accordion (on the item
/// itself when it is outside of one). Returns `false` when it was canceled.
fn dispatch(item: &Element, event_type: &str, cancelable: bool) -> bool {
    let target = accordion_of(item).unwrap_or_else(|| item.clone());
    let detail = Object::new();
    detail.set("item", item.clone());

    event::dispatch_custom(&target, event_type, true, cancelable, detail.as_ref()).unwrap_or(true)
}

/// Expands the item with an animation. With `notify`, the cancelable
/// `wg-expand` goes first and `wg-after-expand` follows; in a single mode the
/// other items of the accordion collapse.
pub async fn expand(item: Element, notify: bool) -> Option<()> {
    if is_expanded(&item) || is_disabled(&item) {
        return None;
    }

    let panel = panel(&item)?;

    if notify && !dispatch(&item, event::EXPAND, true) {
        return None;
    }

    if let Some(accordion) = accordion_of(&item)
        && Mode::of(&accordion).is_single()
    {
        collapse_others(&accordion, &item);
    }

    let generation = bump_generation(&item);
    set_expanded(&item, true);
    set_panel_hidden(&panel, false);

    if !animate_body(&item, &panel, true, generation).await {
        return None;
    }
    reset_body(&panel);

    if notify {
        dispatch(&item, event::AFTER_EXPAND, false);
    }

    Some(())
}

/// Collapses the item with an animation. With `notify`, the cancelable
/// `wg-collapse` goes first and `wg-after-collapse` follows.
pub async fn collapse(item: Element, notify: bool) -> Option<()> {
    if !is_expanded(&item) || is_disabled(&item) {
        return None;
    }

    let panel = panel(&item)?;

    if notify && !dispatch(&item, event::COLLAPSE, true) {
        return None;
    }

    let generation = bump_generation(&item);
    set_expanded(&item, false);

    if !animate_body(&item, &panel, false, generation).await {
        return None;
    }
    set_panel_hidden(&panel, true);
    reset_body(&panel);

    if notify {
        dispatch(&item, event::AFTER_COLLAPSE, false);
    }

    Some(())
}

/// Collapses the other expanded items of the accordion, as Web Awesome does
/// in a single mode: without events of their own.
fn collapse_others(accordion: &Element, item: &Element) {
    for other in items(accordion) {
        if other != *item && is_expanded(&other) {
            spawn_local(async move {
                collapse(other, false).await;
            });
        }
    }
}

/// Toggles the item as its trigger does: the expanded item of a `single`
/// accordion stays expanded.
pub fn toggle_item(item: &Element) {
    if is_disabled(item) {
        return;
    }

    let item = item.clone();
    if is_expanded(&item) {
        let single = accordion_of(&item).is_some_and(|accordion| Mode::of(&accordion) == Mode::Single);
        if !single {
            spawn_local(async move {
                collapse(item, true).await;
            });
        }
    } else {
        spawn_local(async move {
            expand(item, true).await;
        });
    }
}

/// Expands every enabled item of the accordion; nothing in a single mode.
pub fn expand_all(accordion: &Element) {
    if Mode::of(accordion).is_single() {
        return;
    }

    for item in items(accordion) {
        spawn_local(async move {
            expand(item, false).await;
        });
    }
}

/// Collapses every expanded item of the accordion.
pub fn collapse_all(accordion: &Element) {
    for item in items(accordion) {
        spawn_local(async move {
            collapse(item, false).await;
        });
    }
}

/// A click on a trigger toggles its item.
pub fn handle_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let trigger = target.closest(concatcp!('.', ACCORDION_ITEM_TRIGGER)).ok()??;
    let item = item_of_trigger(&trigger)?;

    toggle_item(&item);

    Some(())
}

/// [Enter] and [Space] toggle the item, as `wa-accordion-item` does: the key
/// is canceled, so the button doesn't click on top of it. The arrows move the
/// focus to the previous or next enabled item of the accordion, wrapping
/// around; [Home] and [End] to the first and the last.
pub fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    let trigger = event.target()?.maybe_into_element()?;
    let item = item_of_trigger(&trigger)?;

    if matches!(keyboard.key().as_str(), "Enter" | " ") {
        keyboard.prevent_default();
        toggle_item(&item);
        return Some(());
    }

    let accordion = accordion_of(&item)?;

    let items: Vec<_> = items(&accordion)
        .into_iter()
        .filter(|item| !is_disabled(item))
        .collect();
    let current = items.iter().position(|other| *other == item)?;
    let count = items.len();

    let next = match keyboard.key().as_str() {
        "ArrowDown" => (current + 1) % count,
        "ArrowUp" => (current + count - 1) % count,
        "Home" => 0,
        "End" => count - 1,
        _ => return None,
    };

    keyboard.prevent_default();
    if let Some(trigger) = self::trigger(&items[next]) {
        trigger.focus().ok();
    }

    Some(())
}

/// A search in the page found a match in a collapsed panel and the browser
/// reveals it: the item follows as expanded, collapsing the others in a single
/// mode. The reveal can't be canceled, so only `wg-after-expand` is dispatched.
pub fn handle_beforematch(event: &Event) -> Option<()> {
    let panel = event.target()?.maybe_into_element()?;
    if !panel.class_list().contains(ACCORDION_ITEM_PANEL) {
        return None;
    }

    let item = panel
        .parent_element()
        .filter(|item| item.class_list().contains(ACCORDION_ITEM))?;
    if is_expanded(&item) {
        return None;
    }

    if let Some(accordion) = accordion_of(&item)
        && Mode::of(&accordion).is_single()
    {
        collapse_others(&accordion, &item);
    }

    bump_generation(&item);
    set_expanded(&item, true);
    dispatch(&item, event::AFTER_EXPAND, false);

    Some(())
}

/// Links the trigger and the panel to each other by id. The server does it
/// when the item has an id; otherwise the ids are generated here.
fn link_trigger_and_panel(item: &Element) -> Option<()> {
    let trigger = trigger(item)?;
    let panel = panel(item)?;

    let trigger_id = ensure_id(&trigger, "accordion-item-trigger");
    let panel_id = ensure_id(&panel, "accordion-item-panel");
    trigger.set_attribute("aria-controls", &panel_id).ok();
    panel.set_attribute("aria-labelledby", &trigger_id).ok();

    Some(())
}

/// Synchronizes every accordion item on the page with its `expanded` class:
/// links the trigger and the panel, stops a stale animation, and aligns
/// `aria-expanded` and the panel's `hidden` with the state. Run it after
/// every render.
pub fn init_accordions() {
    for item in all_items() {
        link_trigger_and_panel(&item);

        let expanded = is_expanded(&item);
        set_expanded(&item, expanded);
        if let Some(panel) = panel(&item) {
            reset_body(&panel);
            set_panel_hidden(&panel, !expanded);
        }
    }
}

/// Installs the document-level listeners driving every accordion on the page.
pub fn listen_accordions() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_click(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });

    let options = AddEventListenerOptions::new();
    options.set_capture(true);
    document.add_steady_event_listener_with_options(
        "beforematch",
        |event| {
            handle_beforematch(&event);
        },
        &options,
    );
}
