//! A `Rating` behavior: previewing the value under the pointer, setting it by a
//! press and the keyboard, and restoring it on a form reset. The symbols are
//! clipped by the stylesheet from the `--value` custom property of the rating,
//! so this only keeps `--value` in step with the displayed value, and
//! `aria-valuenow` and the hidden input with the committed one.

use std::cell::Cell;

use const_format::concatcp;
use js_sys::Object;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{Element, Event, HtmlInputElement, KeyboardEvent, PointerEvent};
use wingy_hypertext::class::{
    DISABLED, RATING, RATING_SYMBOL, RATING_SYMBOL_HOVER, RATING_SYMBOLS, RATING_VALUE, READONLY,
};

use crate::util::convert::{format_float, parse_number_attr};
use crate::util::direction::is_rtl;
use crate::util::event;

const RATING_SELECTOR: &str = concatcp!('.', RATING);

/// The value under the pointer while the user hovers over (or touches and
/// drags across) a rating, before committing to it.
const HOVER_VALUE: &str = "data-hover-value";

/// Set once a press ends the hovering: the pointer resting over the
/// rating doesn't start it again until it leaves the rating or presses again,
/// so the user sees the value they have just committed (`isHovering` in
/// `wa-rating`).
const HOVER_PAUSED: &str = "data-hover-paused";

thread_local! {
    /// Whether the pointer was pressed outside of any rating, so releasing it
    /// over one — at the end of a text selection, say — doesn't set its value.
    static PRESSED_OUTSIDE: Cell<bool> = const { Cell::new(false) };
}

struct Config {
    max: f64,
    precision: f64,
    interactive: bool,
}

fn rating_config(rating: &Element) -> Config {
    let classes = rating.class_list();
    Config {
        max: parse_number_attr(rating, "aria-valuemax").unwrap_or(5.0),
        precision: parse_number_attr(rating, "data-precision")
            .filter(|precision| *precision > 0.0)
            .unwrap_or(1.0),
        interactive: !classes.contains(DISABLED) && !classes.contains(READONLY),
    }
}

/// Rounds a value up to the precision, as `wa-rating` does, keeping the digits
/// of the precision so that 0.1 steps don't produce values like 0.30000000000000004.
fn round_to_precision(value: f64, config: &Config) -> f64 {
    let multiplier = 1.0 / config.precision;
    let rounded = (value * multiplier).ceil() / multiplier;
    tidy(rounded, config).clamp(0.0, config.max)
}

fn tidy(value: f64, config: &Config) -> f64 {
    let digits = format!("{}", config.precision)
        .split_once('.')
        .map(|(_, fraction)| fraction.len())
        .unwrap_or(0);
    let factor = 10f64.powi(digits as i32);
    (value * factor).round() / factor
}

fn value_of(rating: &Element) -> f64 {
    parse_number_attr(rating, "aria-valuenow").unwrap_or(0.0)
}

fn hidden_input(rating: &Element) -> Option<HtmlInputElement> {
    rating
        .query_selector(concatcp!('.', RATING_VALUE))
        .ok()??
        .dyn_into::<HtmlInputElement>()
        .ok()
}

/// Draws `value`: the clipped symbols and, while hovering, the enlarged symbol
/// under the pointer.
fn display(rating: &Element, value: f64, hovering: bool) -> Option<()> {
    rating
        .maybe_as_html()?
        .style()
        .set_property("--value", &format_float(value))
        .ok();

    let hovered = if hovering { value.ceil() as usize } else { 0 };
    let symbols = dom::existing::select_all_elements_from(rating, concatcp!('.', RATING_SYMBOL));
    for (index, symbol) in symbols.enumerate() {
        symbol
            .class_list()
            .toggle_with_force(RATING_SYMBOL_HOVER, index + 1 == hovered)
            .ok();
    }

    Some(())
}

/// Commits a value: exposes it, puts it in the form and reports it with a
/// `change` event when it differs from the previous one.
fn set_value(rating: &Element, value: f64) {
    let previous = value_of(rating);
    write_value(rating, value);

    if previous != value {
        event::dispatch(rating, event::CHANGE, true).ok();
    }
}

fn write_value(rating: &Element, value: f64) {
    let text = format_float(value);
    rating.set_attribute("aria-valuenow", &text).ok();
    if let Some(input) = hidden_input(rating) {
        // An empty value leaves a required rating invalid
        input.set_value(if value > 0.0 { &text } else { "" });
    }
    display(rating, value, false);
}

/// The value a pointer at `x` points at, following the text direction.
fn value_from_x(rating: &Element, x: f64, config: &Config) -> Option<f64> {
    let symbols = rating.query_selector(concatcp!('.', RATING_SYMBOLS)).ok()??;
    let rect = symbols.get_bounding_client_rect();
    if rect.width() == 0.0 {
        return None;
    }

    let fraction = if is_rtl(rating) {
        (rect.right() - x) / rect.width()
    } else {
        (x - rect.left()) / rect.width()
    };
    Some(round_to_precision(fraction * config.max, config))
}

/// Reports the hovering with a `wg-hover` event carrying the `phase` and the
/// `value` the rating would take, as `wa-hover` of `wa-rating` does.
fn dispatch_hover(rating: &Element, phase: &str, value: f64) {
    let detail = Object::new();
    detail.set("phase", phase);
    detail.set("value", value);
    event::dispatch_custom(rating, event::HOVER, true, false, detail.as_ref()).ok();
}

/// Previews `value` under the pointer, starting the hovering if needed.
fn hover(rating: &Element, value: f64) {
    let previous = parse_number_attr(rating, HOVER_VALUE);
    rating.set_attribute(HOVER_VALUE, &format_float(value)).ok();
    display(rating, value, true);

    if previous.is_none() {
        dispatch_hover(rating, "start", value);
    }
    if previous != Some(value) {
        dispatch_hover(rating, "move", value);
    }
}

/// Ends the hovering, going back to the committed value.
fn end_hover(rating: &Element) {
    rating.set_attribute(HOVER_PAUSED, "").ok();
    let Some(value) = parse_number_attr(rating, HOVER_VALUE) else {
        return;
    };
    rating.remove_attribute(HOVER_VALUE).ok();
    display(rating, value_of(rating), false);
    dispatch_hover(rating, "end", value);
}

/// The rating `event` happened in, if it can be changed.
fn interactive_rating(event: &Event) -> Option<(Element, Config)> {
    let target = event.target()?.maybe_into_element()?;
    let rating = target.closest(RATING_SELECTOR).ok()??;
    let config = rating_config(&rating);
    config.interactive.then_some((rating, config))
}

//
// Pointer
//

fn handle_pointer_down(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    if pointer.button() != 0 {
        return None;
    }
    let Some((rating, config)) = interactive_rating(event) else {
        PRESSED_OUTSIDE.set(true);
        return None;
    };
    PRESSED_OUTSIDE.set(false);

    rating.remove_attribute(HOVER_PAUSED).ok();
    hover(&rating, value_from_x(&rating, pointer.client_x(), &config)?);
    // Pointer moves and the release keep coming to the rating outside of it,
    // so dragging a finger across it works
    rating.set_pointer_capture(pointer.pointer_id()).ok();

    Some(())
}

fn handle_pointer_move(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    let (rating, config) = interactive_rating(event)?;
    if rating.has_attribute(HOVER_PAUSED) {
        return None;
    }
    hover(&rating, value_from_x(&rating, pointer.client_x(), &config)?);
    Some(())
}

/// Commits the value under the pointer. Pressing the current value clears the
/// rating.
///
/// It's the release, not a `click`, that commits: the press may never come to
/// the page — Firefox spends the press that closes its form validation popup on
/// closing it — and a `click` doesn't follow a release without one. The
/// release, though, does come, so a single press sets a rating that the popup
/// points at.
fn handle_pointer_up(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    if pointer.button() != 0 || PRESSED_OUTSIDE.replace(false) {
        return None;
    }
    let (rating, config) = interactive_rating(event)?;
    release_pointer(&rating, pointer);

    let value = value_from_x(&rating, pointer.client_x(), &config)?;
    let value = if value == value_of(&rating) { 0.0 } else { value };
    end_hover(&rating);
    set_value(&rating, value);

    Some(())
}

fn handle_pointer_cancel(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    PRESSED_OUTSIDE.set(false);
    let (rating, _) = interactive_rating(event)?;
    release_pointer(&rating, pointer);
    end_hover(&rating);
    Some(())
}

fn release_pointer(rating: &Element, pointer: &PointerEvent) {
    if rating.has_pointer_capture(pointer.pointer_id()) {
        rating.release_pointer_capture(pointer.pointer_id()).ok();
    }
}

fn handle_pointer_out(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    let target = event.target()?.maybe_into_element()?;
    let rating = target.closest(RATING_SELECTOR).ok()??;

    // Moving between the parts of the rating isn't leaving it
    let entered = pointer
        .related_target()
        .and_then(|related| related.maybe_into_element());
    if entered.is_some_and(|entered| rating.contains(Some(&entered))) {
        return None;
    }
    end_hover(&rating);
    rating.remove_attribute(HOVER_PAUSED).ok();
    Some(())
}

//
// Keyboard
//

fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    let (rating, config) = interactive_rating(event)?;

    let current = value_of(&rating);
    let rtl = is_rtl(&rating);
    let step = if keyboard.shift_key() { 1.0 } else { config.precision };

    let value = match keyboard.key().as_str() {
        "ArrowUp" => current + step,
        "ArrowDown" => current - step,
        "ArrowRight" if rtl => current - step,
        "ArrowRight" => current + step,
        "ArrowLeft" if rtl => current + step,
        "ArrowLeft" => current - step,
        "Home" => 0.0,
        "End" => config.max,
        _ => return None,
    };

    event.prevent_default();
    set_value(&rating, tidy(value, &config).clamp(0.0, config.max));

    Some(())
}

//
// Form reset
//

/// Brings the ratings of a reset form back to their initial values. The reset
/// itself restores the hidden inputs to the values they were rendered with, so
/// the ratings read them back once it is done.
fn handle_reset(event: &Event) -> Option<()> {
    let form = event.target()?.maybe_into_element()?;

    dom::set_timeout(
        move || {
            for rating in dom::existing::select_all_elements_from(&form, RATING_SELECTOR) {
                let value = hidden_input(&rating)
                    .and_then(|input| input.value().trim().parse().ok())
                    .unwrap_or(0.0);
                write_value(&rating, value);
            }
        },
        0,
    )
    .ok();

    Some(())
}

//
// Initialization
//

/// Installs the document-level listeners that drive all ratings on the page.
pub fn listen_ratings() {
    let document = dom::existing::document();

    document.add_steady_event_listener("pointerdown", |event| {
        handle_pointer_down(&event);
    });
    document.add_steady_event_listener("pointermove", |event| {
        handle_pointer_move(&event);
    });
    document.add_steady_event_listener("pointerup", |event| {
        handle_pointer_up(&event);
    });
    document.add_steady_event_listener("pointercancel", |event| {
        handle_pointer_cancel(&event);
    });
    document.add_steady_event_listener("pointerout", |event| {
        handle_pointer_out(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });
    document.add_steady_event_listener("reset", |event| {
        handle_reset(&event);
    });
}
