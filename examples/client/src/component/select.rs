use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::select::SelectPlacement::Top;
use wingy_hypertext::component::select::{Select, SelectOption};
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Select"</Head>
        <p>"Selects let users choose one or more values from a dropdown list of predefined options, "
            "provided as "<code>SelectOption</code>" children. The dropdown behavior (opening, selection, "
            "keyboard navigation with type-to-select, clearing) is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "
            <code>"listen_selects"</code>" and "<code>"init_selects"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select>
                    <SelectOption value="option-1">"Option 1"</SelectOption>
                    <SelectOption value="option-2">"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select>
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2">"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
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
        <p>"Use the "<code>label</code>" attribute to give the select an accessible label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select label="Favorite Animal">
                    <SelectOption value="birds">"Birds"</SelectOption>
                    <SelectOption value="cats">"Cats"</SelectOption>
                    <SelectOption value="dogs">"Dogs"</SelectOption>
                    <SelectOption value="other">"Other"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select label="Favorite Animal">
                        <SelectOption value="birds">"Birds"</SelectOption>
                        <SelectOption value="cats">"Cats"</SelectOption>
                        <SelectOption value="dogs">"Dogs"</SelectOption>
                        <SelectOption value="other">"Other"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="hint" anchor=true>
            "Hint"
        </Head>
        <p>"Add descriptive hint to a select with the "<code>hint</code>" attribute."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select label="Experience" hint="Please tell us your skill level.">
                    <SelectOption value="novice">"Novice"</SelectOption>
                    <SelectOption value="intermediate">"Intermediate"</SelectOption>
                    <SelectOption value="advanced">"Advanced"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select label="Experience" hint="Please tell us your skill level.">
                        <SelectOption value="novice">"Novice"</SelectOption>
                        <SelectOption value="intermediate">"Intermediate"</SelectOption>
                        <SelectOption value="advanced">"Advanced"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placeholder" anchor=true>
            "Placeholder"
        </Head>
        <p>"Use the "<code>placeholder</code>" attribute to add a placeholder shown while nothing is selected."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select placeholder="Select one">
                    <SelectOption value="option-1">"Option 1"</SelectOption>
                    <SelectOption value="option-2">"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select placeholder="Select one">
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2">"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="clearable" anchor=true>
            "Clearable"
        </Head>
        <p>"Use the "<code>with_clear</code>" attribute to make the control clearable. "
            "The clear button only appears when an option is selected."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select with_clear=true placeholder="Select one">
                    <SelectOption value="option-1" selected=true>"Option 1"</SelectOption>
                    <SelectOption value="option-2">"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select with_clear=true placeholder="Select one">
                        <SelectOption value="option-1" selected=true>"Option 1"</SelectOption>
                        <SelectOption value="option-2">"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" attribute to change the select's visual style."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Select appearance=Outlined>
                        <SelectOption value="option-1">"Outlined"</SelectOption>
                    </Select>
                    <Select appearance=Filled>
                        <SelectOption value="option-1">"Filled"</SelectOption>
                    </Select>
                    <Select appearance=FilledOutlined>
                        <SelectOption value="option-1">"Filled-Outlined"</SelectOption>
                    </Select>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select appearance=Outlined>
                        <SelectOption value="option-1">"Outlined"</SelectOption>
                    </Select>
                    <Select appearance=Filled>
                        <SelectOption value="option-1">"Filled"</SelectOption>
                    </Select>
                    <Select appearance=FilledOutlined>
                        <SelectOption value="option-1">"Filled-Outlined"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill" anchor=true>
            "Pill"
        </Head>
        <p>"Use the "<code>pill</code>" attribute to give selects rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select pill=true>
                    <SelectOption value="option-1">"Option 1"</SelectOption>
                    <SelectOption value="option-2">"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select pill=true>
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2">"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Selects are sized relative to the current font size. You can set "
            <code>"font-size"</code>" style or the corresponding "
            <code>"size-*"</code>
            " class on any select (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Select class=SIZE_EXTRA_SMALL>
                        <SelectOption value="option-1">"Extra small"</SelectOption>
                    </Select>
                    <Select class=SIZE_SMALL>
                        <SelectOption value="option-1">"Small"</SelectOption>
                    </Select>
                    <Select class=SIZE_MEDIUM>
                        <SelectOption value="option-1">"Medium"</SelectOption>
                    </Select>
                    <Select class=SIZE_LARGE>
                        <SelectOption value="option-1">"Large"</SelectOption>
                    </Select>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select class=SIZE_EXTRA_SMALL>
                        <SelectOption value="option-1">"Extra small"</SelectOption>
                    </Select>
                    <Select class=SIZE_SMALL>
                        <SelectOption value="option-1">"Small"</SelectOption>
                    </Select>
                    <Select class=SIZE_MEDIUM>
                        <SelectOption value="option-1">"Medium"</SelectOption>
                    </Select>
                    <Select class=SIZE_LARGE>
                        <SelectOption value="option-1">"Large"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Use the "<code>disabled</code>" attribute to disable a select. "
            "Individual options can be disabled the same way."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Select disabled=true>
                        <SelectOption value="option-1">"Disabled select"</SelectOption>
                    </Select>
                    <Select>
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2" disabled=true>"Option 2 (disabled)"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select disabled=true>
                        <SelectOption value="option-1">"Disabled select"</SelectOption>
                    </Select>
                    <Select>
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2" disabled=true>"Option 2 (disabled)"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="initial-value" anchor=true>
            "Initial Value"
        </Head>
        <p>"Use the "<code>selected</code>" attribute on an option to set the initial selection."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select>
                    <SelectOption value="option-1">"Option 1"</SelectOption>
                    <SelectOption value="option-2" selected=true>"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select>
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2" selected=true>"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="multiple" anchor=true>
            "Multiple"
        </Head>
        <p>"To allow multiple options to be selected, use the "<code>multiple</code>
            " attribute. Clicking an option toggles its selection and the listbox stays open; "
            "the selected options are shown as removable tags. After "
            <code>max_options_visible</code>" tags (3 by default, 0 removes the limit), "
            <code>"+n"</code>" indicates the number of additional selected items."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select label="Favorite Animals" multiple=true with_clear=true>
                    <SelectOption value="birds" selected=true>"Birds"</SelectOption>
                    <SelectOption value="cats">"Cats"</SelectOption>
                    <SelectOption value="dogs" selected=true>"Dogs"</SelectOption>
                    <SelectOption value="other">"Other"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select label="Favorite Animals" multiple=true with_clear=true>
                        <SelectOption value="birds" selected=true>"Birds"</SelectOption>
                        <SelectOption value="cats">"Cats"</SelectOption>
                        <SelectOption value="dogs" selected=true>"Dogs"</SelectOption>
                        <SelectOption value="other">"Other"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="grouping-options" anchor=true>
            "Grouping Options"
        </Head>
        <p>"Use a "<code>Divider</code>" to separate groups of options visually. You can also add "
            <code>small</code>
            " labels, but note that most assistive technologies won't announce them."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select label="Add a language" placeholder="Select one">
                    <small>"Frontend"</small>
                    <SelectOption value="ts">"TypeScript"</SelectOption>
                    <SelectOption value="css">"CSS"</SelectOption>
                    <Divider/>
                    <small>"Backend"</small>
                    <SelectOption value="go">"Go"</SelectOption>
                    <SelectOption value="rust">"Rust"</SelectOption>
                    <SelectOption value="python">"Python"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select label="Add a language" placeholder="Select one">
                        <small>"Frontend"</small>
                        <SelectOption value="ts">"TypeScript"</SelectOption>
                        <SelectOption value="css">"CSS"</SelectOption>
                        <Divider/>
                        <small>"Backend"</small>
                        <SelectOption value="go">"Go"</SelectOption>
                        <SelectOption value="rust">"Rust"</SelectOption>
                        <SelectOption value="python">"Python"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placement" anchor=true>
            "Placement"
        </Head>
        <p>"Set the "<code>placement</code>" attribute to control where the listbox opens: "
            <code>Bottom</code>" (default) or "<code>Top</code>
            ". The actual position may flip to keep the panel in the viewport."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Select placement=Top placeholder="Opens upward">
                    <SelectOption value="option-1">"Option 1"</SelectOption>
                    <SelectOption value="option-2">"Option 2"</SelectOption>
                    <SelectOption value="option-3">"Option 3"</SelectOption>
                </Select>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Select placement=Top placeholder="Opens upward">
                        <SelectOption value="option-1">"Option 1"</SelectOption>
                        <SelectOption value="option-2">"Option 2"</SelectOption>
                        <SelectOption value="option-3">"Option 3"</SelectOption>
                    </Select>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
