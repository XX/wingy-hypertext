//! A Rust port of the `wa-dropdown` behavior: opening and closing the menu with
//! animations, selection (including checkable items), submenus opened by
//! pointer and keyboard, keyboard navigation with type-to-select, and closing
//! on outside interaction. The menu is anchored with the popup helper
//! (`helper::popup`), and the state lives entirely in the DOM (classes and
//! attributes), matching the markup produced by
//! `wingy_hypertext::component::dropdown`.

use js_sys::Object;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{
    AddEventListenerOptions, Element, Event, KeyboardEvent, NodeList, ScrollIntoViewOptions, ScrollLogicalPosition,
};

use crate::helper::popup;
use crate::util::animate::animate_with_class;
use crate::util::convert::bool_to_str;
use crate::util::event;

/// How long the type-to-select buffer lives without new keystrokes.
const TYPEAHEAD_TIMEOUT_MILLIS: f64 = 1000.0;

/// Submenus open next to their item, pulled up slightly so the first submenu
/// item lines up with it.
const SUBMENU_PLACEMENT: &str = "right-start";
const SUBMENU_SKIDDING: f64 = -5.0;
const SUBMENU_PADDING: f64 = 8.0;

/// The menu of the dropdown itself (not of a nested one).
fn menu(dropdown: &Element) -> Option<Element> {
    dropdown
        .query_selector(":scope > .popup > .popup-body > .dropdown-menu")
        .ok()?
}

fn popup_host(dropdown: &Element) -> Option<Element> {
    dropdown.query_selector(":scope > .popup").ok()?
}

/// The trigger is the popup's anchor: the child of the popup that isn't part of
/// the popup itself.
fn trigger(dropdown: &Element) -> Option<Element> {
    dropdown
        .query_selector(":scope > .popup > :not(.popup-body):not(.popup-hover-bridge)")
        .ok()?
}

fn elements(list: NodeList) -> Vec<Element> {
    (0..list.length())
        .filter_map(|index| list.get(index).and_then(|node| node.maybe_into_element()))
        .collect()
}

fn query_all(root: &Element, selector: &str) -> Vec<Element> {
    root.query_selector_all(selector).map(elements).unwrap_or_default()
}

/// The items of a single menu level: submenu items live inside their own item,
/// so only direct children belong to `container`.
fn items(container: &Element) -> Vec<Element> {
    query_all(container, ":scope > .dropdown-item")
}

fn enabled_items(container: &Element) -> Vec<Element> {
    items(container).into_iter().filter(|item| !is_disabled(item)).collect()
}

fn is_open(dropdown: &Element) -> bool {
    dropdown.class_list().contains("open")
}

fn is_disabled(item: &Element) -> bool {
    item.class_list().contains("disabled")
}

fn is_checkbox(item: &Element) -> bool {
    item.get_attribute("role").as_deref() == Some("menuitemcheckbox")
}

fn submenu_of(item: &Element) -> Option<Element> {
    item.query_selector(":scope > .dropdown-submenu").ok()?
}

fn is_submenu_open(item: &Element) -> bool {
    item.class_list().contains("submenu-open")
}

fn item_label(item: &Element) -> String {
    item.get_attribute("data-label").unwrap_or_else(|| {
        // Only the item's own label: a submenu nests items carrying labels too.
        item.query_selector(":scope > .dropdown-item-label")
            .ok()
            .flatten()
            .and_then(|label| label.text_content())
            .unwrap_or_default()
            .trim()
            .to_string()
    })
}

/// Makes `item` the active (highlighted and focused) one. Only a single item of
/// a dropdown is active at a time, across all of its menu levels.
fn set_active_item(dropdown: &Element, item: Option<&Element>) {
    for other in query_all(dropdown, ".dropdown-item.active") {
        other.class_list().remove_1("active").ok();
        other.set_attribute("tabindex", "-1").ok();
    }

    if let Some(item) = item {
        item.class_list().add_1("active").ok();
        item.set_attribute("tabindex", "0").ok();
        if let Some(html) = item.maybe_as_html() {
            html.focus().ok();
        }

        let options = ScrollIntoViewOptions::new();
        options.set_block(ScrollLogicalPosition::Nearest);
        item.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

fn active_item(dropdown: &Element) -> Option<Element> {
    dropdown.query_selector(".dropdown-item.active").ok()?
}

/// The menu level the keyboard currently operates on: the one holding the
/// active item, or the top-level menu.
fn current_menu(dropdown: &Element) -> Option<Element> {
    active_item(dropdown)
        .and_then(|item| item.parent_element())
        .filter(|parent| {
            let classes = parent.class_list();
            classes.contains("dropdown-menu") || classes.contains("dropdown-submenu")
        })
        .or_else(|| menu(dropdown))
}

//
// Opening and closing
//

/// Shows the menu, animating it in unless the `wg-show` event is canceled.
async fn show(dropdown: Element) -> Option<()> {
    let menu = menu(&dropdown)?;
    if is_open(&dropdown) {
        return None;
    }

    // A canceled `wg-show` keeps the dropdown closed.
    if !event::dispatch_custom(&dropdown, event::SHOW, true, true, &JsValue::NULL).unwrap_or(true) {
        return None;
    }

    close_dropdowns_outside(Some(&dropdown));

    dropdown.class_list().add_1("open").ok();
    if let Some(trigger) = trigger(&dropdown) {
        trigger.set_attribute("aria-expanded", "true").ok();
    }

    // Unhide the menu before activating the popup so the positioning logic
    // measures the real dimensions.
    menu.remove_attribute("hidden").ok();
    if let Some(host) = popup_host(&dropdown) {
        popup::set_popup_active(&host, true);
    }

    set_active_item(&dropdown, enabled_items(&menu).first());

    animate_with_class(&menu, "show").await.ok();

    event::dispatch_custom(&dropdown, event::AFTER_SHOW, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Requests to close the menu. Dispatches a cancelable `wg-hide` carrying the
/// `source` element that triggered the request; when canceled the menu stays open.
async fn hide(dropdown: Element, source: Element) -> Option<()> {
    let menu = menu(&dropdown)?;
    if !is_open(&dropdown) {
        return None;
    }

    let detail = Object::new();
    detail.set("source", source);

    if !event::dispatch_custom(&dropdown, event::HIDE, true, true, detail.as_ref()).unwrap_or(true) {
        return None;
    }

    close_all_submenus(&dropdown);
    dropdown.class_list().remove_1("open").ok();
    set_active_item(&dropdown, None);
    if let Some(trigger) = trigger(&dropdown) {
        trigger.set_attribute("aria-expanded", "false").ok();
    }

    animate_with_class(&menu, "hide").await.ok();

    // A quick re-open may have started while the hide animation was running.
    if !is_open(&dropdown) {
        menu.set_attribute("hidden", "").ok();
        if let Some(host) = popup_host(&dropdown) {
            popup::set_popup_active(&host, false);
        }
    }

    event::dispatch_custom(&dropdown, event::AFTER_HIDE, true, false, &JsValue::NULL).ok();

    Some(())
}

/// Opens or closes the dropdown; `source` is reported to `wg-hide` listeners.
pub fn set_dropdown_open(dropdown: &Element, open: bool, source: &Element) {
    let dropdown = dropdown.clone();
    let source = source.clone();
    spawn_local(async move {
        if open {
            show(dropdown).await;
        } else {
            hide(dropdown, source).await;
        }
    });
}

fn open_dropdowns() -> Vec<Element> {
    dom::existing::document()
        .query_selector_all(".dropdown.open")
        .map(elements)
        .unwrap_or_default()
}

/// Closes every open dropdown that does not contain `target`.
fn close_dropdowns_outside(target: Option<&Element>) {
    for dropdown in open_dropdowns() {
        let contains_target = target.is_some_and(|target| dropdown.contains(Some(target.as_ref())));
        if !contains_target {
            set_dropdown_open(&dropdown, false, &dropdown.clone());
        }
    }
}

//
// Submenus
//

/// Positions a submenu next to its item, mirroring `wa-dropdown`'s use of
/// floating-ui: to the end side, flipped and shifted to stay in the viewport.
fn position_submenu(item: &Element, submenu: &Element) -> Option<()> {
    let mut config = popup::PopupConfig::new(SUBMENU_PLACEMENT);
    config.skidding = SUBMENU_SKIDDING;
    config.flip = true;
    config.shift = true;
    config.shift_padding = SUBMENU_PADDING;
    config.auto_size = Some("vertical".to_string());
    config.auto_size_padding = SUBMENU_PADDING;

    let floating = submenu.clone().maybe_into_html()?;
    let (side, align) = popup::place(item, &floating, &config, submenu)?;
    submenu
        .set_attribute("data-placement", &popup::placement_str(side, align))
        .ok();

    Some(())
}

/// Keeps the open submenus anchored to their items while the page scrolls or
/// resizes — the popup helper does the same for the menus themselves.
fn reposition_open_submenus() {
    let Ok(items) = dom::existing::document().query_selector_all(".dropdown-item.submenu-open") else {
        return;
    };
    for item in elements(items) {
        if let Some(submenu) = submenu_of(&item) {
            position_submenu(&item, &submenu);
        }
    }
}

async fn open_submenu(item: Element, focus_first: bool) -> Option<()> {
    let submenu = submenu_of(&item)?;
    if is_disabled(&item) {
        return None;
    }

    // Only one submenu of a level stays open at a time.
    if let Some(dropdown) = item.closest(".dropdown").ok().flatten() {
        close_submenus_outside(&dropdown, &item);
    }

    if !is_submenu_open(&item) {
        item.class_list().add_1("submenu-open").ok();
        item.set_attribute("aria-expanded", "true").ok();
        submenu.remove_attribute("hidden").ok();
        position_submenu(&item, &submenu);

        animate_with_class(&submenu, "show").await.ok();
    }

    if focus_first && let Some(dropdown) = item.closest(".dropdown").ok().flatten() {
        set_active_item(&dropdown, enabled_items(&submenu).first());
    }

    Some(())
}

async fn close_submenu(item: Element) -> Option<()> {
    let submenu = submenu_of(&item)?;
    if !is_submenu_open(&item) {
        return None;
    }

    // Nested submenus close from the inside out.
    for nested in query_all(&submenu, ":scope > .dropdown-item.submenu-open") {
        close_submenu_now(&nested);
    }

    item.class_list().remove_1("submenu-open").ok();
    item.set_attribute("aria-expanded", "false").ok();

    animate_with_class(&submenu, "hide").await.ok();

    if !is_submenu_open(&item) {
        submenu.set_attribute("hidden", "").ok();
    }

    Some(())
}

/// Closes a submenu without animating it, for resetting stale state.
fn close_submenu_now(item: &Element) {
    let Some(submenu) = submenu_of(item) else {
        return;
    };

    for nested in query_all(&submenu, ":scope > .dropdown-item.submenu-open") {
        close_submenu_now(&nested);
    }

    item.class_list().remove_1("submenu-open").ok();
    item.set_attribute("aria-expanded", "false").ok();
    submenu.class_list().remove_2("show", "hide").ok();
    submenu.set_attribute("hidden", "").ok();
}

fn close_all_submenus(dropdown: &Element) {
    for item in query_all(dropdown, ".dropdown-item.submenu-open") {
        close_submenu_now(&item);
    }
}

/// Closes the open submenus that don't lead to `item`, so moving along a menu
/// level collapses the submenu of the item left behind.
fn close_submenus_outside(dropdown: &Element, item: &Element) {
    for open in query_all(dropdown, ".dropdown-item.submenu-open") {
        if !open.contains(Some(item.as_ref())) {
            let open = open.clone();
            spawn_local(async move {
                close_submenu(open).await;
            });
        }
    }
}

/// The deepest open submenu item, i.e. the top of the submenu stack.
fn deepest_open_submenu(dropdown: &Element) -> Option<Element> {
    query_all(dropdown, ".dropdown-item.submenu-open")
        .into_iter()
        .next_back()
}

//
// Selection
//

/// Toggles a checkable item, emits `wg-select`, and closes the dropdown unless
/// the event was canceled.
fn make_selection(dropdown: &Element, item: &Element) -> Option<()> {
    if is_disabled(item) {
        return None;
    }

    if is_checkbox(item) {
        let checked = !item.class_list().contains("checked");
        item.class_list().toggle_with_force("checked", checked).ok();
        item.set_attribute("aria-checked", bool_to_str(checked)).ok();
    }

    let detail = Object::new();
    detail.set("item", item.clone());
    if let Some(value) = item.get_attribute("data-value") {
        detail.set("value", value);
    }

    let selected = event::dispatch_custom(dropdown, event::SELECT, true, true, detail.as_ref()).unwrap_or(true);
    if selected {
        set_dropdown_open(dropdown, false, item);
        if let Some(trigger) = trigger(dropdown).and_then(|trigger| trigger.maybe_into_html()) {
            trigger.focus().ok();
        }
    }

    Some(())
}

//
// Event handlers
//

/// Activates the trigger, selects items and opens submenus.
fn handle_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let dropdown = target.closest(".dropdown").ok()??;

    if let Some(item) = target.closest(".dropdown-item").ok().flatten() {
        if is_disabled(&item) {
            return Some(());
        }

        if submenu_of(&item).is_some() {
            spawn_local(async move {
                open_submenu(item, false).await;
            });
        } else {
            make_selection(&dropdown, &item);
        }
        return Some(());
    }

    // Clicks inside the menu that don't hit an item change nothing
    if target.closest(".dropdown-menu, .dropdown-submenu").ok()?.is_some() {
        return Some(());
    }

    set_dropdown_open(&dropdown, !is_open(&dropdown), &target);
    Some(())
}

/// Closes the dropdowns the pointer went down outside of.
fn handle_mousedown(event: &Event) -> Option<()> {
    let target = event.target().and_then(|target| target.maybe_into_element());
    close_dropdowns_outside(target.as_ref());
    Some(())
}

/// Opens the submenu of the hovered item and collapses the submenus left behind.
fn handle_mouseover(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let item = target.closest(".dropdown-item").ok()??;
    let dropdown = item.closest(".dropdown").ok()??;

    if !is_open(&dropdown) {
        return None;
    }

    if submenu_of(&item).is_some() && !is_disabled(&item) {
        spawn_local(async move {
            open_submenu(item, false).await;
        });
    } else {
        close_submenus_outside(&dropdown, &item);
    }

    Some(())
}

fn typeahead_buffer(dropdown: &Element) -> String {
    let last = dropdown
        .get_attribute("data-typeahead-time")
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.0);

    if js_sys::Date::now() - last > TYPEAHEAD_TIMEOUT_MILLIS {
        String::new()
    } else {
        dropdown.get_attribute("data-typeahead").unwrap_or_default()
    }
}

fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;

    // The top-most open dropdown owns the keyboard.
    let dropdown = open_dropdowns().into_iter().next_back()?;
    let key = keyboard.key();

    // Escape collapses the deepest submenu, or closes the dropdown
    if key == "Escape" {
        event.prevent_default();
        event.stop_propagation();

        if let Some(item) = deepest_open_submenu(&dropdown) {
            set_active_item(&dropdown, Some(&item));
            spawn_local(async move {
                close_submenu(item).await;
            });
        } else {
            set_dropdown_open(&dropdown, false, &dropdown.clone());
            if let Some(trigger) = trigger(&dropdown).and_then(|trigger| trigger.maybe_into_html()) {
                trigger.focus().ok();
            }
        }
        return Some(());
    }

    if key == "Tab" {
        set_dropdown_open(&dropdown, false, &dropdown.clone());
        return Some(());
    }

    let menu = current_menu(&dropdown)?;
    let items = enabled_items(&menu);
    let active = active_item(&dropdown);

    // Navigate the current menu level
    if matches!(key.as_str(), "ArrowUp" | "ArrowDown" | "Home" | "End") {
        event.prevent_default();
        event.stop_propagation();

        if items.is_empty() {
            return Some(());
        }

        let current_index = active
            .as_ref()
            .and_then(|active| items.iter().position(|item| item.is_same_node(Some(active))));
        let last = items.len() - 1;
        let new_index = match key.as_str() {
            "ArrowDown" => current_index.map_or(0, |index| if index >= last { 0 } else { index + 1 }),
            "ArrowUp" => current_index.map_or(last, |index| if index == 0 { last } else { index - 1 }),
            "Home" => 0,
            _ => last,
        };
        let item = items.get(new_index);
        set_active_item(&dropdown, item);
        if let Some(item) = item {
            close_submenus_outside(&dropdown, item);
        }
        return Some(());
    }

    // Enter the submenu of the active item
    if key == "ArrowRight" {
        if let Some(active) = active.filter(|active| submenu_of(active).is_some()) {
            event.prevent_default();
            event.stop_propagation();
            spawn_local(async move {
                open_submenu(active, true).await;
            });
        }
        return Some(());
    }

    // Leave the current submenu, returning to the item that owns it
    if key == "ArrowLeft" {
        if menu.class_list().contains("dropdown-submenu")
            && let Some(item) = menu.parent_element()
        {
            event.prevent_default();
            event.stop_propagation();
            set_active_item(&dropdown, Some(&item));
            spawn_local(async move {
                close_submenu(item).await;
            });
        }
        return Some(());
    }

    // Activate the active item; space also feeds the type-to-select buffer
    if key == "Enter" || (key == " " && typeahead_buffer(&dropdown).is_empty()) {
        event.prevent_default();
        event.stop_propagation();

        if let Some(active) = active {
            if submenu_of(&active).is_some() {
                spawn_local(async move {
                    open_submenu(active, true).await;
                });
            } else {
                make_selection(&dropdown, &active);
            }
        }
        return Some(());
    }

    // All other "printable" keys trigger type to select
    if key.chars().count() == 1 || key == "Backspace" {
        // Don't block important key combos like CMD+R
        if keyboard.meta_key() || keyboard.ctrl_key() || keyboard.alt_key() {
            return Some(());
        }

        event.prevent_default();
        event.stop_propagation();

        let mut buffer = typeahead_buffer(&dropdown);
        if key == "Backspace" {
            buffer.pop();
        } else {
            buffer.push_str(&key.to_lowercase());
        }
        dropdown.set_attribute("data-typeahead", &buffer).ok();
        dropdown
            .set_attribute("data-typeahead-time", &js_sys::Date::now().to_string())
            .ok();

        for item in &items {
            if item_label(item).to_lowercase().starts_with(&buffer) {
                set_active_item(&dropdown, Some(item));
                close_submenus_outside(&dropdown, item);
                break;
            }
        }
    }

    Some(())
}

//
// Initialization
//

/// Aligns the items of a menu level with each other: when one of them is
/// checkable or opens a submenu, the others reserve the same space. This is
/// what `wa-dropdown` does with its `checkbox-adjacent`/`submenu-adjacent`
/// properties when the items are slotted in.
fn sync_adjacent_items(container: &Element) {
    let items = items(container);
    let has_checkbox = items.iter().any(is_checkbox);
    let has_submenu = items.iter().any(|item| submenu_of(item).is_some());

    for item in &items {
        item.class_list()
            .toggle_with_force("checkbox-adjacent", has_checkbox)
            .ok();
        item.class_list()
            .toggle_with_force("submenu-adjacent", has_submenu)
            .ok();

        if let Some(submenu) = submenu_of(item) {
            sync_adjacent_items(&submenu);
        }
    }
}

/// Synchronizes every `.dropdown` on the page with its markup state: resets a
/// stale open menu, prepares the trigger, aligns the items, and opens the
/// dropdowns rendered with `data-open`. Run it after every render.
pub fn init_dropdowns() {
    let Ok(dropdowns) = dom::existing::document().query_selector_all(".dropdown") else {
        return;
    };

    for dropdown in elements(dropdowns) {
        dropdown.class_list().remove_1("open").ok();
        close_all_submenus(&dropdown);

        if let Some(menu) = menu(&dropdown) {
            menu.class_list().remove_2("show", "hide").ok();
            menu.set_attribute("hidden", "").ok();
            sync_adjacent_items(&menu);
        }
        if let Some(host) = popup_host(&dropdown) {
            popup::set_popup_active(&host, false);
        }
        if let Some(trigger) = trigger(&dropdown) {
            trigger.set_attribute("aria-haspopup", "menu").ok();
            trigger.set_attribute("aria-expanded", "false").ok();
        }

        if dropdown.has_attribute("data-open") {
            set_dropdown_open(&dropdown, true, &dropdown.clone());
        }
    }
}

/// Installs the document-level listeners that drive all dropdowns on the page.
pub fn listen_dropdowns() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_click(&event);
    });
    document.add_steady_event_listener("mousedown", |event| {
        handle_mousedown(&event);
    });
    document.add_steady_event_listener("mouseover", |event| {
        handle_mouseover(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });

    // Scroll events don't bubble, but they do capture, so a capturing listener
    // on the window sees scrolling of any nested container too.
    let window = dom::existing::window();
    let options = AddEventListenerOptions::new();
    options.set_capture(true);
    options.set_passive(true);
    window.add_steady_event_listener_with_options(
        "scroll",
        |_| {
            reposition_open_submenus();
        },
        &options,
    );
    window.add_steady_event_listener("resize", |_| {
        reposition_open_submenus();
    });
}
