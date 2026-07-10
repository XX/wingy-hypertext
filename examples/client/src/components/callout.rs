use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{
    CALLOUT_ICON, CALLOUT_MESSAGE, ICON, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, STACK,
};
use wingy_hypertext::components::button::Button;
use wingy_hypertext::components::callout::Callout;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::variant::Variant::*;

use crate::fontawesome;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Callout"</Head>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Callout icon=(fontawesome::icon("circle-info"))>
                    "This is a standard callout. You can customize its content and even the icon."
                </Callout>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout icon=(fontawesome::icon("circle-info"))>
                        "This is a standard callout. You can customize its content and even the icon."
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="variants" anchor=true>
            "Variants"
        </Head>
        <p>"Use the "<code>variant</code>" attribute to match the callout to its message."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Callout variant=Brand icon=(fontawesome::icon("circle-info"))>
                        <strong>"A new theme is available"</strong><br/>
                        "Try it from Settings whenever you're ready."
                    </Callout>
                    <Callout variant=Success icon=(fontawesome::icon("circle-check"))>
                        <strong>"Your changes have been saved"</strong><br/>
                        "You can safely close this tab now."
                    </Callout>
                    <Callout variant=Neutral icon=(fontawesome::icon("gear"))>
                        <strong>"Your settings have been updated"</strong><br/>
                        "Changes take effect on your next login."
                    </Callout>
                    <Callout variant=Warning icon=(fontawesome::icon("triangle-exclamation"))>
                        <strong>"Your session is about to expire"</strong><br/>
                        "Save your work to avoid losing it."
                    </Callout>
                    <Callout variant=Danger icon=(fontawesome::icon("circle-exclamation"))>
                        <strong>"This action can't be undone"</strong><br/>
                        "Deleting a project removes it for everyone on the team."
                    </Callout>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout variant=Brand icon=(fontawesome::icon("circle-info"))>
                        <strong>"A new theme is available"</strong><br/>
                        "Try it from Settings whenever you're ready."
                    </Callout>
                    <Callout variant=Success icon=(fontawesome::icon("circle-check"))>
                        <strong>"Your changes have been saved"</strong><br/>
                        "You can safely close this tab now."
                    </Callout>
                    <Callout variant=Neutral icon=(fontawesome::icon("gear"))>
                        <strong>"Your settings have been updated"</strong><br/>
                        "Changes take effect on your next login."
                    </Callout>
                    <Callout variant=Warning icon=(fontawesome::icon("triangle-exclamation"))>
                        <strong>"Your session is about to expire"</strong><br/>
                        "Save your work to avoid losing it."
                    </Callout>
                    <Callout variant=Danger icon=(fontawesome::icon("circle-exclamation"))>
                        <strong>"This action can't be undone"</strong><br/>
                        "Deleting a project removes it for everyone on the team."
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" attribute to change the callout's visual style. With no "
            <code>appearance</code>
            " set, a callout renders with a quiet fill and border, matching "<code>"filled-outlined"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Callout variant=Brand appearance=Accent icon=(fontawesome::icon("circle-info"))>
                        "This "<strong>"accent"</strong>" callout draws the most attention."
                    </Callout>
                    <Callout variant=Brand appearance=FilledOutlined icon=(fontawesome::icon("circle-info"))>
                        "This callout is both "<strong>"filled"</strong>" and "<strong>"outlined"</strong>"."
                    </Callout>
                    <Callout variant=Brand appearance=Filled icon=(fontawesome::icon("circle-info"))>
                        "This callout is only "<strong>"filled"</strong>"."
                    </Callout>
                    <Callout variant=Brand appearance=Outlined icon=(fontawesome::icon("circle-info"))>
                        "Here's an "<strong>"outlined"</strong>" callout."
                    </Callout>
                    <Callout variant=Brand appearance=Plain icon=(fontawesome::icon("circle-info"))>
                        "No fill or border on this "<strong>"plain"</strong>" callout."
                    </Callout>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout variant=Brand appearance=Accent icon=(fontawesome::icon("circle-info"))>
                        "This "<strong>"accent"</strong>" callout draws the most attention."
                    </Callout>
                    <Callout variant=Brand appearance=FilledOutlined icon=(fontawesome::icon("circle-info"))>
                        "This callout is both "<strong>"filled"</strong>" and "<strong>"outlined"</strong>"."
                    </Callout>
                    <Callout variant=Brand appearance=Filled icon=(fontawesome::icon("circle-info"))>
                        "This callout is only "<strong>"filled"</strong>"."
                    </Callout>
                    <Callout variant=Brand appearance=Outlined icon=(fontawesome::icon("circle-info"))>
                        "Here's an "<strong>"outlined"</strong>" callout."
                    </Callout>
                    <Callout variant=Brand appearance=Plain icon=(fontawesome::icon("circle-info"))>
                        "No fill or border on this "<strong>"plain"</strong>" callout."
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Callouts are sized relative to the current font size. You can set "
            <code>"font-size"</code>" style or the corresponding "
            <code>"size-*"</code>
            " class on any callout (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Callout class=SIZE_EXTRA_SMALL icon=(fontawesome::icon("circle-info"))>
                        "Extra-small callout for minimal emphasis."
                    </Callout>
                    <Callout class=SIZE_SMALL icon=(fontawesome::icon("circle-info"))>
                        "Small callout for a bit of emphasis."
                    </Callout>
                    <Callout class=SIZE_MEDIUM icon=(fontawesome::icon("circle-info"))>
                        "Medium callout, the default size."
                    </Callout>
                    <Callout class=SIZE_LARGE icon=(fontawesome::icon("circle-info"))>
                        "Large callout for more emphasis."
                    </Callout>
                    <Callout class=SIZE_EXTRA_LARGE icon=(fontawesome::icon("circle-info"))>
                        "Extra-large callout for maximum emphasis."
                    </Callout>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout class=SIZE_EXTRA_SMALL icon=(fontawesome::icon("circle-info"))>
                        "Extra-small callout for minimal emphasis."
                    </Callout>
                    <Callout class=SIZE_SMALL icon=(fontawesome::icon("circle-info"))>
                        "Small callout for a bit of emphasis."
                    </Callout>
                    <Callout class=SIZE_MEDIUM icon=(fontawesome::icon("circle-info"))>
                        "Medium callout, the default size."
                    </Callout>
                    <Callout class=SIZE_LARGE icon=(fontawesome::icon("circle-info"))>
                        "Large callout for more emphasis."
                    </Callout>
                    <Callout class=SIZE_EXTRA_LARGE icon=(fontawesome::icon("circle-info"))>
                        "Extra-large callout for maximum emphasis."
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="without-icon" anchor=true>
            "Without an Icon"
        </Head>
        <p>"Icons are optional. Omit the "<code>icon</code>" attribute for a text-only callout."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Callout variant=Brand>"All times are shown in your local timezone."</Callout>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Callout variant=Brand>"All times are shown in your local timezone."</Callout>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="bare-body" anchor=true>
            "Bare Body"
        </Head>
        <p>"Use the "<code>bare</code>
            " attribute to take full control of the callout's body: children are rendered as is, without the "
            <code>"callout-message"</code>
            " container, so you can lay out the markup manually — including the icon."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Callout variant=Success bare=true>
                    <div class=CALLOUT_ICON>
                        (fontawesome::icon("circle-check"))
                    </div>
                    <div class=CALLOUT_MESSAGE>
                        <strong>"All checks have passed"</strong><br/>
                        "This body markup is provided entirely by hand via children."
                    </div>
                </Callout>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout variant=Success bare=true>
                        <div class=CALLOUT_ICON>
                            (fontawesome::icon("circle-check"))
                        </div>
                        <div class=CALLOUT_MESSAGE>
                            <strong>"All checks have passed"</strong><br/>
                            "This body markup is provided entirely by hand via children."
                        </div>
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="close-button" anchor=true>
            "With close button"
        </Head>
        <p>"A callout has no built-in close behavior, but you can place a close "
            <code>Button</code>" with the "<code>close</code>
            " class anywhere inside its content. The click on the button must be handled explicitly: either call the ready-made "
            <code>"wingy_hypertext_web::components::callout::listen_close_callout"</code>
            " once at startup (as this page does in "<code>"init()"</code>
            ") — it listens for clicks on a "<code>close</code>
            " element and removes the closest "<code>callout</code>
            " from the DOM — or register your own handler manually."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Callout variant=Danger icon=(fontawesome::icon("circle-exclamation"))>
                    <div class="wa-flank:end wa-align-items-start">
                        <div>
                            <strong>"This action can't be undone"</strong><br/>
                            "Deleting a project removes it for everyone on the team."
                        </div>
                        <div>
                            <Button class="close" class=SIZE_SMALL appearance=Plain variant=Danger pill=true>
                                <span class=ICON>
                                    (fontawesome::icon("xmark"))
                                </span>
                            </Button>
                        </div>
                    </div>
                </Callout>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Callout variant=Danger icon=(fontawesome::icon("circle-exclamation"))>
                        <div class="wa-flank:end wa-align-items-start">
                            <div>
                                <strong>"This action can't be undone"</strong><br/>
                                "Deleting a project removes it for everyone on the team."
                            </div>
                            <div>
                                <Button class="close" class=SIZE_SMALL appearance=Plain variant=Danger pill=true>
                                    <span class=ICON>
                                        (fontawesome::icon("xmark"))
                                    </span>
                                </Button>
                            </div>
                        </div>
                    </Callout>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
