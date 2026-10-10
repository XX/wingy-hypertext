use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use js_sys::Reflect;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::{Event, HtmlInputElement};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{
    CLUSTER, GAP_S, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK,
};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::rating::RatingSymbolState::*;
use wingy_hypertext::component::rating::{Rating, RatingSymbol};
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::variant::Variant::*;
use wingy_hypertext_web::util::event;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Rating"</Head>
        <p>"Ratings display a numeric score as a row of selectable symbols, typically stars. Use them to capture "
            "quick feedback or show an average rating for a product or piece of content."
        </p>
        <p>"The value is rendered on the server, partially filled symbols included; hovering, clicking and the "
            "keyboard are implemented in Rust in "<code>"wingy-hypertext-web"</code>". The value travels with "
            "the form in a hidden input."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating"/>
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
        <p>"Ratings are usually identified by context, so the label isn't displayed. Always provide one with "
            "the "<code>label</code>" property so assistive devices can announce the control."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rate this component"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rate this component"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Use the "<code>disabled</code>" property to disable the rating."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" disabled=true value=3.0/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" disabled=true value=3.0/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="readonly" anchor=true>
            "Readonly"
        </Head>
        <p>"Use the "<code>readonly</code>" property to display a rating that users can't change. Unlike "
            <code>disabled</code>", a readonly rating still submits its value with the form."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" readonly=true value=3.0/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" readonly=true value=3.0/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The rating follows its "<code>"size-*"</code>" class, like the other form controls."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Rating label="Extra small" class=SIZE_EXTRA_SMALL/>
                    <Rating label="Small" class=SIZE_SMALL/>
                    <Rating label="Medium" class=SIZE_MEDIUM/>
                    <Rating label="Large" class=SIZE_LARGE/>
                    <Rating label="Extra large" class=SIZE_EXTRA_LARGE/>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Extra small" class=SIZE_EXTRA_SMALL/>
                    <Rating label="Small" class=SIZE_SMALL/>
                    <Rating label="Medium" class=SIZE_MEDIUM/>
                    <Rating label="Large" class=SIZE_LARGE/>
                    <Rating label="Extra large" class=SIZE_EXTRA_LARGE/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"For finer control, set the "<code>"font-size"</code>" property directly."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" style="font-size: 3rem"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" style="font-size: 3rem"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="max-value" anchor=true>
            "Max Value"
        </Head>
        <p>"Ratings go from 0 to 5 by default. Use the "<code>max</code>" property to change the highest "
            "possible value."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" max=3/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" max=3/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="precision" anchor=true>
            "Precision"
        </Head>
        <p>"Use the "<code>precision</code>" property to let users select fractional ratings, such as half "
            "stars."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" precision=0.5 value=2.5/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" precision=0.5 value=2.5/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="custom-icons" anchor=true>
            "Custom Icons"
        </Head>
        <p>"Pass a "<code>RatingSymbol</code>" as the children to render a custom symbol in place of the "
            "default star. Without a "<code>state</code>" the same symbol is used for the empty and the filled "
            "state, which differ only in color."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" style="--symbol-color-active: #ff4136">
                    <RatingSymbol>(fontawesome::solid::Heart)</RatingSymbol>
                </Rating>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" style="--symbol-color-active: #ff4136">
                        <RatingSymbol>(fontawesome::solid::Heart)</RatingSymbol>
                    </Rating>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"Set "<code>state</code>" to draw a different symbol for the empty and the filled state."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating" value=3.0 style="--symbol-color-active: #ff4136">
                    <RatingSymbol state=Empty>(fontawesome::regular::Heart)</RatingSymbol>
                    <RatingSymbol state=Full>(fontawesome::solid::Heart)</RatingSymbol>
                </Rating>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating" value=3.0 style="--symbol-color-active: #ff4136">
                        <RatingSymbol state=Empty>(fontawesome::regular::Heart)</RatingSymbol>
                        <RatingSymbol state=Full>(fontawesome::solid::Heart)</RatingSymbol>
                    </Rating>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="value-based-icons" anchor=true>
            "Value-Based Icons"
        </Head>
        <p>"Give a "<code>RatingSymbol</code>" a "<code>value</code>" to draw it at that position only, so "
            "you can render different icons across the scale."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Rating label="Rating">
                    <RatingSymbol value=1>(fontawesome::solid::FaceAngry)</RatingSymbol>
                    <RatingSymbol value=2>(fontawesome::solid::FaceFrown)</RatingSymbol>
                    <RatingSymbol value=3>(fontawesome::solid::FaceMeh)</RatingSymbol>
                    <RatingSymbol value=4>(fontawesome::solid::FaceSmile)</RatingSymbol>
                    <RatingSymbol value=5>(fontawesome::solid::FaceLaugh)</RatingSymbol>
                </Rating>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Rating label="Rating">
                        <RatingSymbol value=1>(fontawesome::solid::FaceAngry)</RatingSymbol>
                        <RatingSymbol value=2>(fontawesome::solid::FaceFrown)</RatingSymbol>
                        <RatingSymbol value=3>(fontawesome::solid::FaceMeh)</RatingSymbol>
                        <RatingSymbol value=4>(fontawesome::solid::FaceSmile)</RatingSymbol>
                        <RatingSymbol value=5>(fontawesome::solid::FaceLaugh)</RatingSymbol>
                    </Rating>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="detecting-hover" anchor=true>
            "Detecting Hover"
        </Head>
        <p>"Use the "<code>"wg-hover"</code>" event to react as the user hovers over (or touches and drags "
            "across) the rating, before they commit to a value. The event's "<code>detail</code>" carries "
            <code>phase</code>" — "<code>start</code>", "<code>move</code>" or "<code>end</code>" — and the "
            <code>value</code>" the rating would take."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_S, " rating-hover-demo")>
                    <Rating label="Rating"/>
                    <span class="rating-hover-output"></span>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="rating-hover-demo">
                        <Rating label="Rating"/>
                        <span class="rating-hover-output"></span>
                    </div>

                    // The demo listens for the event and names the hovered value
                    document.add_steady_event_listener(event::HOVER, |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="required" anchor=true>
            "Required"
        </Head>
        <p>"Use the "<code>required</code>" property to make the rating mandatory. The form won't submit until "
            "the user selects a value — the validation is the browser's own."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <form class="rating-form-demo">
                    <Rating label="Rating" name="required-rating" required=true/>
                    <div class=(CLUSTER, " ", GAP_S) style="margin-block-start: 1em">
                        <Button appearance=Filled variant=Neutral attrs=(attrs!["type" = &"submit"])>
                            "Submit"
                        </Button>
                    </div>
                    <p class="rating-demo-output">"Nothing submitted yet"</p>
                </form>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <form>
                        <Rating label="Rating" name="rating" required=true/>
                        <Button appearance=Filled variant=Neutral attrs=(attrs!["type" = &"submit"])>"Submit"</Button>
                    </form>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="form-submission" anchor=true>
            "Form Submission"
        </Head>
        <p>"Ratings work in forms just like native form controls: the "<code>name</code>" and the value are "
            "included in the form data on submit, and resetting the form brings back the initial value."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <form class="rating-form-demo">
                    <Rating name="experience" label="How would you rate your experience?" value=4.0 required=true/>
                    <div class=(CLUSTER, " ", GAP_S) style="margin-block-start: 1em">
                        <Button attrs=(attrs!["type" = &"submit"])>"Submit"</Button>
                        <Button appearance=Filled variant=Neutral attrs=(attrs!["type" = &"reset"])>"Reset"</Button>
                    </div>
                    <p class="rating-demo-output">"Nothing submitted yet"</p>
                </form>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <form>
                        <Rating name="experience" label="How would you rate your experience?" value=4.0 required=true/>
                        <Button attrs=(attrs!["type" = &"submit"])>"Submit"</Button>
                        <Button appearance=Filled variant=Neutral attrs=(attrs!["type" = &"reset"])>"Reset"</Button>
                    </form>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

/// One-time wiring for the demos of the page: the hovered value named next to
/// the rating, and the forms printing what they submit instead of leaving the
/// page.
pub fn listen_rating_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener(event::HOVER, |event| {
        name_hovered_value(&event);
    });
    document.add_steady_event_listener("submit", |event| {
        report_submit(&event);
    });
}

fn name_hovered_value(event: &Event) -> Option<()> {
    const TERMS: [&str; 6] = ["No rating", "Terrible", "Bad", "OK", "Good", "Excellent"];

    let rating = event.target()?.maybe_into_element()?;
    let demo = rating.closest(".rating-hover-demo").ok()??;
    let output = demo.query_selector(".rating-hover-output").ok()??;

    let detail = Reflect::get(event, &"detail".into()).ok()?;
    let phase = Reflect::get(&detail, &"phase".into()).ok()?.as_string()?;
    let value = Reflect::get(&detail, &"value".into()).ok()?.as_f64()?;

    let text = if phase == "end" {
        ""
    } else {
        TERMS.get(value as usize).copied().unwrap_or_default()
    };
    output.set_text_content(Some(text));

    Some(())
}

fn report_submit(event: &Event) -> Option<()> {
    let form = event.target()?.maybe_into_element()?;
    if !form.class_list().contains("rating-form-demo") {
        return None;
    }
    event.prevent_default();

    let submitted = dom::existing::select_all_elements_from(&form, "input[name]")
        .filter_map(|input| input.dyn_into::<HtmlInputElement>().ok())
        .filter(|input| !input.disabled())
        .map(|input| format!("{}={}", input.name(), input.value()))
        .collect::<Vec<_>>()
        .join(", ");

    let output = form.query_selector(".rating-demo-output").ok()??;
    output.set_text_content(Some(&format!("Submitted: {submitted}")));

    Some(())
}
