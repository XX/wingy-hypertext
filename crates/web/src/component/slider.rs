//! A `Slider` behavior: dragging a thumb along the track, the keyboard, the
//! range mode with two thumbs and the tooltip showing the current value. The
//! markup is rendered with its final positions, so this only keeps them in step
//! with the value: the positions live in the `--position`, `--start` and `--end`
//! custom properties, the value in `aria-valuenow` and in the hidden input the
//! form submits.

use std::cell::RefCell;
use std::rc::Rc;

use const_format::concatcp;
use js_sys::Function;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{Element, Event, HtmlInputElement, KeyboardEvent, PointerEvent};
use wingy_hypertext::class::{SLIDER_CONTROL, SLIDER_THUMB, SLIDER_THUMB_MAX, SLIDER_THUMB_MIN};

use crate::component::tooltip;
use crate::helper::popup;
use crate::util::direction::is_rtl;
use crate::util::event;

/// Which thumb of a range slider a value belongs to.
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Thumb {
    Single,
    Min,
    Max,
}

impl Thumb {
    pub const fn selector(self) -> &'static str {
        match self {
            Self::Single => concatcp!('.', SLIDER_THUMB),
            Self::Min => concatcp!('.', SLIDER_THUMB_MIN),
            Self::Max => concatcp!('.', SLIDER_THUMB_MAX),
        }
    }
}

struct Config {
    min: f64,
    max: f64,
    step: f64,
    indicator_offset: Option<f64>,
    vertical: bool,
    range: bool,
    interactive: bool,
}

fn number_attr(element: &Element, name: &str) -> Option<f64> {
    element.get_attribute(name)?.trim().parse().ok()
}

fn slider_config(slider: &Element) -> Config {
    let classes = slider.class_list();
    Config {
        min: number_attr(slider, "data-min").unwrap_or(0.0),
        max: number_attr(slider, "data-max").unwrap_or(100.0),
        step: number_attr(slider, "data-step").unwrap_or(1.0).max(f64::EPSILON),
        indicator_offset: number_attr(slider, "data-indicator-offset"),
        vertical: classes.contains("vertical"),
        range: slider.query_selector(Thumb::Min.selector()).ok().flatten().is_some(),
        interactive: !classes.contains("disabled") && !slider.has_attribute("data-readonly"),
    }
}

/// Formats a value the way the markup does, so the DOM doesn't drift from what
/// the server rendered: whole numbers stay whole.
fn format(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

/// Clamps a value to the bounds and rounds it to the nearest step, keeping the
/// precision of the step so that dragging a slider of 0.1 doesn't produce
/// values like 0.30000000000000004.
fn clamp_to_step(value: f64, config: &Config) -> f64 {
    let stepped = (value / config.step).round() * config.step;
    let clamped = stepped.clamp(config.min, config.max);

    let precision = format!("{}", config.step)
        .split_once('.')
        .map(|(_, fraction)| fraction.trim_end_matches('0').len())
        .unwrap_or(0);
    let factor = 10f64.powi(precision as i32);
    (clamped * factor).round() / factor
}

fn percentage(value: f64, config: &Config) -> f64 {
    if config.max <= config.min {
        return 0.0;
    }
    (((value - config.min) / (config.max - config.min)) * 100.0).clamp(0.0, 100.0)
}

fn thumb_of(slider: &Element, thumb: Thumb) -> Option<Element> {
    slider.query_selector(thumb.selector()).ok()?
}

/// The element carrying the slider role and the value of `thumb`: the thumb
/// itself in a range slider, the control otherwise.
fn value_holder(slider: &Element, thumb: Thumb) -> Option<Element> {
    match thumb {
        Thumb::Single => slider.query_selector(concatcp!('.', SLIDER_CONTROL)).ok()?,
        _ => thumb_of(slider, thumb),
    }
}

fn value_of(slider: &Element, thumb: Thumb) -> Option<f64> {
    number_attr(&value_holder(slider, thumb)?, "aria-valuenow")
}

fn hidden_input(slider: &Element, thumb: Thumb) -> Option<HtmlInputElement> {
    let mut inputs = dom::existing::select_all_elements_from(slider, "input[type='hidden']");
    let index = if thumb == Thumb::Max { 1 } else { 0 };
    inputs
        .nth(index)
        .and_then(|input| input.dyn_into::<HtmlInputElement>().ok())
}

/// The tooltip anchored to `thumb`, if the slider renders one.
fn tooltip_of(slider: &Element, thumb: Thumb) -> Option<Element> {
    let id = thumb_of(slider, thumb)?.id();
    if id.is_empty() {
        return None;
    }
    dom::existing::document()
        .query_selector(&format!(".tooltip .popup[data-anchor='{id}']"))
        .ok()?
        .and_then(|popup| popup.closest(".tooltip").ok().flatten())
}

fn set_tooltips_open(slider: &Element, open: bool) {
    for thumb in [Thumb::Single, Thumb::Min, Thumb::Max] {
        if let Some(tooltip) = tooltip_of(slider, thumb) {
            tooltip::set_tooltip_open(&tooltip, open, slider);
        }
    }
}

/// Writes a value back into the markup and reports it with an `input` event,
/// the way `wa-slider` does: the thumb position, the filled part of the track,
/// the exposed value and the one the form submits.
fn set_value(slider: &Element, thumb: Thumb, value: f64, config: &Config) -> Option<()> {
    let value = clamp_to_step(value, config);

    // The thumbs of a range slider never cross each other
    let value = match thumb {
        Thumb::Min => value.min(value_of(slider, Thumb::Max).unwrap_or(config.max)),
        Thumb::Max => value.max(value_of(slider, Thumb::Min).unwrap_or(config.min)),
        Thumb::Single => value,
    };

    let holder = value_holder(slider, thumb)?;
    let previous = number_attr(&holder, "aria-valuenow");
    let text = format(value);
    holder.set_attribute("aria-valuenow", &text).ok();
    holder.set_attribute("aria-valuetext", &text).ok();

    if let Some(thumb_element) = thumb_of(slider, thumb).and_then(|thumb| thumb.maybe_into_html()) {
        thumb_element
            .style()
            .set_property("--position", &format!("{}%", percentage(value, config)))
            .ok();
    }

    if let Some(input) = hidden_input(slider, thumb) {
        input.set_value(&text);
    }

    if let Some(tooltip) = tooltip_of(slider, thumb) {
        if let Some(body) = tooltip.query_selector(".tooltip-body").ok().flatten() {
            body.set_text_content(Some(&text));
        }
        // The popup is placed when it is shown, so a moving thumb has to ask
        // for a new position itself
        if let Some(host) = tooltip.query_selector(":scope > .popup").ok().flatten() {
            popup::reposition(&host);
        }
    }

    update_indicator(slider, config);

    // Only a real change is worth reporting: dragging fires on every pointer move
    if previous != Some(value) {
        event::dispatch(slider, event::INPUT, true).ok();
    }

    Some(())
}

/// Redraws the filled part of the track from the current values.
fn update_indicator(slider: &Element, config: &Config) -> Option<()> {
    let (start, end) = if config.range {
        let min = percentage(value_of(slider, Thumb::Min)?, config);
        let max = percentage(value_of(slider, Thumb::Max)?, config);
        (min.min(max), min.max(max))
    } else {
        let offset = percentage(config.indicator_offset.unwrap_or(config.min), config);
        let value = percentage(value_of(slider, Thumb::Single)?, config);
        (offset.min(value), offset.max(value))
    };

    let indicator = slider.query_selector(".slider-indicator").ok()??.maybe_into_html()?;
    indicator.style().set_property("--start", &format!("{start}%")).ok();
    indicator.style().set_property("--end", &format!("{end}%")).ok();

    Some(())
}

/// The value a pointer at `(x, y)` points at, following the orientation and the
/// text direction.
fn value_from_pointer(slider: &Element, x: f64, y: f64, config: &Config) -> Option<f64> {
    let track = slider.query_selector(".slider-track").ok()??;
    let rect = track.get_bounding_client_rect();

    let fraction = if config.vertical {
        if rect.height() == 0.0 {
            return None;
        }
        (rect.bottom() - y) / rect.height()
    } else if rect.width() == 0.0 {
        return None;
    } else if is_rtl(slider) {
        (rect.right() - x) / rect.width()
    } else {
        (x - rect.left()) / rect.width()
    };

    Some(config.min + fraction.clamp(0.0, 1.0) * (config.max - config.min))
}

/// Remembers the thumb the user is working with, so the next drag of two thumbs
/// sitting on the same value keeps moving that one instead of jumping to its
/// neighbour (`activeThumb` in `wa-slider`).
fn set_active_thumb(slider: &Element, thumb: Thumb) {
    if let Thumb::Min | Thumb::Max = thumb {
        let value = if thumb == Thumb::Min { "min" } else { "max" };
        slider.set_attribute("data-active-thumb", value).ok();
    }
}

fn active_thumb(slider: &Element) -> Option<Thumb> {
    match slider.get_attribute("data-active-thumb")?.as_str() {
        "min" => Some(Thumb::Min),
        "max" => Some(Thumb::Max),
        _ => None,
    }
}

/// The thumb a drag starting at `value` moves: the closest one. When both are
/// equally close, the pointer decides — it moves the thumb it is beyond — and
/// the one already in use wins a tie between the thumbs themselves.
fn thumb_for(slider: &Element, value: f64, config: &Config) -> Thumb {
    if !config.range {
        return Thumb::Single;
    }

    let min = value_of(slider, Thumb::Min).unwrap_or(config.min);
    let max = value_of(slider, Thumb::Max).unwrap_or(config.max);
    let (min_distance, max_distance) = ((value - min).abs(), (value - max).abs());

    if min_distance < max_distance || value < min {
        Thumb::Min
    } else if max_distance < min_distance || value > max {
        Thumb::Max
    } else {
        // Both thumbs sit on the pointer: keep the one that was moved last
        active_thumb(slider).unwrap_or(Thumb::Max)
    }
}

//
// Dragging
//

/// Starts a drag when the track is pressed, moving the thumb to the pointer and
/// following it until the pointer is released.
fn handle_pointer_down(event: &Event) -> Option<()> {
    let pointer: &PointerEvent = event.dyn_ref()?;
    let target = event.target()?.maybe_into_element()?;
    let slider = target.closest(".slider").ok()??;
    target.closest(".slider-control").ok()??;

    let config = slider_config(&slider);
    if !config.interactive {
        return None;
    }

    // No `preventDefault` here: it would block the native focus the click gives
    // the control, and the programmatic focus that replaced it made the browser
    // treat a plain click as keyboard navigation and draw the focus ring.
    let value = value_from_pointer(&slider, pointer.client_x(), pointer.client_y(), &config)?;
    let thumb = thumb_for(&slider, value, &config);
    let value_when_dragging_started = value_of(&slider, thumb);
    set_active_thumb(&slider, thumb);
    set_value(&slider, thumb, value, &config);
    set_tooltips_open(&slider, true);

    let root = dom::existing::document_element();

    let drag_move = dom::event::js_function({
        let slider = slider.clone();
        move |event: Event| {
            let Some(pointer) = event.dyn_ref::<PointerEvent>() else {
                return;
            };
            let config = slider_config(&slider);
            if let Some(value) = value_from_pointer(&slider, pointer.client_x(), pointer.client_y(), &config) {
                set_value(&slider, thumb, value, &config);
            }
        }
    });

    // The slot lets the stop handler remove itself.
    let stop_slot: Rc<RefCell<Option<Function>>> = Rc::new(RefCell::new(None));
    let drag_stop = dom::event::js_function({
        let slider = slider.clone();
        let root = root.clone();
        let drag_move = drag_move.clone();
        let stop_slot = stop_slot.clone();
        move |_| {
            root.remove_event_listener_with_callback("pointermove", &drag_move).ok();
            if let Some(stop) = stop_slot.borrow().as_ref() {
                root.remove_event_listener_with_callback("pointerup", stop).ok();
                root.remove_event_listener_with_callback("pointercancel", stop).ok();
            }

            // A drag reports its result once, when the pointer is released
            if value_of(&slider, thumb) != value_when_dragging_started {
                event::dispatch(&slider, event::CHANGE, true).ok();
            }

            // The tooltip stays while the slider keeps the focus
            if !is_focused(&slider) {
                set_tooltips_open(&slider, false);
            }
        }
    });
    *stop_slot.borrow_mut() = Some(drag_stop.clone());

    root.add_event_listener_with_callback("pointermove", &drag_move).ok();
    root.add_event_listener_with_callback("pointerup", &drag_stop).ok();
    root.add_event_listener_with_callback("pointercancel", &drag_stop).ok();

    Some(())
}

fn is_focused(slider: &Element) -> bool {
    dom::existing::document()
        .active_element()
        .is_some_and(|active| slider.contains(Some(&active)))
}

//
// Keyboard
//

pub fn handle_keydown(event: &Event) -> Option<()> {
    let keyboard: &KeyboardEvent = event.dyn_ref()?;
    let target = event.target()?.maybe_into_element()?;
    let slider = target.closest(".slider").ok()??;

    let config = slider_config(&slider);
    if !config.interactive {
        return None;
    }

    let thumb = if target.class_list().contains(SLIDER_THUMB_MIN) {
        Thumb::Min
    } else if target.class_list().contains(SLIDER_THUMB_MAX) {
        Thumb::Max
    } else if config.range {
        return None;
    } else {
        Thumb::Single
    };

    let current = value_of(&slider, thumb)?;
    let rtl = is_rtl(&slider);
    let page = ((config.max - config.min) / 10.0).max(config.step);

    let value = match keyboard.key().as_str() {
        "ArrowUp" => current + config.step,
        "ArrowDown" => current - config.step,
        "ArrowRight" => {
            if rtl {
                current - config.step
            } else {
                current + config.step
            }
        },
        "ArrowLeft" => {
            if rtl {
                current + config.step
            } else {
                current - config.step
            }
        },
        "Home" => config.min,
        "End" => config.max,
        "PageUp" => current + page,
        "PageDown" => current - page,
        _ => return None,
    };

    event.prevent_default();
    set_active_thumb(&slider, thumb);
    set_value(&slider, thumb, value, &config);
    if value_of(&slider, thumb) != Some(current) {
        event::dispatch(&slider, event::CHANGE, true).ok();
    }

    Some(())
}

//
// Initialization
//

/// Synchronizes every slider on the page with the values in its markup. Run it
/// after every render.
pub fn init_sliders() {
    for slider in dom::existing::select_all_elements(".slider") {
        let config = slider_config(&slider);
        for thumb in [Thumb::Single, Thumb::Min, Thumb::Max] {
            if let Some(value) = value_of(&slider, thumb) {
                set_value(&slider, thumb, value, &config);
            }
        }
    }
}

/// Installs the document-level listeners that drive all sliders on the page.
pub fn listen_sliders() {
    let document = dom::existing::document();

    document.add_steady_event_listener("pointerdown", |event| {
        handle_pointer_down(&event);
    });
    document.add_steady_event_listener("keydown", |event| {
        handle_keydown(&event);
    });

    // The tooltip follows the focus, as in `wa-slider`
    document.add_steady_event_listener("focusin", |event| {
        show_tooltips_on_focus(&event, true);
    });
    document.add_steady_event_listener("focusout", |event| {
        show_tooltips_on_focus(&event, false);
    });
}

fn show_tooltips_on_focus(event: &Event, open: bool) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let slider = target.closest(".slider").ok()??;

    if open {
        if target.class_list().contains(SLIDER_THUMB_MIN) {
            set_active_thumb(&slider, Thumb::Min);
        } else if target.class_list().contains(SLIDER_THUMB_MAX) {
            set_active_thumb(&slider, Thumb::Max);
        }
    }

    set_tooltips_open(&slider, open);
    Some(())
}
