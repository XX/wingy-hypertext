use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::input::Input;
use wingy_hypertext::components::input::InputType::*;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Input"</Head>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Input label="Name" placeholder="Enter your name" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Input label="Name" placeholder="Enter your name" />"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="labels" anchor=true>
            "Labels"
        </Head>
        <p>"Use the "<code>"label"</code>" attribute to give the input an accessible label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Input label="What is your name?" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Input label="What is your name?" />"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="hint" anchor=true>
            "Hint"
        </Head>
        <p>"Add descriptive help text to an input with the "<code>"hint"</code>" attribute."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Input label="Nickname" hint="What would you like people to call you?" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Input label="Nickname" hint="What would you like people to call you?" />"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placeholder" anchor=true>
            "Placeholder"
        </Head>
        <p>"Use the "<code>"placeholder"</code>" attribute to add placeholder text."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Input placeholder="Type something" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Input placeholder="Type something" />"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="types" anchor=true>
            "Types"
        </Head>
        <p>"Use the "<code>"input_type"</code>" attribute to change the type of data the input accepts."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-stack wa-gap-s">
                    <Input input_type=Email placeholder="Email" />
                    <Input input_type=Password placeholder="Password" />
                    <Input input_type=Number placeholder="Number" />
                    <Input input_type=Search placeholder="Search" />
                    <Input input_type=Date />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Input input_type=Email placeholder="Email" />
                    <Input input_type=Password placeholder="Password" />
                    <Input input_type=Number placeholder="Number" />
                    <Input input_type=Search placeholder="Search" />
                    <Input input_type=Date />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the input's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-stack wa-gap-s">
                    <Input appearance=Outlined placeholder="Outlined" />
                    <Input appearance=Filled placeholder="Filled" />
                    <Input appearance=FilledOutlined placeholder="Filled-Outlined" />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Input appearance=Outlined placeholder="Outlined" />
                    <Input appearance=Filled placeholder="Filled" />
                    <Input appearance=FilledOutlined placeholder="Filled-Outlined" />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Inputs are sized relative to the current font size. Set the "
            <code>"font-size"</code>
            " style or the corresponding "
            <code>"size-*"</code>
            " class on any input (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-stack wa-gap-s">
                    <Input class="size-small" placeholder="Small" />
                    <Input class="size-medium" placeholder="Medium" />
                    <Input class="size-large" placeholder="Large" />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Input class="size-small" placeholder="Small" />
                    <Input class="size-medium" placeholder="Medium" />
                    <Input class="size-large" placeholder="Large" />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill" anchor=true>
            "Pill"
        </Head>
        <p>"Use the "<code>"pill"</code>" attribute to give inputs rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Input pill=true placeholder="Pill input" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Input pill=true placeholder="Pill input" />"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="states" anchor=true>
            "Disabled, Readonly & Required"
        </Head>
        <p>"Use the "<code>"disabled"</code>", "<code>"readonly"</code>", and "<code>"required"</code>" attributes to control the input's state."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-stack wa-gap-s">
                    <Input label="Disabled" placeholder="You can't edit me" disabled=true />
                    <Input label="Readonly" value="Read-only value" readonly=true />
                    <Input label="Required" placeholder="This field is required" required=true />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Input label="Disabled" placeholder="You can't edit me" disabled=true />
                    <Input label="Readonly" value="Read-only value" readonly=true />
                    <Input label="Required" placeholder="This field is required" required=true />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
