use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Element, Event, HtmlInputElement};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{CLUSTER, GAP_L};
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::input::Input;
use wingy_hypertext::components::input::InputType::Number;
use wingy_hypertext::components::select::{Select, SelectOption};
use wingy_hypertext::components::switch::Switch;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layouts::divider::Divider;
use wingy_hypertext::layouts::popup::Placement::*;
use wingy_hypertext::layouts::popup::{AutoSize, Popup, SyncSize};
use wingy_hypertext_web::layouts::popup::set_popup_active;

const ANCHOR_STYLE: &str = "display: inline-block; width: 150px; height: 150px; border: dashed 2px var(--wa-color-neutral-fill-loud); margin: 50px;";
const BOX_STYLE: &str = "width: 100px; height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);";

fn anchor() -> impl Renderable {
    rsx! { <span style=ANCHOR_STYLE></span> }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Popup"</Head>
        <p>"Popup is a low-level utility that anchors one element to another and keeps them positioned "
            "together as the page scrolls or resizes. It provides positioning only — no styles — and serves "
            "as a building block for popovers, dropdowns, and tooltips (the "<code>Select</code>
            " listbox is positioned by it). The positioning logic is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "
            <code>"listen_popups"</code>" and "<code>"init_popups"</code>
            "; the popups in the sections below are statically "<code>active</code>" for demonstration."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="popup-overview">
                    <Popup
                        placement=Top
                        arrow=true
                        active=true
                        style="--arrow-color: var(--wa-color-brand-fill-loud)"
                        anchor=(anchor())
                    >
                        <div style=BOX_STYLE></div>
                    </Popup>
                    <Divider/>
                    <div class=(CLUSTER, " ", GAP_L) style="align-items: end;">
                        <Select label="Placement" name="placement" style="min-width: 10em;">
                            <SelectOption value="top" selected=true>"top"</SelectOption>
                            <SelectOption value="top-start">"top-start"</SelectOption>
                            <SelectOption value="top-end">"top-end"</SelectOption>
                            <SelectOption value="bottom">"bottom"</SelectOption>
                            <SelectOption value="bottom-start">"bottom-start"</SelectOption>
                            <SelectOption value="bottom-end">"bottom-end"</SelectOption>
                            <SelectOption value="right">"right"</SelectOption>
                            <SelectOption value="right-start">"right-start"</SelectOption>
                            <SelectOption value="right-end">"right-end"</SelectOption>
                            <SelectOption value="left">"left"</SelectOption>
                            <SelectOption value="left-start">"left-start"</SelectOption>
                            <SelectOption value="left-end">"left-end"</SelectOption>
                        </Select>
                        <Input input_type=Number label="Distance" name="distance" value="0" style="width: 7em;"/>
                        <Input input_type=Number label="Skidding" name="skidding" value="0" style="width: 7em;"/>
                        <Switch name="active" checked=true>"Active"</Switch>
                        <Switch name="arrow">"Arrow"</Switch>
                    </div>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="popup-overview">
                        <Popup placement=Top arrow=true active=true anchor=(rsx! { <span class="anchor"></span> })>
                            <div class="box"></div>
                        </Popup>
                        <Divider/>
                        <div class=(CLUSTER, " ", GAP_L)>
                            <Select label="Placement" name="placement">
                                <SelectOption value="top" selected=true>"top"</SelectOption>
                                ...
                            </Select>
                            <Input input_type=Number label="Distance" name="distance" value="0"/>
                            <Input input_type=Number label="Skidding" name="skidding" value="0"/>
                            <Switch name="active" checked=true>"Active"</Switch>
                            <Switch name="arrow">"Arrow"</Switch>
                        </div>
                    </div>

                    // The controls are wired to the popup with a delegated listener,
                    // see `listen_popup_overview` in examples/client/src/layouts/popup.rs
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="placement" anchor=true>
            "Placement"
        </Head>
        <p>"Use the "<code>placement</code>" attribute to set the preferred placement of the popup: "
            "four sides with optional "<code>"-start"</code>" and "<code>"-end"</code>" alignments."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=TopStart active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"top-start"</div>
                </Popup>
                <Popup placement=Right active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"right"</div>
                </Popup>
                <Popup placement=BottomEnd active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"bottom-end"</div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=TopStart active=true anchor=(...)>...</Popup>
                    <Popup placement=Right active=true anchor=(...)>...</Popup>
                    <Popup placement=BottomEnd active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="distance-skidding" anchor=true>
            "Distance & Skidding"
        </Head>
        <p>"Use the "<code>distance</code>" attribute to offset the popup away from its anchor, and "
            <code>skidding</code>" to offset it along the anchor."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Top distance=24 active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"distance"</div>
                </Popup>
                <Popup placement=Top skidding=40 active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"skidding"</div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Top distance=24 active=true anchor=(...)>...</Popup>
                    <Popup placement=Top skidding=40 active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="arrows" anchor=true>
            "Arrows"
        </Head>
        <p>"Add an arrow to your popup with the "<code>arrow</code>" attribute. Customize its size and color with the "
            <code>"--arrow-size"</code>" and "<code>"--arrow-color"</code>" custom properties, and align it with "
            <code>arrow_placement</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup
                    placement=Top
                    distance=8
                    arrow=true
                    active=true
                    style="--arrow-color: var(--wa-color-brand-fill-loud)"
                    anchor=(anchor())
                >
                    <div style=BOX_STYLE></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup
                        placement=Top
                        distance=8
                        arrow=true
                        active=true
                        style="--arrow-color: var(--wa-color-brand-fill-loud)"
                        anchor=(...)
                    >
                        ...
                    </Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="sync" anchor=true>
            "Syncing with the Anchor's Dimensions"
        </Head>
        <p>"Use the "<code>sync</code>" attribute to make the popup the same width or height as the anchor element. "
            "This is useful for controls that need the popup to stay the same size as the trigger, such as dropdowns."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Bottom distance=8 sync=(SyncSize::Width) active=true anchor=(anchor())>
                    <div style="height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Bottom distance=8 sync=(SyncSize::Width) active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="flip" anchor=true>
            "Flip"
        </Head>
        <p>"When the popup doesn't have enough room in its preferred placement, the "<code>flip</code>
            " attribute moves it to the opposite side instead of letting it overflow. "
            "Scroll the page so this anchor approaches the top of the viewport — the popup flips below."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Top distance=8 flip=true active=true anchor=(anchor())>
                    <div style=BOX_STYLE></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Top distance=8 flip=true active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="shift" anchor=true>
            "Shift"
        </Head>
        <p>"When the popup overflows the viewport along its anchor, the "<code>shift</code>
            " attribute moves it back into view. The popup below is wider than its anchor and hugs the viewport "
            "edge instead of overflowing it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Bottom distance=8 shift=true shift_padding=8 active=true anchor=(anchor())>
                    <div style="width: 300px; height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Bottom distance=8 shift=true shift_padding=8 active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="auto-size" anchor=true>
            "Auto-Size"
        </Head>
        <p>"Use the "<code>auto_size</code>" attribute to tell the popup to resize when necessary to prevent it from "
            "overflowing. The available space is exposed to the popup content through the read-only "
            <code>"--auto-size-available-width"</code>" and "<code>"--auto-size-available-height"</code>
            " custom properties."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup
                    placement=Bottom
                    distance=8
                    flip=true
                    auto_size=(AutoSize::Vertical)
                    auto_size_padding=10
                    active=true
                    anchor=(anchor())
                >
                    <div style="width: 100px; height: 400px; max-height: var(--auto-size-available-height); overflow: auto; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup
                        placement=Bottom
                        distance=8
                        flip=true
                        auto_size=(AutoSize::Vertical)
                        auto_size_padding=10
                        active=true
                        anchor=(...)
                    >
                        <div style="max-height: var(--auto-size-available-height); overflow: auto; ...">...</div>
                    </Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring: any change to the controls inside `.popup-overview` is
/// applied to the demo popup.
pub fn listen_popup_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener("change", |event| {
        handle_overview_event(&event);
    });
    document.add_steady_event_listener("input", |event| {
        handle_overview_event(&event);
    });
}

/// Applies the initial control state to the demo popup. Run it after every render.
pub fn init_popup_overview() {
    if let Some(overview) = dom::existing::document()
        .query_selector(".popup-overview")
        .ok()
        .flatten()
    {
        sync_overview(&overview);
    }
}

fn handle_overview_event(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let overview = target.closest(".popup-overview").ok()??;
    sync_overview(&overview)
}

fn control_value(overview: &Element, selector: &str) -> Option<String> {
    let input: HtmlInputElement = overview.query_selector(selector).ok()??.dyn_into().ok()?;
    Some(input.value())
}

fn control_checked(overview: &Element, selector: &str) -> bool {
    overview
        .query_selector(selector)
        .ok()
        .flatten()
        .and_then(|control| control.dyn_into::<HtmlInputElement>().ok())
        .is_some_and(|input| input.checked())
}

/// Reads the control values and applies them to the popup's `data-*`
/// configuration, the arrow visibility, and the active state.
fn sync_overview(overview: &Element) -> Option<()> {
    // The demo popup is the direct `.popup` child: the `Select` control
    // contains its own nested popup host
    let popup = overview.query_selector(":scope > .popup").ok()??;

    let placement = control_value(overview, ".value-input[name='placement']")
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "top".to_string());
    let distance = control_value(overview, ".control[name='distance']").unwrap_or_default();
    let skidding = control_value(overview, ".control[name='skidding']").unwrap_or_default();
    let active = control_checked(overview, ".control[name='active']");
    let arrow = control_checked(overview, ".control[name='arrow']");

    popup.set_attribute("data-placement", &placement).ok();
    popup
        .set_attribute("data-distance", if distance.is_empty() { "0" } else { &distance })
        .ok();
    popup
        .set_attribute("data-skidding", if skidding.is_empty() { "0" } else { &skidding })
        .ok();

    if let Some(arrow_element) = popup.query_selector(".popup-body > .arrow").ok().flatten() {
        if arrow {
            arrow_element.remove_attribute("hidden").ok();
        } else {
            arrow_element.set_attribute("hidden", "").ok();
        }
    }

    set_popup_active(&popup, active);

    Some(())
}
