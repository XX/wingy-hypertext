use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{GAP_L, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::textarea::Resize::*;
use wingy_hypertext::component::textarea::{Textarea, TextareaParamBuilder};
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Textarea"</Head>
        <p>"Textareas collect multiline text from the user. It is the "<code>Input</code>" of long text: the "
            "same label, hint, appearance and states around a native "<code>"<textarea>"</code>", so typing, "
            "resizing by hand, validation and form submission are native."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea label="Feedback" name="feedback"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Feedback" name="feedback"/>
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
        <p>"Use the "<code>label</code>" property to give the textarea an accessible label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea label="Comments"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Comments"/>
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
                <Textarea label="Feedback" hint="Please tell us what you think."/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Feedback" hint="Please tell us what you think."/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placeholder" anchor=true>
            "Placeholder"
        </Head>
        <p>"Use the "<code>placeholder</code>" property to add placeholder text."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea label="Comments" placeholder="Share your thoughts"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Comments" placeholder="Share your thoughts"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the textarea's look."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Textarea appearance=Outlined placeholder="outlined"/>
                    <Textarea appearance=Filled placeholder="filled"/>
                    <Textarea appearance=FilledOutlined placeholder="filled-outlined"/>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea appearance=Outlined placeholder="outlined"/>
                    <Textarea appearance=Filled placeholder="filled"/>
                    <Textarea appearance=FilledOutlined placeholder="filled-outlined"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Set "<code>disabled</code>" to disable the textarea."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea placeholder="Disabled" disabled=true/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea placeholder="Disabled" disabled=true/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="readonly" anchor=true>
            "Readonly"
        </Head>
        <p>"Set "<code>readonly</code>" to keep a value visible but uneditable. Unlike "<code>disabled</code>
            ", a readonly textarea stays focusable and its value is still submitted with the form."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea
                    label="Release notes"
                    value="Fixed a handful of bugs and polished the edges."
                    readonly=true
                />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea
                        label="Release notes"
                        value="Fixed a handful of bugs and polished the edges."
                        readonly=true
                    />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The textarea follows its "<code>"size-*"</code>" class, like the other form controls."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Textarea class=SIZE_EXTRA_SMALL rows=2 placeholder="Extra small"/>
                    <Textarea class=SIZE_SMALL rows=2 placeholder="Small"/>
                    <Textarea class=SIZE_MEDIUM rows=2 placeholder="Medium"/>
                    <Textarea class=SIZE_LARGE rows=2 placeholder="Large"/>
                    <Textarea class=SIZE_EXTRA_LARGE rows=2 placeholder="Extra large"/>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea class=SIZE_SMALL rows=2 placeholder="Small"/>
                    <Textarea class=SIZE_LARGE rows=2 placeholder="Large"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="rows" anchor=true>
            "Rows"
        </Head>
        <p>"Use "<code>rows</code>" to change the number of text rows the field shows by default."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea rows=2 label="Two rows"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea rows=2 label="Two rows"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="resize" anchor=true>
            "Resize"
        </Head>
        <p>"Use "<code>resize</code>" to control how the field can be resized: "<code>Vertical</code>
            " by default, "<code>None</code>", "<code>Horizontal</code>", "<code>Both</code>", or "
            <code>Auto</code>" to grow with the content as the user types. The first four are the native CSS "
            "resize; "<code>Auto</code>" is measured on the client."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Textarea label="Vertical (default)" rows=2/>
                    <Textarea label="None" resize=None rows=2/>
                    <Textarea label="Both" resize=Both rows=2/>
                    <Textarea
                        label="Auto"
                        resize=Auto
                        rows=2
                        hint="Type a few lines: the field grows with them."
                    />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Vertical (default)" rows=2/>
                    <Textarea label="None" resize=None rows=2/>
                    <Textarea label="Both" resize=Both rows=2/>
                    <Textarea label="Auto" resize=Auto rows=2/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="character-count" anchor=true>
            "Character Count"
        </Head>
        <p>"Set "<code>count</code>" to show a character count below the field. With "<code>maxlength</code>
            " it shows how many characters are left instead. The length is counted the way the browser counts "
            "it for "<code>maxlength</code>", so the two never disagree."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <Textarea label="Comments" hint="Share your thoughts with us" rows=2 count=true/>
                    <Textarea
                        label="Bio"
                        hint="Tell us a little about yourself"
                        rows=2
                        count=true
                        maxlength=100
                    />
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea label="Comments" hint="Share your thoughts with us" count=true/>
                    <Textarea label="Bio" count=true maxlength=100/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="initial-value" anchor=true>
            "Initial Value"
        </Head>
        <p>"Use "<code>value</code>" to set the initial value. A textarea carries it as its content, so the "
            "field is filled in even before any script runs."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Textarea value="Write something awesome!"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Textarea value="Write something awesome!"/>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
