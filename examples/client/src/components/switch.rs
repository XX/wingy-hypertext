use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{HINT, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::switch::{Switch, Toggle};
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Switch"</Head>
        <p>"Switches toggle a single setting on or off and apply the change immediately, "
            "without requiring a form submission."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch>"Enable notifications"</Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch>"Enable notifications"</Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="label" anchor=true>
            "Label"
        </Head>
        <p>"Add label text as the switch's default content. For labels that contain HTML, slot the markup in directly."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch>"Subscribe to the newsletter"</Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch>"Subscribe to the newsletter"</Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="hint" anchor=true>
            "Hint"
        </Head>
        <p>"Add descriptive hint to a switch with the "<code>"hint"</code>
            " attribute. For hints that contain HTML, use the "<code>"bare"</code>
            " attribute and compose the body from a "<code>"Toggle"</code>" and custom hint markup."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch hint="You can change this at any time in settings.">
                    "Email me about new releases"
                </Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch hint="You can change this at any time in settings.">
                        "Email me about new releases"
                    </Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="initial-value" anchor=true>
            "Initial Value"
        </Head>
        <p>"Use the "<code>"checked"</code>" attribute to activate the switch."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch checked=true>"Remember this device"</Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch checked=true>"Remember this device"</Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Use the "<code>"disabled"</code>" attribute to disable the switch."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch disabled=true>"Sync over cellular"</Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch disabled=true>"Sync over cellular"</Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Switches are sized relative to the current font size. You can set "
            <code>"font-size"</code>" style or the corresponding "
            <code>"size-*"</code>
            " class on any switch (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Switch class=SIZE_EXTRA_SMALL>"Extra Small"</Switch>
                    <Switch class=SIZE_SMALL>"Small"</Switch>
                    <Switch class=SIZE_MEDIUM>"Medium"</Switch>
                    <Switch class=SIZE_LARGE>"Large"</Switch>
                    <Switch class=SIZE_EXTRA_LARGE>"Extra Large"</Switch>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Switch class=SIZE_EXTRA_SMALL>"Extra Small"</Switch>
                        <Switch class=SIZE_SMALL>"Small"</Switch>
                        <Switch class=SIZE_MEDIUM>"Medium"</Switch>
                        <Switch class=SIZE_LARGE>"Large"</Switch>
                        <Switch class=SIZE_EXTRA_LARGE>"Extra Large"</Switch>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="custom-properties" anchor=true>
            "Custom Properties"
        </Head>
        <p>"Use the available custom properties to change how the switch is styled."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch style="--width: 80px; --height: 40px; --thumb-size: 36px;">"Really big"</Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch style="--width: 80px; --height: 40px; --thumb-size: 36px;">"Really big"</Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="bare-body" anchor=true>
            "Bare Body"
        </Head>
        <p>"Use the "<code>bare</code>
            " attribute to take full control of the switch's body: children are rendered as is, without the "
            <code>"Toggle"</code>
            " component container, so you can lay out the markup manually — including the hint."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Switch bare=true>
                    <Toggle>
                        "Email me about new releases"
                    </Toggle>
                    <small class=HINT>"You can change this "<strong>"at any time"</strong>" in settings."</small>
                </Switch>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Switch bare=true>
                        <Toggle>
                            "Email me about new releases"
                        </Toggle>
                        <small class=HINT>"You can change this "<strong>"at any time"</strong>" in settings."</small>
                    </Switch>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
