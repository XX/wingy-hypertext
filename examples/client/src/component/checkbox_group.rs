use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{GAP_L, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK};
use wingy_hypertext::component::checkbox::Checkbox;
use wingy_hypertext::component::checkbox_group::CheckboxGroup;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::switch::Switch;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::orientation::Orientation::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Checkbox Group"</Head>
        <p>"Checkboxes in a group remain independent form controls with their own "<code>name</code>", "
            <code>value</code>" and validation. The group exists to give them a shared label, a hint and an "
            "accessible grouping — it is a native "<code>"<fieldset>"</code>", so "<code>disabled</code>
            " on it disables every item inside."
        </p>
        <p>"Unlike the choices of a radio group, the items keep their own width, so a click lands on the label "
            "rather than on the whole row."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <CheckboxGroup label="Interests">
                    <Checkbox name="design">"Design"</Checkbox>
                    <Checkbox name="development">"Development"</Checkbox>
                    <Checkbox name="marketing">"Marketing"</Checkbox>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Interests">
                        <Checkbox name="design">"Design"</Checkbox>
                        <Checkbox name="development">"Development"</Checkbox>
                        <Checkbox name="marketing">"Marketing"</Checkbox>
                    </CheckboxGroup>
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
        <p>"Use the "<code>label</code>" property to give the group an accessible label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <CheckboxGroup label="Toppings">
                    <Checkbox name="pepperoni">"Pepperoni"</Checkbox>
                    <Checkbox name="mushrooms">"Mushrooms"</Checkbox>
                    <Checkbox name="onions">"Onions"</Checkbox>
                    <Checkbox name="extra-cheese">"Extra cheese"</Checkbox>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Toppings">
                        <Checkbox name="pepperoni">"Pepperoni"</Checkbox>
                        <Checkbox name="mushrooms">"Mushrooms"</Checkbox>
                        <Checkbox name="onions">"Onions"</Checkbox>
                    </CheckboxGroup>
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
                <CheckboxGroup label="Workdays" hint="Choose as many as you like.">
                    <Checkbox name="monday">"Monday"</Checkbox>
                    <Checkbox name="wednesday">"Wednesday"</Checkbox>
                    <Checkbox name="friday">"Friday"</Checkbox>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Workdays" hint="Choose as many as you like.">
                        <Checkbox name="monday">"Monday"</Checkbox>
                        <Checkbox name="wednesday">"Wednesday"</Checkbox>
                        <Checkbox name="friday">"Friday"</Checkbox>
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="orientation" anchor=true>
            "Orientation"
        </Head>
        <p>"Groups stack vertically by default. Set "<code>"orientation=Horizontal"</code>" to lay the items "
            "out in a row."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <CheckboxGroup label="Sizes" orientation=Horizontal>
                    <Checkbox name="small">"Small"</Checkbox>
                    <Checkbox name="medium">"Medium"</Checkbox>
                    <Checkbox name="large">"Large"</Checkbox>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Sizes" orientation=Horizontal>
                        <Checkbox name="small">"Small"</Checkbox>
                        <Checkbox name="medium">"Medium"</Checkbox>
                        <Checkbox name="large">"Large"</Checkbox>
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The size of the items follows the group's "<code>"size-*"</code>" class."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <CheckboxGroup label="Extra small" class=SIZE_EXTRA_SMALL>
                        <Checkbox name="xs-1">"Option 1"</Checkbox>
                        <Checkbox name="xs-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                    <CheckboxGroup label="Small" class=SIZE_SMALL>
                        <Checkbox name="s-1">"Option 1"</Checkbox>
                        <Checkbox name="s-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                    <CheckboxGroup label="Medium" class=SIZE_MEDIUM>
                        <Checkbox name="m-1">"Option 1"</Checkbox>
                        <Checkbox name="m-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                    <CheckboxGroup label="Large" class=SIZE_LARGE>
                        <Checkbox name="l-1">"Option 1"</Checkbox>
                        <Checkbox name="l-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                    <CheckboxGroup label="Extra large" class=SIZE_EXTRA_LARGE>
                        <Checkbox name="xl-1">"Option 1"</Checkbox>
                        <Checkbox name="xl-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Small" class=SIZE_SMALL>
                        <Checkbox name="option-1">"Option 1"</Checkbox>
                        <Checkbox name="option-2">"Option 2"</Checkbox>
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Set "<code>disabled</code>" on an item to disable it, or on the group to disable all of them: the "
            "group is a native "<code>"<fieldset>"</code>", so it does that on its own."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(STACK, " ", GAP_L)>
                    <CheckboxGroup label="Add-ons">
                        <Checkbox name="insurance" disabled=true>"Insurance"</Checkbox>
                        <Checkbox name="gift-wrap" disabled=true>"Gift wrap"</Checkbox>
                        <Checkbox name="express-shipping">"Express shipping"</Checkbox>
                    </CheckboxGroup>
                    <CheckboxGroup label="Add-ons (whole group)" disabled=true>
                        <Checkbox name="insurance-all" checked=true>"Insurance"</Checkbox>
                        <Checkbox name="gift-wrap-all">"Gift wrap"</Checkbox>
                    </CheckboxGroup>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Add-ons">
                        <Checkbox name="insurance" disabled=true>"Insurance"</Checkbox>
                        <Checkbox name="express-shipping">"Express shipping"</Checkbox>
                    </CheckboxGroup>

                    <CheckboxGroup label="Add-ons" disabled=true>
                        ...
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="switches" anchor=true>
            "Switches"
        </Head>
        <p>"A checkbox group also works with "<code>Switch</code>"es."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <CheckboxGroup label="Notifications" hint="Pick at least one channel.">
                    <Switch name="email" checked=true>"Email"</Switch>
                    <Switch name="sms">"SMS"</Switch>
                    <Switch name="push">"Push"</Switch>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Notifications" hint="Pick at least one channel.">
                        <Switch name="email" checked=true>"Email"</Switch>
                        <Switch name="sms">"SMS"</Switch>
                        <Switch name="push">"Push"</Switch>
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="required" anchor=true>
            "Required"
        </Head>
        <p>"Set "<code>required</code>" on the group to mark its label. Since every checkbox is an independent "
            "control, the group doesn't enforce anything by itself — the requirement comes from "
            <code>required</code>" on the checkbox, which the browser validates."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <CheckboxGroup label="Accept terms" required=true>
                    <Checkbox name="terms" value="accepted" required=true>
                        "I agree to the terms and conditions"
                    </Checkbox>
                </CheckboxGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <CheckboxGroup label="Accept terms" required=true>
                        <Checkbox name="terms" value="accepted" required=true>
                            "I agree to the terms and conditions"
                        </Checkbox>
                    </CheckboxGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
