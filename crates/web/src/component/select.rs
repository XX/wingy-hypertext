//! A Rust port of the `wa-select` behavior: opening/closing the listbox with
//! animation, single and multiple selection, keyboard navigation with
//! type-to-select, clearing, and closing on outside interaction. The state
//! lives entirely in the DOM (classes and attributes), matching the markup
//! produced by `wingy_hypertext::component::select`.

use hypertext::RenderableExt;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{
    Element, Event, EventInit, HtmlInputElement, KeyboardEvent, ScrollIntoViewOptions, ScrollLogicalPosition,
};
use wingy_hypertext::component::tag::Tag;

use crate::helper::popup;
use crate::util::animate::animate_with_class;
pub use crate::util::class::{is_disabled, is_multiple, is_open, is_selected};
use crate::util::convert::bool_to_str;
use crate::util::typeahead::{TypeaheadKey, typeahead_buffer, update_typeahead_buffer};

fn display_input(select: &Element) -> Option<HtmlInputElement> {
    select.query_selector(".display-input").ok()??.dyn_into().ok()
}

fn value_input(select: &Element) -> Option<HtmlInputElement> {
    select.query_selector(".value-input").ok()??.dyn_into().ok()
}

fn listbox(select: &Element) -> Option<Element> {
    select.query_selector(".listbox").ok()?
}

fn popup_host(select: &Element) -> Option<Element> {
    select.query_selector(".select-popup").ok()?
}

fn clear_button(select: &Element) -> Option<Element> {
    select.query_selector(".clear-button").ok()?
}

fn options(select: &Element) -> Vec<Element> {
    let mut options = Vec::new();
    if let Ok(list) = select.query_selector_all(".listbox .option") {
        for i in 0..list.length() {
            if let Some(option) = list.get(i).and_then(|node| node.maybe_into_element()) {
                options.push(option);
            }
        }
    }
    options
}

fn current_option(select: &Element) -> Option<Element> {
    select.query_selector(".option.current").ok()?
}

fn option_label(option: &Element) -> String {
    option.get_attribute("data-label").unwrap_or_else(|| {
        option
            .query_selector(".option-label")
            .ok()
            .flatten()
            .and_then(|label| label.text_content())
            .unwrap_or_default()
            .trim()
            .to_string()
    })
}

fn option_value(option: &Element) -> String {
    option
        .get_attribute("data-value")
        .unwrap_or_else(|| option_label(option))
}

fn set_option_selected(option: &Element, selected: bool) {
    option.class_list().toggle_with_force("selected", selected).ok();
    option.set_attribute("aria-selected", bool_to_str(selected)).ok();
}

/// Makes `current` the highlighted option, moving focus to it.
fn set_current_option(select: &Element, current: Option<&Element>) {
    for option in options(select) {
        option.class_list().remove_1("current").ok();
        option.set_attribute("tabindex", "-1").ok();
    }

    if let Some(option) = current {
        option.class_list().add_1("current").ok();
        option.set_attribute("tabindex", "0").ok();
        if let Some(html) = option.maybe_as_html() {
            html.focus().ok();
        }

        let options = ScrollIntoViewOptions::new();
        options.set_block(ScrollLogicalPosition::Nearest);
        option.scroll_into_view_with_scroll_into_view_options(&options);
    }
}

/// Creates a tag element by rendering the library's `Tag` component, so the
/// dynamically created tags always match the server-rendered markup. The
/// `data-value` attribute is set on top, like `wa-select` does with `wa-tag`.
fn create_tag(select: &Element, label: &str, value: Option<&str>) -> Option<Element> {
    let markup = Tag::builder()
        .pill(select.class_list().contains("pill"))
        .with_remove(value.is_some())
        .children(&label)
        .render();

    let holder = dom::existing::document().create_element("div").ok()?;
    holder.set_inner_html(markup.as_inner());
    let tag = holder.first_element_child()?;

    if let Some(value) = value {
        tag.set_attribute("data-value", value).ok()?;
    }

    Some(tag)
}

/// Renders the selected options of a `multiple` select as removable tags,
/// or "+n" after the `data-max-options-visible` limit (3 by default, 0 removes it).
fn rebuild_tags(select: &Element, selected: &[Element]) -> Option<()> {
    let tags = select.query_selector(".combobox .tags").ok()??;
    tags.set_inner_html("");

    let max = select
        .get_attribute("data-max-options-visible")
        .and_then(|value| value.parse::<usize>().ok())
        .filter(|&max| max > 0)
        .unwrap_or(if select.has_attribute("data-max-options-visible") {
            usize::MAX
        } else {
            3
        });
    let visible = max.min(selected.len());

    for option in &selected[..visible] {
        if let Some(tag) = create_tag(select, &option_label(option), Some(&option_value(option))) {
            tags.append_child(&tag).ok();
        }
    }

    if selected.len() > visible
        && let Some(more) = create_tag(select, &format!("+{}", selected.len() - visible), None)
    {
        tags.append_child(&more).ok();
    }

    Some(())
}

/// Updates the value input, the display label, the tags, and the clear button
/// to match the currently selected options. Must be called whenever the
/// selection changes.
pub fn selection_changed(select: &Element) -> Option<()> {
    let selected: Vec<Element> = options(select).into_iter().filter(is_selected).collect();
    let values: Vec<String> = selected.iter().map(option_value).collect();

    if is_multiple(select) {
        rebuild_tags(select, &selected);

        // The tags may change the combobox height, so keep the open listbox anchored
        if is_open(select)
            && let Some(host) = popup_host(select)
        {
            popup::reposition(&host);
        }
    }

    if let Some(input) = value_input(select) {
        input.set_value(&values.join(" "));
    }

    let display_label = if is_multiple(select) {
        // Web Awesome localizes this term; only the English variant is provided here.
        match selected.len() {
            0 => String::new(),
            1 => "1 option selected".to_string(),
            n => format!("{n} options selected"),
        }
    } else {
        selected.first().map(option_label).unwrap_or_default()
    };
    display_input(select)?.set_value(&display_label);

    // The clear button only shows when there is something to clear.
    if let Some(clear) = clear_button(select) {
        if (values.is_empty() && display_label.is_empty()) || is_disabled(select) {
            clear.set_attribute("hidden", "").ok();
        } else {
            clear.remove_attribute("hidden").ok();
        }
    }

    Some(())
}

fn dispatch_form_events(select: &Element) {
    for name in ["input", "change"] {
        let init = EventInit::new();
        init.set_bubbles(true);
        if let Ok(event) = Event::new_with_event_init_dict(name, &init) {
            select.dispatch_event(&event).ok();
        }
    }
}

/// Opens or closes the listbox, animating it unless reduced motion is requested
/// (the `show`/`hide` animations are disabled by CSS in that case).
pub async fn set_select_open(select: Element, open: bool) -> Option<()> {
    let listbox = listbox(&select)?;
    if open == is_open(&select) {
        return None;
    }

    if open {
        if is_disabled(&select) {
            return None;
        }

        select.class_list().add_1("open").ok();
        display_input(&select)?.set_attribute("aria-expanded", "true").ok();

        // Unhide the listbox before activating the popup so the positioning
        // logic measures the real dimensions.
        listbox.remove_attribute("hidden").ok();
        if let Some(host) = popup_host(&select) {
            popup::set_popup_active(&host, true);
        }

        // Highlight the selected option, or the first one
        let options = options(&select);
        let current = options.iter().find(|option| is_selected(option)).or(options.first());
        set_current_option(&select, current);

        animate_with_class(&listbox, "show").await.ok();
    } else {
        // Remove `.open` before the animation so the expand icon rotates back
        // while the listbox collapses.
        select.class_list().remove_1("open").ok();
        display_input(&select)?.set_attribute("aria-expanded", "false").ok();
        set_current_option(&select, None);

        animate_with_class(&listbox, "hide").await.ok();

        // A quick re-open may have started while the hide animation was running.
        if !is_open(&select) {
            listbox.set_attribute("hidden", "").ok();
            if let Some(host) = popup_host(&select) {
                popup::set_popup_active(&host, false);
            }
        }
    }

    Some(())
}

fn toggle_select(select: &Element, open: bool) {
    let select = select.clone();
    spawn_local(async move {
        set_select_open(select, open).await;
    });
}

/// Closes every open select that does not contain `target`.
fn close_open_selects_outside(target: Option<&Element>) {
    let Ok(selects) = dom::existing::document().query_selector_all(".select.open") else {
        return;
    };
    for i in 0..selects.length() {
        let Some(select) = selects.get(i).and_then(|node| node.maybe_into_element()) else {
            continue;
        };
        let contains_target = target.is_some_and(|target| select.contains(Some(target.as_ref())));
        if !contains_target {
            toggle_select(&select, false);
        }
    }
}

fn select_option(select: &Element, option: &Element) {
    if is_multiple(select) {
        set_option_selected(option, !is_selected(option));
    } else {
        for other in options(select) {
            set_option_selected(&other, other.is_same_node(Some(option)));
        }
    }

    selection_changed(select);
    dispatch_form_events(select);

    if !is_multiple(select) {
        toggle_select(select, false);
        if let Some(display) = display_input(select) {
            display.focus().ok();
        }
    }
}

//
// Event handlers
//

/// Toggles the listbox when the combobox is pressed and closes open selects
/// when pressing outside of them.
fn handle_select_mousedown(event: &Event) -> Option<()> {
    let target = event.target().and_then(|target| target.maybe_into_element());

    if let Some(target) = &target
        && target.closest(".clear-button").ok()?.is_none()
        && target.closest(".tag-remove").ok()?.is_none()
        && let Some(combobox) = target.closest(".select .combobox").ok()?
        && let Some(select) = combobox.closest(".select").ok()?
        && !is_disabled(&select)
    {
        // Prevent the press from stealing focus from the display input
        event.prevent_default();
        display_input(&select)?.focus().ok();
        toggle_select(&select, !is_open(&select));
    }

    close_open_selects_outside(target.as_ref());
    Some(())
}

fn handle_option_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let option = target.closest(".listbox .option").ok()??;
    let select = option.closest(".select").ok()??;

    if !option.class_list().contains("disabled") {
        select_option(&select, &option);

        // A pointer selection always returns focus to the display input; with
        // `multiple` the listbox stays open (keyboard selection keeps the
        // focus on the current option instead).
        if is_multiple(&select)
            && let Some(display) = display_input(&select)
        {
            display.focus().ok();
        }
    }
    Some(())
}

fn handle_clear_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let clear = target.closest(".select .clear-button").ok()??;
    let select = clear.closest(".select").ok()??;

    if is_disabled(&select) {
        return None;
    }

    for option in options(&select) {
        set_option_selected(&option, false);
    }
    selection_changed(&select);
    dispatch_form_events(&select);
    display_input(&select)?.focus().ok();

    Some(())
}

/// Deselects the option represented by a tag when the tag's remove button is
/// activated (the `handleTagRemove` part of `wa-select`).
fn handle_tag_remove_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let remove_button = target.closest(".tag-remove").ok()??;
    let tag = remove_button.closest(".select .tags .tag").ok()??;
    let select = tag.closest(".select").ok()??;

    if is_disabled(&select) {
        return None;
    }

    let value = tag.get_attribute("data-value")?;
    for option in options(&select) {
        if option_value(&option) == value {
            set_option_selected(&option, false);
        }
    }
    selection_changed(&select);
    dispatch_form_events(&select);

    Some(())
}

fn handle_label_click(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let label = target.closest(".select > .label").ok()??;
    let select = label.closest(".select").ok()??;

    display_input(&select)?.focus().ok();
    Some(())
}

fn handle_select_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    let target = event.target()?.maybe_into_element()?;
    let select = target.closest(".select").ok()??;

    // Ignore presses when the target is the clear button or a tag's remove button
    if target.closest(".clear-button").ok()?.is_some() || target.closest(".tag-remove").ok()?.is_some() {
        return None;
    }

    if is_disabled(&select) {
        return None;
    }

    let key = keyboard.key();
    let open = is_open(&select);
    let options = options(&select);
    let current = current_option(&select);

    // Close when pressing escape
    if key == "Escape" {
        if open {
            event.prevent_default();
            event.stop_propagation();
            toggle_select(&select, false);
            display_input(&select)?.focus().ok();
        }
        return Some(());
    }

    // Handle enter and space. When pressing space, we allow for type to select behaviors
    // so if there's anything in the buffer we don't close it.
    if key == "Enter" || (key == " " && typeahead_buffer(&select).is_empty()) {
        event.prevent_default();
        event.stop_propagation();

        if !open {
            toggle_select(&select, true);
            return Some(());
        }

        if let Some(current) = &current
            && !current.class_list().contains("disabled")
        {
            select_option(&select, current);
        }
        return Some(());
    }

    // Navigate options
    if matches!(key.as_str(), "ArrowUp" | "ArrowDown" | "Home" | "End") {
        event.prevent_default();

        if !open {
            // Opening highlights the selected option (or the first one) by itself
            toggle_select(&select, true);
            return Some(());
        }
        if options.is_empty() {
            return Some(());
        }

        let current_index = current
            .as_ref()
            .and_then(|current| options.iter().position(|option| option.is_same_node(Some(current))));
        let last = options.len() - 1;
        let new_index = match key.as_str() {
            "ArrowDown" => current_index.map_or(0, |index| if index >= last { 0 } else { index + 1 }),
            "ArrowUp" => current_index.map_or(last, |index| if index == 0 { last } else { index - 1 }),
            "Home" => 0,
            _ => last,
        };
        set_current_option(&select, options.get(new_index));
        return Some(());
    }

    // All other "printable" keys trigger type to select
    if let Some(key) = TypeaheadKey::new(&key) {
        // Don't block important key combos like CMD+R
        if keyboard.meta_key() || keyboard.ctrl_key() || keyboard.alt_key() {
            return Some(());
        }

        // Open, unless the key that triggered is backspace
        if !open {
            if let TypeaheadKey::Backspace = key {
                return Some(());
            }
            toggle_select(&select, true);
        }

        event.prevent_default();
        event.stop_propagation();

        let buffer = update_typeahead_buffer(&select, key);

        for option in &options {
            if option_label(option).to_lowercase().starts_with(&buffer) {
                set_current_option(&select, Some(option));
                break;
            }
        }
    }

    Some(())
}

/// Closes open selects when focus moves out of them.
fn handle_select_focusin(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    close_open_selects_outside(Some(&target));
    Some(())
}

/// Synchronizes every `.select` on the page with its markup state:
/// resets a stale open listbox and applies the initial selection to the
/// display and value inputs. Run it after every render.
pub fn init_selects() {
    let Ok(selects) = dom::existing::document().query_selector_all(".select") else {
        return;
    };
    for i in 0..selects.length() {
        let Some(select) = selects.get(i).and_then(|node| node.maybe_into_element()) else {
            continue;
        };

        select.class_list().remove_1("open").ok();
        if let Some(listbox) = listbox(&select) {
            listbox.class_list().remove_2("show", "hide").ok();
            listbox.set_attribute("hidden", "").ok();
        }
        if let Some(host) = popup_host(&select) {
            popup::set_popup_active(&host, false);
        }
        selection_changed(&select);
    }
}

/// Installs the document-level listeners that drive all selects on the page.
pub fn listen_selects() {
    let document = dom::existing::document();

    document.add_steady_event_listener("mousedown", |event| {
        handle_select_mousedown(&event);
    });
    document.add_steady_event_listener("click", |event| {
        handle_option_click(&event)
            .or_else(|| handle_clear_click(&event))
            .or_else(|| handle_tag_remove_click(&event))
            .or_else(|| handle_label_click(&event));
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_select_keydown(&event);
    });
    document.add_steady_event_listener("focusin", |event| {
        handle_select_focusin(&event);
    });
}
