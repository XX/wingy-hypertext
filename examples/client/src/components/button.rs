use hypertext::prelude::{GlobalAttributes, SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::components::button::Button;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::link::LinkSetters;
use wingy_hypertext::link::Target::*;
use wingy_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Button"</Head>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button>"Button"</Button>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Button>"Button"</Button>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="variants" anchor=true>
            "Variants"
        </Head>
        <p>"Use the "<code>variant</code>" attribute to set the button's semantic variant."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Neutral>"Neutral"</Button>
                    <Button variant=Brand>"Brand"</Button>
                    <Button variant=Success>"Success"</Button>
                    <Button variant=Warning>"Warning"</Button>
                    <Button variant=Danger>"Danger"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button variant=Neutral>"Neutral"</Button>
                    <Button variant=Brand>"Brand"</Button>
                    <Button variant=Success>"Success"</Button>
                    <Button variant=Warning>"Warning"</Button>
                    <Button variant=Danger>"Danger"</Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the button's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Neutral appearance=Accent>"Accent"</Button>
                    <Button variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Neutral appearance=Filled>"Filled"</Button>
                    <Button variant=Neutral appearance=Outlined>"Outlined"</Button>
                    <Button variant=Neutral appearance=Plain>"Plain"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand appearance=Accent>"Accent"</Button>
                    <Button variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Brand appearance=Filled>"Filled"</Button>
                    <Button variant=Brand appearance=Outlined>"Outlined"</Button>
                    <Button variant=Brand appearance=Plain>"Plain"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Success appearance=Accent>"Accent"</Button>
                    <Button variant=Success appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Success appearance=Filled>"Filled"</Button>
                    <Button variant=Success appearance=Outlined>"Outlined"</Button>
                    <Button variant=Success appearance=Plain>"Plain"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Warning appearance=Accent>"Accent"</Button>
                    <Button variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Warning appearance=Filled>"Filled"</Button>
                    <Button variant=Warning appearance=Outlined>"Outlined"</Button>
                    <Button variant=Warning appearance=Plain>"Plain"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Danger appearance=Accent>"Accent"</Button>
                    <Button variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Button>
                    <Button variant=Danger appearance=Filled>"Filled"</Button>
                    <Button variant=Danger appearance=Outlined>"Outlined"</Button>
                    <Button variant=Danger appearance=Plain>"Plain"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Neutral appearance=Accent>"Accent"</Button>
                        <Button variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Neutral appearance=Filled>"Filled"</Button>
                        <Button variant=Neutral appearance=Outlined>"Outlined"</Button>
                        <Button variant=Neutral appearance=Plain>"Plain"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand appearance=Accent>"Accent"</Button>
                        <Button variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Brand appearance=Filled>"Filled"</Button>
                        <Button variant=Brand appearance=Outlined>"Outlined"</Button>
                        <Button variant=Brand appearance=Plain>"Plain"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Success appearance=Accent>"Accent"</Button>
                        <Button variant=Success appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Success appearance=Filled>"Filled"</Button>
                        <Button variant=Success appearance=Outlined>"Outlined"</Button>
                        <Button variant=Success appearance=Plain>"Plain"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Warning appearance=Accent>"Accent"</Button>
                        <Button variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Warning appearance=Filled>"Filled"</Button>
                        <Button variant=Warning appearance=Outlined>"Outlined"</Button>
                        <Button variant=Warning appearance=Plain>"Plain"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Button variant=Danger appearance=Accent>"Accent"</Button>
                        <Button variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Button>
                        <Button variant=Danger appearance=Filled>"Filled"</Button>
                        <Button variant=Danger appearance=Outlined>"Outlined"</Button>
                        <Button variant=Danger appearance=Plain>"Plain"</Button>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Buttons are sized relative to the current font size. You can set "
            <code>"font-size"</code>
            " style or the corresponding "
            <code>"size-*"</code>
            " class on any button (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand class="size-extra-small">"Brand"</Button>
                    <Button variant=Brand class="size-small">"Brand"</Button>
                    <Button variant=Brand class="size-medium">"Brand"</Button>
                    <Button variant=Brand class="size-large">"Brand"</Button>
                    <Button variant=Brand class="size-extra-large">"Brand"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                    <Button variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Button>
                </div>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Button>
                    <Button variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand class="size-extra-small">"Brand"</Button>
                        <Button variant=Brand class="size-small">"Brand"</Button>
                        <Button variant=Brand class="size-medium">"Brand"</Button>
                        <Button variant=Brand class="size-large">"Brand"</Button>
                        <Button variant=Brand class="size-extra-large">"Brand"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs" style="margin-block-end: 1rem;">
                        <Button variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Button>
                    </div>
                    <div class="wa-cluster wa-gap-2xs">
                        <Button variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Button>
                        <Button variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Button>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill-buttons" anchor=true>
            "Pill Buttons"
        </Head>
        <p>"Use the "<code>"pill"</code>" attribute to give buttons rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Button variant=Neutral pill=true class="size-extra-small">"Neutral"</Button>
                    <Button variant=Brand pill=true class="size-small">"Brand"</Button>
                    <Button variant=Success pill=true class="size-medium">"Success"</Button>
                    <Button variant=Warning pill=true class="size-large">"Warning"</Button>
                    <Button variant=Danger pill=true class="size-extra-large">"Danger"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button variant=Neutral pill=true class="size-extra-small">"Neutral"</Button>
                    <Button variant=Brand pill=true class="size-small">"Brand"</Button>
                    <Button variant=Success pill=true class="size-medium">"Success"</Button>
                    <Button variant=Warning pill=true class="size-large">"Warning"</Button>
                    <Button variant=Danger pill=true class="size-extra-large">"Danger"</Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="link-buttons" anchor=true>
            "Link Buttons"
        </Head>
        <p>"It's often helpful to have a button that works like a link. This is possible by setting the "
          <code>"href"</code>" attribute, which will make the component render an "<code>"<a>"</code>
          " under the hood. This gives you all the default link behavior the browser provides  and exposes the "
          <code>"rel"</code>", "<code>"target"</code>", and "<code>"download"</code>" attributes."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Button href="https://example.com/">"Link"</Button>
                    <Button href="https://example.com/" target=Blank>"New Window"</Button>
                    <Button href="https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/car.svg" download="auto.svg">"Download"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button href="https://example.com/">"Link"</Button>
                    <Button href="https://example.com/" target=Blank>"New Window"</Button>
                    <Button href="https://dev.w3.org/SVG/tools/svgweb/samples/svg-files/car.svg" download="auto.svg">"Download"</Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="icon-buttons" anchor=true>
            "Icon Buttons"
        </Head>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="wa-cluster wa-gap-2xs">
                    <Button appearance=Accent>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                    <Button appearance=FilledOutlined>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                    <Button appearance=Filled>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                    <Button appearance=Outlined>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                    <Button appearance=Plain>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Accent>
                        <span class="icon">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M277.8 8.6c-12.3-11.4-31.3-11.4-43.5 0l-224 208c-9.6 9-12.8 22.9-8 35.1S18.8 272 32 272l16 0 0 176c0 35.3 28.7 64 64 64l288 0c35.3 0 64-28.7 64-64l0-176 16 0c13.2 0 25-8.1 29.8-20.3s1.6-26.2-8-35.1l-224-208zM240 320l32 0c26.5 0 48 21.5 48 48l0 96-128 0 0-96c0-26.5 21.5-48 48-48z"/>
                            </svg>
                        </span>
                    </Button>
                    <Button appearance=FilledOutlined>
                        <span class="icon">
                            ...
                        </span>
                    </Button>
                    <Button appearance=Filled>
                        <span class="icon">
                            ...
                        </span>
                    </Button>
                    <Button appearance=Outlined>
                        <span class="icon">
                            ...
                        </span>
                    </Button>
                    <Button appearance=Plain>
                        <span class="icon">
                            ...
                        </span>
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
