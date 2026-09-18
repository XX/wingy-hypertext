use hypertext::prelude::{GlobalAttributes, HtmxAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::radio::Radio;
use wingy_hypertext::component::radio_group::RadioGroup;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Radio"</Head>
        <p>"A radio is a single choice of a "<code>RadioGroup</code>" and is meant to be used inside one: the "
            "group gives it the label, the hint and the layout, and the radios of one group are tied together "
            "by their "<code>name</code>"."
        </p>
        <p>"Under the hood it is a native "<code>"<input type=\"radio\">"</code>" wrapped in a label, so "
            "selecting an option, moving between the options with the arrow keys and submitting the value are "
            "all native, with no client-side code. Set "<code>checked</code>" on the radio that should start "
            "selected, "<code>disabled</code>" to disable it, and "<code>"appearance=Button"</code>" to render "
            "the group as a segmented control."
        </p>
        <p>"See the "
            <a
                href="/radio-group"
                hx-get="/radio-group"
                hx-target=".main-content"
                hx-swap="innerHTML"
                hx-push-url="true"
            >"Radio Group"</a>
            " page for the examples of this component in action."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <RadioGroup label="Network">
                    <Radio name="network" value="off">"Off"</Radio>
                    <Radio name="network" value="wifi" checked=true>"Wi-Fi"</Radio>
                    <Radio name="network" value="all">"Everything"</Radio>
                </RadioGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <RadioGroup label="Network">
                        <Radio name="network" value="off">"Off"</Radio>
                        <Radio name="network" value="wifi" checked=true>"Wi-Fi"</Radio>
                        <Radio name="network" value="all">"Everything"</Radio>
                    </RadioGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
