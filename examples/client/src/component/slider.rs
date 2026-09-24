use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::Event;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{
    CLUSTER, GAP_L, GAP_XL, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK,
};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::slider::{Slider, SliderTooltip};
use wingy_hypertext::helper::popup::PopupPlacement::Right;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext::orientation::Orientation::*;
use wingy_hypertext_web::component::slider::Thumb;
use wingy_hypertext_web::util::event;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Slider"</Head>
        <p>"Sliders let the user choose a number — or a range of numbers — by dragging a thumb along a track. "
            "The positions of the thumb, of the filled part and of the markers are rendered on the server, so "
            "the slider looks right before any script runs; dragging, the keyboard and the tooltip are "
            "implemented in Rust in "<code>"wingy-hypertext-web"</code>"."
        </p>
        <p>"The control isn't a native one, so the value travels with the form in a hidden input — a range "
            "slider submits "<code>"{name}-min"</code>" and "<code>"{name}-max"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider name="volume" value=50.0/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider name="volume" value=50.0/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="label" anchor=true>
            "Label"
        </Head>
        <p>"Use the "<code>label</code>" property to give the slider an accessible label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider label="Volume" name="labelled-volume"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider label="Volume" name="volume"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="hint" anchor=true>
            "Hint"
        </Head>
        <p>"Add a descriptive hint with the "<code>hint</code>" property."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider label="Volume" hint="Controls the volume of the current song." value=50.0/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider label="Volume" hint="Controls the volume of the current song." value=50.0/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="min-max-step" anchor=true>
            "Min, Max & Step"
        </Head>
        <p>"Use "<code>min</code>" and "<code>max</code>" to define the range, and "<code>step</code>
            " to control the increment between values."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider
                    id="slider-fraction"
                    label="Between zero and one"
                    min=0.0
                    max=1.0
                    step=0.1
                    value=0.5
                    tooltip=(Default::default())
                />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider
                        id="slider-fraction"
                        label="Between zero and one"
                        min=0.0
                        max=1.0
                        step=0.1
                        value=0.5
                        tooltip=(Default::default())
                    />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="tooltip" anchor=true>
            "Showing a Tooltip"
        </Head>
        <p>"Set "<code>tooltip</code>" to show the current value while the slider is focused or dragged. The "
            "tooltip anchors to the thumb, so the slider needs an "<code>id</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider id="slider-quality" label="Quality" name="quality" value=50.0 tooltip=(Default::default())/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider id="slider-quality" label="Quality" name="quality" value=50.0 tooltip=(Default::default())/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="markers" anchor=true>
            "Showing Markers"
        </Head>
        <p>"Set "<code>markers</code>" to draw an indicator at every step. It works best with a smaller range "
            "of values."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider label="Size" name="size" min=0.0 max=8.0 value=4.0 markers=true/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider label="Size" name="size" min=0.0 max=8.0 value=4.0 markers=true/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="references" anchor=true>
            "Adding References"
        </Head>
        <p>"The children are the references shown under the track. They are spread from its start to its end, "
            "so they line up with the start, the center and the end positions."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider
                    label="Speed"
                    name="speed"
                    min=1.0
                    max=5.0
                    value=3.0
                    markers=true
                    hint="Controls the speed of the thing you're currently doing."
                >
                    <span>"Slow"</span>
                    <span>"Medium"</span>
                    <span>"Fast"</span>
                </Slider>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider label="Speed" name="speed" min=1.0 max=5.0 value=3.0 markers=true>
                        <span>"Slow"</span>
                        <span>"Medium"</span>
                        <span>"Fast"</span>
                    </Slider>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="range" anchor=true>
            "Range Selection"
        </Head>
        <p>"Set "<code>range</code>" for a selection with two thumbs, and give them their starting positions "
            "with "<code>min_value</code>" and "<code>max_value</code>". The thumbs never cross each other."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider
                    id="slider-price"
                    label="Price range"
                    name="price"
                    range=(25.0..=75.0)
                    min=0.0
                    max=100.0
                    tooltip=(Default::default())
                />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider
                        id="slider-price"
                        label="Price range"
                        name="price"
                        range=(25.0..=75.0)
                        tooltip=(Default::default())
                    />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="vertical" anchor=true>
            "Vertical Sliders"
        </Head>
        <p>"Set "<code>"orientation=Vertical"</code>" for a vertical slider, a range one included."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_XL)>
                    <Slider orientation=Vertical label="Volume" name="v-volume" value=65.0/>
                    <Slider orientation=Vertical label="Bass" name="v-bass" value=50.0/>
                    <Slider orientation=Vertical label="Treble" name="v-treble" value=40.0/>
                    <Slider
                        id="slider-temperature"
                        orientation=Vertical
                        label="Temperature"
                        name="temperature"
                        range=(30.0..=70.0)
                        tooltip=(SliderTooltip::new().with_placement(Right))
                    />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider orientation=Vertical label="Volume" name="volume" value=65.0/>

                    <Slider
                        id="slider-temperature"
                        orientation=Vertical
                        label="Temperature"
                        range=(30.0..=70.0)
                        tooltip=(SliderTooltip::new().with_placement(Right))
                    />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The slider follows its "<code>"size-*"</code>" class, like the other form controls."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Slider class=SIZE_EXTRA_SMALL label="Extra small" value=50.0/>
                    <Slider class=SIZE_SMALL label="Small" value=50.0/>
                    <Slider class=SIZE_MEDIUM label="Medium" value=50.0/>
                    <Slider class=SIZE_LARGE label="Large" value=50.0/>
                    <Slider class=SIZE_EXTRA_LARGE label="Extra large" value=50.0/>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider class=SIZE_SMALL label="Small" value=50.0/>
                    <Slider class=SIZE_LARGE label="Large" value=50.0/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="indicator-offset" anchor=true>
            "Indicator Offset"
        </Head>
        <p>"By default the filled part extends from the minimum to the current value. Set "
            <code>indicator_offset</code>" to start it from another value instead."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider
                    id="slider-friendliness"
                    label="User friendliness"
                    hint="Did you find our product easy to use?"
                    name="friendliness"
                    value=0.0
                    min=-5.0
                    max=5.0
                    indicator_offset=0.0
                    markers=true
                    tooltip=(Default::default())
                >
                    <span>"Difficult"</span>
                    <span>"Moderate"</span>
                    <span>"Easy"</span>
                </Slider>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider
                        id="slider-friendliness"
                        label="User friendliness"
                        value=0.0
                        min=-5.0
                        max=5.0
                        indicator_offset=0.0
                        markers=true
                        tooltip=(Default::default())
                    >
                        <span>"Difficult"</span>
                        <span>"Moderate"</span>
                        <span>"Easy"</span>
                    </Slider>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>


        <Head level=H3 id="reacting-to-input" anchor=true>
            "Reacting to Input"
        </Head>
        <p>"The slider reports every change with an "<code>input</code>" event and the settled value with "
            <code>change</code>", both bubbling from the slider element — the same names a native form control "
            "uses. Here moving the slider resizes the preview text."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="slider-text-size-demo">
                    <p class="slider-text-size-preview">"The quick brown fox jumps over the lazy dog."</p>
                    <Divider/>
                    <Slider
                        id="slider-text-size"
                        label="Text size"
                        min=12.0
                        max=48.0
                        value=18.0
                        tooltip=(Default::default())
                    />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="slider-text-size-demo">
                        <p class="slider-text-size-preview">"The quick brown fox…"</p>
                        <Slider id="slider-text-size" label="Text size" min=12.0 max=48.0 value=18.0 tooltip=(Default::default())/>
                    </div>

                    // The demo listens for the event and resizes the preview
                    document.add_steady_event_listener(event::INPUT, |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="filtering" anchor=true>
            "Filtering with a Range"
        </Head>
        <p>"Two thumbs make a natural filter. Dragging them hides the items whose price falls outside the "
            "selected range; the values are formatted by the demo, as the slider itself renders plain numbers."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="slider-price-filter-demo">
                    <ul class="slider-price-filter-list">
                        <li data-price="15">"Sticker pack — $15"</li>
                        <li data-price="30">"T-shirt — $30"</li>
                        <li data-price="55">"Hoodie — $55"</li>
                        <li data-price="80">"Backpack — $80"</li>
                        <li data-price="120">"Jacket — $120"</li>
                    </ul>
                    <Divider/>
                    <Slider
                        id="slider-price-filter"
                        label="Price range"
                        max=150.0
                        range=(0.0..=150.0)
                        tooltip=(Default::default())
                    />
                    <p class="slider-demo-output">"Showing everything"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ul class="slider-price-filter-list">
                        <li data-price="15">"Sticker pack — $15"</li>
                        ...
                    </ul>
                    <Slider
                        id="slider-price-filter"
                        label="Price range"
                        max=150.0
                        range=(0.0..=150.0)
                        tooltip=(Default::default())
                    />

                    // The demo hides the items outside of the range on every `input`
                    document.add_steady_event_listener(event::INPUT, |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="formatting" anchor=true>
            "Formatting the Value"
        </Head>
        <p>"The slider renders plain numbers, so a formatted value — a currency, a unit — is the job of the "
            "code that reads it. The filter above shows the pattern: the demo keeps the tooltip's numbers and "
            "prints the formatted range next to the list. A "<code>value_formatter</code>" like the one "
            <code>"wa-slider"</code>" takes is a JavaScript function, and has no server-side equivalent."
        </p>
        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Set "<code>disabled</code>" to disable the slider: it stops responding and leaves the tab order."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider label="Disabled" value=50.0 disabled=true/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider label="Disabled" value=50.0 disabled=true/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="readonly" anchor=true>
            "Readonly"
        </Head>
        <p>"Set "<code>readonly</code>" to show a value the user can't change. Unlike "<code>disabled</code>
            ", a readonly slider stays focusable and its value is still submitted with the form."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Slider
                    id="slider-load"
                    label="Server load"
                    name="load"
                    value=72.0
                    readonly=true
                    tooltip=(Default::default())
                />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Slider id="slider-load" label="Server load" name="load" value=72.0 readonly=true tooltip=(Default::default())/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

/// One-time wiring for the two demos of the page: the preview that follows the
/// value of a slider, and the list filtered by a range.
pub fn listen_slider_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener(event::INPUT, |event| {
        resize_preview(&event);
        filter_by_price(&event);
    });
}

fn resize_preview(event: &Event) -> Option<()> {
    let slider = event.target()?.maybe_into_element()?;
    let demo = slider.closest(".slider-text-size-demo").ok()??;
    let value = slider
        .query_selector("[role=slider]")
        .ok()??
        .get_attribute("aria-valuenow")?;

    let preview = demo
        .query_selector(".slider-text-size-preview")
        .ok()??
        .maybe_into_html()?;
    preview.style().set_property("font-size", &format!("{value}px")).ok();

    Some(())
}

fn filter_by_price(event: &Event) -> Option<()> {
    let slider = event.target()?.maybe_into_element()?;
    let demo = slider.closest(".slider-price-filter-demo").ok()??;

    let value_of = |selector: &str| -> Option<f64> {
        slider
            .query_selector(selector)
            .ok()??
            .get_attribute("aria-valuenow")?
            .parse()
            .ok()
    };
    let min = value_of(Thumb::Min.selector())?;
    let max = value_of(Thumb::Max.selector())?;

    let mut shown = 0;
    for item in dom::existing::select_all_elements_from(&demo, ".slider-price-filter-list li") {
        let price: f64 = item
            .get_attribute("data-price")
            .and_then(|price| price.parse().ok())
            .unwrap_or_default();
        let hidden = price < min || price > max;
        if hidden {
            item.set_attribute("hidden", "").ok();
        } else {
            item.remove_attribute("hidden").ok();
            shown += 1;
        }
    }

    let output = demo.query_selector(".slider-demo-output").ok()??;
    output.set_text_content(Some(&format!("${min} – ${max}: {shown} of 5 items")));

    Some(())
}
