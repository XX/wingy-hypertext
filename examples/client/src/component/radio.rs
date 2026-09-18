use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::Event;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{GAP_L, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::radio::Radio;
use wingy_hypertext::component::radio::RadioAppearance::*;
use wingy_hypertext::component::radio_group::RadioGroup;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::orientation::Orientation::*;
use wingy_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Radio Group"</Head>
        <p>"Radio groups are used to group multiple radios so they function as a single control. The group "
            "renders a native "<code>"<fieldset>"</code>", and every radio is a native "
            <code>"<input type=\"radio\">"</code>" in a label — selecting an option, moving between the "
            "options with the arrow keys and submitting the value are all native, with no client-side code."
        </p>
        <p>"Radios of one group are tied together by their "<code>name</code>", and the selected one carries "
            <code>checked</code>": the group doesn't set either, since it only sees its children as markup."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <RadioGroup label="Coffee roast">
                    <Radio name="roast" value="light">"Light roast"</Radio>
                    <Radio name="roast" value="medium" checked=true>"Medium roast"</Radio>
                    <Radio name="roast" value="dark">"Dark roast"</Radio>
                </RadioGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Coffee roast">
                        <Radio name="roast" value="light">"Light roast"</Radio>
                        <Radio name="roast" value="medium" checked=true>"Medium roast"</Radio>
                        <Radio name="roast" value="dark">"Dark roast"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="initial-value" anchor=true>
            "Initial Value"
        </Head>
        <p>"Set "<code>checked</code>" on the radio that should start selected, just like in native HTML."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <RadioGroup label="Coffee roast">
                    <Radio name="initial-roast" value="light">"Light roast"</Radio>
                    <Radio name="initial-roast" value="medium">"Medium roast"</Radio>
                    <Radio name="initial-roast" value="dark" checked=true>"Dark roast"</Radio>
                </RadioGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Coffee roast">
                        <Radio name="roast" value="light">"Light roast"</Radio>
                        <Radio name="roast" value="medium">"Medium roast"</Radio>
                        <Radio name="roast" value="dark" checked=true>"Dark roast"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="hint" anchor=true>
            "Hint"
        </Head>
        <p>"Add a descriptive hint to a group with the "<code>hint</code>" property."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <RadioGroup label="Coffee roast" hint="Pick the roast we'll grind for your order.">
                    <Radio name="hint-roast" value="light">"Light roast"</Radio>
                    <Radio name="hint-roast" value="medium" checked=true>"Medium roast"</Radio>
                    <Radio name="hint-roast" value="dark">"Dark roast"</Radio>
                </RadioGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Coffee roast" hint="Pick the roast we'll grind for your order.">
                        <Radio name="roast" value="light">"Light roast"</Radio>
                        <Radio name="roast" value="medium" checked=true>"Medium roast"</Radio>
                        <Radio name="roast" value="dark">"Dark roast"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="radio-buttons" anchor=true>
            "Radio Buttons"
        </Head>
        <p>"Set "<code>"appearance=Button"</code>" on every radio of a group to render it as a segmented "
            "control. The group collapses the borders between the neighbours and rounds only its outer edges."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <RadioGroup
                        label="Color scheme"
                        hint="Choose how the interface should appear."
                        orientation=Horizontal
                    >
                        <Radio appearance=Button name="scheme" value="light">"Light"</Radio>
                        <Radio appearance=Button name="scheme" value="dark">"Dark"</Radio>
                        <Radio appearance=Button name="scheme" value="auto" checked=true>"Auto"</Radio>
                    </RadioGroup>
                    <RadioGroup
                        label="Color scheme"
                        style="max-width: 300px"
                        hint="Choose how the interface should appear."
                    >
                        <Radio appearance=Button name="scheme-vertical" value="light">"Light"</Radio>
                        <Radio appearance=Button name="scheme-vertical" value="dark">"Dark"</Radio>
                        <Radio appearance=Button name="scheme-vertical" value="auto" checked=true>"Auto"</Radio>
                    </RadioGroup>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup
                        label="Color scheme"
                        hint="Choose how the interface should appear."
                        orientation=Horizontal
                    >
                        <Radio appearance=Button name="scheme" value="light">"Light"</Radio>
                        <Radio appearance=Button name="scheme" value="dark">"Dark"</Radio>
                        <Radio appearance=Button name="scheme" value="auto" checked=true>"Auto"</Radio>
                    </RadioGroup>
                    
                    <RadioGroup
                        label="Color scheme"
                        style="max-width: 300px"
                        hint="Choose how the interface should appear."
                    >
                        <Radio appearance=Button name="scheme-vertical" value="light">"Light"</Radio>
                        <Radio appearance=Button name="scheme-vertical" value="dark">"Dark"</Radio>
                        <Radio appearance=Button name="scheme-vertical" value="auto" checked=true>"Auto"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Set "<code>disabled</code>" on a radio to disable that option, or on the group to disable all of "
            "them: the group is a native "<code>"<fieldset>"</code>", so it does that on its own."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <RadioGroup label="Shipping speed">
                        <Radio name="shipping" value="standard" checked=true>"Standard"</Radio>
                        <Radio name="shipping" value="express">"Express"</Radio>
                        <Radio name="shipping" value="overnight" disabled=true>"Overnight"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Shipping speed (whole group)" disabled=true>
                        <Radio name="shipping-disabled" value="standard" checked=true>"Standard"</Radio>
                        <Radio name="shipping-disabled" value="express">"Express"</Radio>
                        <Radio name="shipping-disabled" value="overnight">"Overnight"</Radio>
                    </RadioGroup>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Shipping speed">
                        <Radio name="shipping" value="standard" checked=true>"Standard"</Radio>
                        <Radio name="shipping" value="express">"Express"</Radio>
                        <Radio name="shipping" value="overnight" disabled=true>"Overnight"</Radio>
                    </RadioGroup>

                    <RadioGroup label="Shipping speed" disabled=true>
                        ...
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="orientation" anchor=true>
            "Orientation"
        </Head>
        <p>"The default orientation is "<code>Vertical</code>". Set "<code>"orientation=Horizontal"</code>
            " to lay the options out on the same row."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <RadioGroup
                    label="Shipping speed"
                    hint="Choose how fast you'd like your order."
                    orientation=Horizontal
                >
                    <Radio name="shipping-horizontal" value="standard" checked=true>"Standard"</Radio>
                    <Radio name="shipping-horizontal" value="express">"Express"</Radio>
                    <Radio name="shipping-horizontal" value="overnight">"Overnight"</Radio>
                </RadioGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup
                        label="Shipping speed"
                        hint="Choose how fast you'd like your order."
                        orientation=Horizontal
                    >
                        <Radio name="shipping" value="standard" checked=true>"Standard"</Radio>
                        <Radio name="shipping" value="express">"Express"</Radio>
                        <Radio name="shipping" value="overnight">"Overnight"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The size of the radios follows the group's "<code>"size-*"</code>" class. Put the class on a "
            "single radio instead to size it on its own."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <RadioGroup label="Extra small" class=SIZE_EXTRA_SMALL>
                        <Radio name="roast-xs" value="light">"Light roast"</Radio>
                        <Radio name="roast-xs" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Small" class=SIZE_SMALL>
                        <Radio name="roast-s" value="light">"Light roast"</Radio>
                        <Radio name="roast-s" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Medium" class=SIZE_MEDIUM>
                        <Radio name="roast-m" value="light">"Light roast"</Radio>
                        <Radio name="roast-m" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Large" class=SIZE_LARGE>
                        <Radio name="roast-l" value="light">"Light roast"</Radio>
                        <Radio name="roast-l" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Extra large" class=SIZE_EXTRA_LARGE>
                        <Radio name="roast-xl" value="light">"Light roast"</Radio>
                        <Radio name="roast-xl" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>
                    <RadioGroup label="Mixed sizes">
                        <Radio name="mixed" value="s" class=SIZE_SMALL>"Small"</Radio>
                        <Radio name="mixed" value="m" class=SIZE_MEDIUM checked=true>"Medium"</Radio>
                        <Radio name="mixed" value="l" class=SIZE_LARGE>"Large"</Radio>
                    </RadioGroup>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Small" class=SIZE_SMALL>
                        <Radio name="roast" value="light">"Light roast"</Radio>
                        <Radio name="roast" value="medium" checked=true>"Medium roast"</Radio>
                    </RadioGroup>

                    <RadioGroup label="Mixed sizes">
                        <Radio name="mixed" value="s" class=SIZE_SMALL>"Small"</Radio>
                        <Radio name="mixed" value="m" class=SIZE_MEDIUM checked=true>"Medium"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="validation" anchor=true>
            "Validation"
        </Head>
        <p>"Set "<code>required</code>" on the radios to make selecting an option mandatory, and on the group "
            "to mark its label. The validation is the browser's own: an empty group blocks the submit and "
            "shows the built-in message."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <form class="radio-validation-demo">
                    <RadioGroup label="Coffee roast" required=true>
                        <Radio name="required-roast" value="light" required=true>"Light roast"</Radio>
                        <Radio name="required-roast" value="medium" required=true>"Medium roast"</Radio>
                        <Radio name="required-roast" value="dark" required=true>"Dark roast"</Radio>
                    </RadioGroup>
                    <Button
                        appearance=Filled
                        variant=Neutral
                        attrs=(attrs!["type" = &"submit"])
                        style="margin-block-start: 1em"
                    >
                        "Submit"
                    </Button>
                    <p class="radio-demo-output">"Nothing submitted yet"</p>
                </form>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <form>
                        <RadioGroup label="Coffee roast" required=true>
                            <Radio name="roast" value="light" required=true>"Light roast"</Radio>
                            <Radio name="roast" value="medium" required=true>"Medium roast"</Radio>
                            <Radio name="roast" value="dark" required=true>"Dark roast"</Radio>
                        </RadioGroup>
                        <Button appearance=Filled variant=Neutral attrs=(attrs!["type" = &"submit"])>"Submit"</Button>
                    </form>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

/// One-time wiring for the validation demo: the browser blocks an empty group
/// on its own, so the listener only reports the value of a valid submit instead
/// of letting the form navigate away.
pub fn listen_radio_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener("submit", |event| {
        report_submit(&event);
    });
}

fn report_submit(event: &Event) -> Option<()> {
    let form = event.target()?.maybe_into_element()?;
    let demo = form.closest(".radio-validation-demo").ok()??;
    event.prevent_default();

    let output = demo.query_selector(".radio-demo-output").ok()??;
    let checked = demo
        .query_selector(".radio > .control:checked")
        .ok()?
        .and_then(|control| control.get_attribute("value"))
        .unwrap_or_default();
    output.set_text_content(Some(&format!("Submitted: {checked}")));

    Some(())
}
