//! A Rust port of the `wa-animation` playback behavior: animates the first
//! element child of the `.animation` host with the Web Animations API, using
//! the baked-in presets from `@shoelace-style/animations` (embedded as JSON)
//! or custom keyframes. The configuration is read from `data-*` attributes on
//! the host element rendered by `wingy_hypertext::helpers::animation` and must
//! be wired up on the client with `init_animations`.

use js_sys::{JSON, Object, Reflect};
use wasm_bindgen::prelude::*;
use wasm_dom as dom;
use wasm_dom::existing::access::CastToElement;
use web_sys::{
    Animation, AnimationPlayState, Element, Event, EventInit, FillMode, KeyframeAnimationOptions, PlaybackDirection,
};

/// The name of the event emitted on the host when the animation starts or restarts.
pub const START_EVENT: &str = "wa-start";

/// The name of the event emitted on the host when the animation finishes.
pub const FINISH_EVENT: &str = "wa-finish";

/// The name of the event emitted on the host when the animation is canceled.
pub const CANCEL_EVENT: &str = "wa-cancel";

/// The `Animation.id` marking the animations created by this module, so they
/// can be told apart from CSS transitions and animations of the same element
/// in `getAnimations()`.
const ANIMATION_ID: &str = "wingy-animation";

/// The baked-in animation keyframes and easing functions from
/// `@shoelace-style/animations`, as `{ "animations": {...}, "easings": {...} }`.
const PRESETS_JSON: &str = include_str!("animations.json");

thread_local! {
    static PRESETS: Object = JSON::parse(PRESETS_JSON)
        .expect_throw("cannot parse the animation presets JSON")
        .unchecked_into();
}

fn presets_entry(key: &str) -> Object {
    PRESETS
        .with(|presets| Reflect::get(presets, &JsValue::from_str(key)))
        .expect_throw("cannot read the animation presets entry")
        .unchecked_into()
}

fn entry_names(key: &str) -> Vec<String> {
    Object::keys(&presets_entry(key))
        .iter()
        .filter_map(|name| name.as_string())
        .collect()
}

/// Gets a list of all supported animation names.
pub fn animation_names() -> Vec<String> {
    entry_names("animations")
}

/// Gets a list of all supported easing function names.
pub fn easing_names() -> Vec<String> {
    entry_names("easings")
}

/// The keyframes of a built-in animation, as a JS array of keyframe objects.
fn preset_keyframes(name: &str) -> Option<JsValue> {
    let keyframes = Reflect::get(&presets_entry("animations"), &JsValue::from_str(name)).ok()?;
    (!keyframes.is_undefined()).then_some(keyframes)
}

/// Resolves a baked-in easing name to its easing function; custom easing
/// functions (e.g. `cubic-bezier(...)`) pass through unchanged.
fn resolve_easing(easing: &str) -> String {
    Reflect::get(&presets_entry("easings"), &JsValue::from_str(easing))
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or_else(|| easing.to_string())
}

struct AnimationConfig {
    name: String,
    play: bool,
    delay: f64,
    direction: PlaybackDirection,
    duration: f64,
    easing: String,
    end_delay: f64,
    fill: FillMode,
    iterations: f64,
    iteration_start: f64,
    keyframes: Option<String>,
    playback_rate: f64,
}

fn number_attr(host: &Element, name: &str, default: f64) -> f64 {
    host.get_attribute(name)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn read_config(host: &Element) -> AnimationConfig {
    let direction = match host.get_attribute("data-direction").as_deref() {
        Some("reverse") => PlaybackDirection::Reverse,
        Some("alternate") => PlaybackDirection::Alternate,
        Some("alternate-reverse") => PlaybackDirection::AlternateReverse,
        _ => PlaybackDirection::Normal,
    };
    let fill = match host.get_attribute("data-fill").as_deref() {
        Some("none") => FillMode::None,
        Some("forwards") => FillMode::Forwards,
        Some("backwards") => FillMode::Backwards,
        Some("both") => FillMode::Both,
        _ => FillMode::Auto,
    };

    AnimationConfig {
        name: host.get_attribute("data-name").unwrap_or_else(|| "none".to_string()),
        play: host.has_attribute("data-play"),
        delay: number_attr(host, "data-delay", 0.0),
        direction,
        duration: number_attr(host, "data-duration", 1000.0),
        easing: host
            .get_attribute("data-easing")
            .unwrap_or_else(|| "linear".to_string()),
        end_delay: number_attr(host, "data-end-delay", 0.0),
        fill,
        iterations: number_attr(host, "data-iterations", f64::INFINITY),
        iteration_start: number_attr(host, "data-iteration-start", 0.0),
        keyframes: host.get_attribute("data-keyframes"),
        playback_rate: number_attr(host, "data-playback-rate", 1.0),
    }
}

/// The animation applies to the first element child of the host; subsequent
/// children are ignored, like the default slot of `wa-animation`.
fn animated_element(host: &Element) -> Option<Element> {
    host.first_element_child()
}

/// The animation currently applied to the animated element, identified by the
/// `ANIMATION_ID` marker — the element may also carry unrelated CSS
/// transitions (e.g. a hovered button), which `getAnimations()` sorts first.
/// Canceled animations (and finished ones that don't fill) are no longer
/// returned by `getAnimations()`, in which case the animation is recreated on
/// demand.
fn current_animation(host: &Element) -> Option<Animation> {
    animated_element(host)?
        .get_animations()
        .iter()
        .map(|animation| animation.unchecked_into::<Animation>())
        .find(|animation| animation.id() == ANIMATION_ID)
}

fn dispatch(host: &Element, name: &str) {
    let init = EventInit::new();
    init.set_bubbles(true);
    if let Ok(event) = Event::new_with_event_init_dict(name, &init) {
        host.dispatch_event(&event).ok();
    }
}

/// Clears all keyframe effects caused by the host's animation and aborts its playback.
pub fn destroy_animation(host: &Element) {
    if let Some(animation) = current_animation(host) {
        animation.set_onfinish(None);
        animation.set_oncancel(None);
        animation.cancel();
    }
}

/// (Re)creates the animation from the host's `data-*` configuration, canceling
/// any previous animation of the element. The animation starts playing when
/// `data-play` is set and is created paused otherwise.
pub fn create_animation(host: &Element) -> Option<()> {
    let config = read_config(host);
    let element = animated_element(host)?;
    let keyframes = match &config.keyframes {
        Some(keyframes) => JSON::parse(keyframes).ok()?,
        None => preset_keyframes(&config.name)?,
    };

    destroy_animation(host);

    let options = KeyframeAnimationOptions::new();
    options.set_delay(config.delay);
    options.set_direction(config.direction);
    options.set_duration(config.duration);
    options.set_easing(&resolve_easing(&config.easing));
    options.set_end_delay(config.end_delay);
    options.set_fill(config.fill);
    options.set_iteration_start(config.iteration_start);
    options.set_iterations(config.iterations);

    let animation = element.animate_with_keyframe_animation_options(keyframes.dyn_ref::<Object>(), &options);
    animation.set_id(ANIMATION_ID);
    animation.set_playback_rate(config.playback_rate);

    // Like the `play` attribute of `wa-animation`, `data-play` is removed when
    // the animation finishes or gets canceled.
    let finish_host = host.clone();
    let on_finish = dom::event::js_function(move |_| {
        finish_host.remove_attribute("data-play").ok();
        dispatch(&finish_host, FINISH_EVENT);
    });
    animation.set_onfinish(Some(&on_finish));

    let cancel_host = host.clone();
    let on_cancel = dom::event::js_function(move |_| {
        cancel_host.remove_attribute("data-play").ok();
        dispatch(&cancel_host, CANCEL_EVENT);
    });
    animation.set_oncancel(Some(&on_cancel));

    if config.play {
        dispatch(host, START_EVENT);
    } else {
        animation.pause().ok();
    }

    Some(())
}

/// Plays or pauses the animation, syncing the `data-play` attribute. A
/// finished or canceled animation is recreated, so setting `play` again
/// restarts it — this is how one-shot animations are replayed on demand.
pub fn set_animation_play(host: &Element, play: bool) {
    if play {
        host.set_attribute("data-play", "").ok();
    } else {
        host.remove_attribute("data-play").ok();
    }

    match current_animation(host) {
        Some(animation) if animation.play_state() != AnimationPlayState::Finished => {
            if play {
                if animation.play_state() != AnimationPlayState::Running {
                    dispatch(host, START_EVENT);
                }
                animation.play().ok();
            } else {
                animation.pause().ok();
            }
        },
        _ => {
            if play {
                create_animation(host);
            }
        },
    }
}

/// Changes the playback rate without causing the animation to restart. The
/// default is `1`; a negative value plays the animation in reverse.
pub fn set_animation_playback_rate(host: &Element, playback_rate: f64) {
    host.set_attribute("data-playback-rate", &playback_rate.to_string())
        .ok();

    if let Some(animation) = current_animation(host) {
        animation.set_playback_rate(playback_rate);
    }
}

/// Sets the current animation time, in milliseconds.
pub fn set_animation_current_time(host: &Element, time: f64) {
    if let Some(animation) = current_animation(host) {
        animation.set_current_time(Some(time));
    }
}

/// Sets the playback time to the end of the animation corresponding to the
/// current playback direction.
pub fn finish_animation(host: &Element) {
    if let Some(animation) = current_animation(host) {
        animation.finish().ok();
    }
}

/// Creates the animation for every `.animation` host on the page. Run it after
/// every render.
pub fn init_animations() {
    let Ok(hosts) = dom::existing::document().query_selector_all(".animation") else {
        return;
    };
    for i in 0..hosts.length() {
        if let Some(host) = hosts.get(i).and_then(|node| node.maybe_into_element()) {
            create_animation(&host);
        }
    }
}
