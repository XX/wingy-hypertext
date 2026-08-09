use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::TEXT_CENTER;
use wingy_hypertext::component::callout::Callout;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext::orientation::Orientation::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Divider"</Head>
        <p>"Dividers visually separate or group adjacent elements with a horizontal or vertical line. "
            "Use them to establish rhythm and hierarchy within menus, toolbars, and layouts."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Divider/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Divider/>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="width" anchor=true>
            "Width"
        </Head>
        <p>"Use the "<code>"--width"</code>" custom property to change the width of the divider."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Divider style="--width: 4px;"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Divider style="--width: 4px;"/>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="color" anchor=true>
            "Color"
        </Head>
        <p>"Use the "<code>"--color"</code>" custom property to change the color of the divider."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Divider style="--color: var(--wa-color-brand-fill-loud);"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Divider style="--color: var(--wa-color-brand-fill-loud);"/>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="spacing" anchor=true>
            "Spacing"
        </Head>
        <p>"Use the "<code>"--spacing"</code>
            " custom property to change the amount of space between the divider and its neighboring elements."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=TEXT_CENTER>
                    "Above"
                    <Divider style="--spacing: 2rem;"/>
                    "Below"
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=TEXT_CENTER>
                        "Above"
                        <Divider style="--spacing: 2rem;"/>
                        "Below"
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="orientation" anchor=true>
            "Orientation"
        </Head>
        <p>"The default orientation for dividers is "<code>"Horizontal"</code>". Set the "
            <code>"orientation"</code>" attribute to "<code>"Vertical"</code>
            " to draw a vertical divider. The divider will span the full height of its Flexbox or CSS Grid container."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div style="display: flex; align-items: center;">
                    "First"
                    <Divider orientation=Vertical/>
                    "Middle"
                    <Divider orientation=Vertical/>
                    "Last"
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div style="display: flex; align-items: center;">
                        "First"
                        <Divider orientation=Vertical/>
                        "Middle"
                        <Divider orientation=Vertical/>
                        "Last"
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
        <Callout icon=(fontawesome::solid::CircleInfo)>
            "If your container isn't Flexbox or CSS Grid, you may need to set an explicit height for the divider."
        </Callout>
    }
}
