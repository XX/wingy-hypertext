use hypertext::prelude::{GlobalAttributes, HtmxAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{GAP_L, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::component::checkbox::Checkbox;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Checkbox"</Head>
        <p>"Checkboxes allow the user to toggle an option on or off. Under the hood it is a native "
            <code>"<input type=\"checkbox\">"</code>" wrapped in a label, so toggling, keyboard support and "
            "form submission are native."
        </p>
        <p>"Several checkboxes can be presented as one control with a shared label and hint — see the "
            <a
                href="/checkbox-group"
                hx-get="/checkbox-group"
                hx-target=".main-content"
                hx-swap="innerHTML"
                hx-push-url="true"
            >"Checkbox Group"</a>
            " page."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Checkbox name="terms" value="accepted">"I agree to the terms and conditions"</Checkbox>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox name="terms" value="accepted">"I agree to the terms and conditions"</Checkbox>
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
        <p>"Set "<code>checked</code>" to activate the checkbox. Like in native HTML it is the initial value, "
            "not the current state."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Checkbox name="remember" checked=true>"Remember me"</Checkbox>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox name="remember" checked=true>"Remember me"</Checkbox>
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
                <Checkbox name="newsletter" hint="You can turn this off later in settings.">
                    "Subscribe to the newsletter"
                </Checkbox>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox name="newsletter" hint="You can turn this off later in settings.">
                        "Subscribe to the newsletter"
                    </Checkbox>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="indeterminate" anchor=true>
            "Indeterminate"
        </Head>
        <p>"Set "<code>indeterminate</code>" to draw the checkbox in the state a “select all” control takes "
            "while its group is partially selected. The state is a DOM property rather than an attribute, so "
            "the markup carries a marker and "<code>"init_checkboxes"</code>" applies it on the client; "
            "clicking the checkbox clears it natively."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Checkbox name="select-all" indeterminate=true>"Select all"</Checkbox>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox name="select-all" indeterminate=true>"Select all"</Checkbox>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Set "<code>disabled</code>" to disable the checkbox."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Checkbox name="marketing" disabled=true>"I accept marketing emails"</Checkbox>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox name="marketing" disabled=true>"I accept marketing emails"</Checkbox>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The checkbox follows its "<code>"size-*"</code>" class, like the other form controls."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Checkbox class=SIZE_EXTRA_SMALL>"Extra small"</Checkbox>
                    <Checkbox class=SIZE_SMALL>"Small"</Checkbox>
                    <Checkbox class=SIZE_MEDIUM>"Medium"</Checkbox>
                    <Checkbox class=SIZE_LARGE>"Large"</Checkbox>
                    <Checkbox class=SIZE_EXTRA_LARGE>"Extra large"</Checkbox>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Checkbox class=SIZE_SMALL>"Small"</Checkbox>
                    <Checkbox class=SIZE_LARGE>"Large"</Checkbox>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
