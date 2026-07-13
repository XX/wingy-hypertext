use std::cell::RefCell;

use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Element, Event, HtmlInputElement, IntersectionObserver, IntersectionObserverEntry};
use wingy_hypertext::action::ActionSetters;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{CLUSTER, GAP_L};
use wingy_hypertext::components::button::Button;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::input::Input;
use wingy_hypertext::components::input::InputType::Number;
use wingy_hypertext::components::select::{Select, SelectOption};
use wingy_hypertext::helpers::animation::Animation;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layouts::divider::Divider;
use wingy_hypertext::variant::Variant::*;
use wingy_hypertext_web::helpers::animation::{
    animation_names, create_animation, easing_names, set_animation_current_time, set_animation_play,
    set_animation_playback_rate,
};
use wingy_hypertext_web::utils::action::register_action;

const BOX_STYLE: &str = "display: inline-block; width: 100px; height: 100px; background-color: var(--wa-color-brand-fill-loud); margin: 1.5rem;";
const SANDBOX_BOX_STYLE: &str = "width: 100px; height: 100px; background-color: var(--wa-color-brand-fill-loud);";

/// The custom keyframes of the "Custom Keyframe Formats" example, carried to
/// the client as JSON in the `data-keyframes` attribute.
const CUSTOM_KEYFRAMES: &str = r#"[
    {
        "offset": 0,
        "easing": "cubic-bezier(0.250, 0.460, 0.450, 0.940)",
        "fillMode": "both",
        "transformOrigin": "center center",
        "transform": "rotate(0)"
    },
    {
        "offset": 1,
        "easing": "cubic-bezier(0.250, 0.460, 0.450, 0.940)",
        "fillMode": "both",
        "transformOrigin": "center center",
        "transform": "rotate(90deg)"
    }
]"#;

fn animated_box() -> impl Renderable {
    rsx! { <div style=BOX_STYLE></div> }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Animation"</Head>
        <p>"Animate elements declaratively with nearly 100 baked-in presets, or roll your own with custom "
            "keyframes. Powered by the Web Animations API. To animate an element, wrap it in "<code>Animation</code>
            " and set an animation "<code>name</code>". The animation will not start until you set the "
            <code>play</code>" attribute. The playback logic is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "<code>"init_animations"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview>
                <div class="animation-overview">
                    <Animation name="bounce" duration=2000 play=true>(animated_box())</Animation>
                    <Animation name="jello" duration=2000 play=true>(animated_box())</Animation>
                    <Animation name="heartBeat" duration=2000 play=true>(animated_box())</Animation>
                    <Animation name="flip" duration=2000 play=true>(animated_box())</Animation>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="animation-overview">
                        <Animation name="bounce" duration=2000 play=true><div class="box"></div></Animation>
                        <Animation name="jello" duration=2000 play=true><div class="box"></div></Animation>
                        <Animation name="heartBeat" duration=2000 play=true><div class="box"></div></Animation>
                        <Animation name="flip" duration=2000 play=true><div class="box"></div></Animation>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"The animation will only be applied to the first child element found in "<code>Animation</code>
            ". To animate multiple elements, either wrap them in a single container or use multiple "
            <code>Animation</code>"s."
        </p>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="animations-easings" anchor=true>
            "Animations & Easings"
        </Head>
        <p>"This example demonstrates all of the baked-in animations and easings. Animations are based on those "
            "found in the popular "<a href="https://animate.style/" target="_blank">"Animate.css"</a>" library. "
            "The lists of names are provided by "<code>"animation_names"</code>" and "<code>"easing_names"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview>
                <div class="animation-sandbox">
                    <Animation name="bounce" easing="ease-in-out" duration=2000 play=true>
                        <div style=SANDBOX_BOX_STYLE></div>
                    </Animation>
                    <Divider/>
                    <div class=(CLUSTER, " ", GAP_L) style="align-items: end;">
                        <Select label="Animation" name="animation" style="min-width: 12em;">
                            @for name in animation_names() {
                                <SelectOption value=(name.clone()) selected=(name == "bounce")>(name.clone())</SelectOption>
                            }
                        </Select>
                        <Select label="Easing" name="easing" style="min-width: 12em;">
                            @for name in easing_names() {
                                <SelectOption value=(name.clone()) selected=(name == "easeInOut")>(name.clone())</SelectOption>
                            }
                        </Select>
                        <Input input_type=Number label="Playback Rate" name="playback-rate" value="1" style="width: 8em;"/>
                    </div>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="animation-sandbox">
                        <Animation name="bounce" easing="ease-in-out" duration=2000 play=true>
                            <div class="box"></div>
                        </Animation>
                        <Divider/>
                        <div class=(CLUSTER, " ", GAP_L)>
                            <Select label="Animation" name="animation">
                                @for name in animation_names() {
                                    <SelectOption value=(name.clone()) selected=(name == "bounce")>(name.clone())</SelectOption>
                                }
                            </Select>
                            <Select label="Easing" name="easing">
                                @for name in easing_names() {
                                    <SelectOption value=(name.clone()) selected=(name == "easeInOut")>(name.clone())</SelectOption>
                                }
                            </Select>
                            <Input input_type=Number label="Playback Rate" name="playback-rate" value="1"/>
                        </div>
                    </div>

                    // The controls are wired to the animation with a delegated listener,
                    // see `listen_animation_overview` in examples/client/src/helpers/animation.rs
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="intersection-observer" anchor=true>
            "Using Intersection Observer"
        </Head>
        <p>"Use an Intersection Observer to control the animation when an element enters or exits the viewport. "
            "For example, scroll the box below in and out of your screen. The animation stops when the box exits "
            "the viewport and restarts each time it enters the viewport."
        </p>
        <CodeExample>
            <CodeExamplePreview>
                <div class="animation-scroll">
                    <Animation name="jackInTheBox" duration=2000 iterations=(1.0)>(animated_box())</Animation>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="animation-scroll">
                        <Animation name="jackInTheBox" duration=2000 iterations=(1.0)><div class="box"></div></Animation>
                    </div>

                    // An IntersectionObserver watches the box (not the animation host!)
                    // and toggles the playback, see `init_animation_scroll` in
                    // examples/client/src/helpers/animation.rs:
                    //
                    // if intersecting {
                    //     // Start the animation when the box enters the viewport
                    //     set_animation_play(&host, true);
                    // } else {
                    //     set_animation_play(&host, false);
                    //     set_animation_current_time(&host, 0.0);
                    // }
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="custom-keyframes" anchor=true>
            "Custom Keyframe Formats"
        </Head>
        <p>"Supply your own keyframe formats to build custom animations: pass a JSON array of keyframe objects "
            "with the "<code>keyframes</code>" attribute. When it is set, "<code>name</code>" is ignored."
        </p>
        <CodeExample>
            <CodeExamplePreview>
                <div class="animation-keyframes">
                    <Animation easing="ease-in-out" duration=2000 play=true keyframes=CUSTOM_KEYFRAMES>
                        <div style=SANDBOX_BOX_STYLE></div>
                    </Animation>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r##"
                    <Animation
                        easing="ease-in-out"
                        duration=2000
                        play=true
                        keyframes=r#"[
                            {
                                "offset": 0,
                                "easing": "cubic-bezier(0.250, 0.460, 0.450, 0.940)",
                                "fillMode": "both",
                                "transformOrigin": "center center",
                                "transform": "rotate(0)"
                            },
                            {
                                "offset": 1,
                                "easing": "cubic-bezier(0.250, 0.460, 0.450, 0.940)",
                                "fillMode": "both",
                                "transformOrigin": "center center",
                                "transform": "rotate(90deg)"
                            }
                        ]"#
                    >
                        <div class="box"></div>
                    </Animation>
                "##</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="on-demand" anchor=true>
            "Playing Animations on Demand"
        </Head>
        <p>"Animations won't play until you apply the "<code>play</code>" attribute. You can omit it initially, "
            "then apply it on demand such as after a user interaction. In this example, the button will animate "
            "once every time the button is clicked."
        </p>
        <CodeExample>
            <CodeExamplePreview>
                <div class="animation-form">
                    <Animation name="rubberBand" duration=1000 iterations=(1.0)>
                        <Button appearance=Filled variant=Brand action="play-animation">"Click me"</Button>
                    </Animation>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="animation-form">
                        <Animation name="rubberBand" duration=1000 iterations=(1.0)>
                            <Button appearance=Filled variant=Brand action="play-animation">"Click me"</Button>
                        </Animation>
                    </div>

                    // The `play-animation` action starts the animation on click:
                    //
                    // register_action("play-animation", |_args, ctx| {
                    //     if let Ok(Some(host)) = ctx.element.closest(".animation") {
                    //         set_animation_play(&host, true);
                    //     }
                    // });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive demo wiring
//

thread_local! {
    /// The Intersection Observer of the scroll example, disconnected and
    /// recreated on every render.
    static SCROLL_OBSERVER: RefCell<Option<IntersectionObserver>> = const { RefCell::new(None) };
}

/// One-time wiring: any change to the controls inside `.animation-sandbox` is
/// applied to the demo animation, and clicking a `play-animation` action
/// starts the closest animation.
pub fn listen_animation_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener("change", |event| {
        handle_sandbox_event(&event);
    });
    document.add_steady_event_listener("input", |event| {
        handle_sandbox_event(&event);
    });

    register_action("play-animation", |_args, ctx| {
        if let Ok(Some(host)) = ctx.element.closest(".animation") {
            set_animation_play(&host, true);
        }
    });
}

/// Applies the initial control state to the demo animation and installs the
/// Intersection Observer of the scroll example. Run it after every render.
pub fn init_animation_overview() {
    if let Some(sandbox) = dom::existing::document()
        .query_selector(".animation-sandbox")
        .ok()
        .flatten()
    {
        sync_sandbox(&sandbox);
    }

    init_animation_scroll();
}

fn handle_sandbox_event(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let sandbox = target.closest(".animation-sandbox").ok()??;
    sync_sandbox(&sandbox)
}

fn control_value(sandbox: &Element, selector: &str) -> Option<String> {
    let input: HtmlInputElement = sandbox.query_selector(selector).ok()??.dyn_into().ok()?;
    Some(input.value())
}

/// Reads the control values and applies them to the animation's `data-*`
/// configuration. Changing the name or easing recreates the animation;
/// the playback rate is applied without a restart.
fn sync_sandbox(sandbox: &Element) -> Option<()> {
    let host = sandbox.query_selector(":scope > .animation").ok()??;

    let name = control_value(sandbox, ".value-input[name='animation']")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "bounce".to_string());
    let easing = control_value(sandbox, ".value-input[name='easing']")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "easeInOut".to_string());
    let playback_rate = control_value(sandbox, ".control[name='playback-rate']")
        .and_then(|value| value.parse().ok())
        .unwrap_or(1.0);

    let changed = host.get_attribute("data-name").as_deref() != Some(&name)
        || host.get_attribute("data-easing").as_deref() != Some(&easing);

    host.set_attribute("data-name", &name).ok();
    host.set_attribute("data-easing", &easing).ok();
    if changed {
        create_animation(&host);
    }
    set_animation_playback_rate(&host, playback_rate);

    Some(())
}

/// Watches the box of the scroll example entering and exiting the viewport:
/// the animation restarts on enter and stops on exit. Note that the observer
/// watches the box, not the animation host.
fn init_animation_scroll() {
    SCROLL_OBSERVER.with(|observer| {
        if let Some(observer) = observer.borrow_mut().take() {
            observer.disconnect();
        }
    });

    let Some(host) = dom::existing::document()
        .query_selector(".animation-scroll .animation")
        .ok()
        .flatten()
    else {
        return;
    };
    let Some(watched_box) = host.first_element_child() else {
        return;
    };

    let callback_host = host.clone();
    let callback = Closure::<dyn FnMut(js_sys::Array)>::new(move |entries: js_sys::Array| {
        let intersecting = entries
            .get(0)
            .dyn_into::<IntersectionObserverEntry>()
            .map(|entry| entry.is_intersecting())
            .unwrap_or(false);

        if intersecting {
            // Start the animation when the box enters the viewport
            set_animation_play(&callback_host, true);
        } else {
            set_animation_play(&callback_host, false);
            set_animation_current_time(&callback_host, 0.0);
        }
    });

    if let Ok(observer) = IntersectionObserver::new(callback.as_ref().unchecked_ref()) {
        observer.observe(&watched_box);
        SCROLL_OBSERVER.with(|slot| *slot.borrow_mut() = Some(observer));
    }
    callback.forget();
}
