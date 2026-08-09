use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{
    CLUSTER, END, GAP_2XS, ICON, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL, START,
};
use wingy_hypertext::component::badge::Badge;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Badge"</Head>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Badge>"Badge"</Badge>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Badge>"Badge"</Badge>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="variants" anchor=true>
            "Variants"
        </Head>
        <p>"Use the "<code>variant</code>" attribute to set the badge's semantic variant."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Badge variant=Neutral>"Neutral"</Badge>
                    <Badge variant=Brand>"Brand"</Badge>
                    <Badge variant=Success>"Success"</Badge>
                    <Badge variant=Warning>"Warning"</Badge>
                    <Badge variant=Danger>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Badge variant=Neutral>"Neutral"</Badge>
                    <Badge variant=Brand>"Brand"</Badge>
                    <Badge variant=Success>"Success"</Badge>
                    <Badge variant=Warning>"Warning"</Badge>
                    <Badge variant=Danger>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the badge's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Neutral appearance=Accent>"Accent"</Badge>
                    <Badge variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Neutral appearance=Filled>"Filled"</Badge>
                    <Badge variant=Neutral appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Brand appearance=Accent>"Accent"</Badge>
                    <Badge variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Brand appearance=Filled>"Filled"</Badge>
                    <Badge variant=Brand appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Success appearance=Accent>"Accent"</Badge>
                    <Badge variant=Success appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Success appearance=Filled>"Filled"</Badge>
                    <Badge variant=Success appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Warning appearance=Accent>"Accent"</Badge>
                    <Badge variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Warning appearance=Filled>"Filled"</Badge>
                    <Badge variant=Warning appearance=Outlined>"Outlined"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Badge variant=Danger appearance=Accent>"Accent"</Badge>
                    <Badge variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Badge>
                    <Badge variant=Danger appearance=Filled>"Filled"</Badge>
                    <Badge variant=Danger appearance=Outlined>"Outlined"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Neutral appearance=Accent>"Accent"</Badge>
                        <Badge variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Neutral appearance=Filled>"Filled"</Badge>
                        <Badge variant=Neutral appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Brand appearance=Accent>"Accent"</Badge>
                        <Badge variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Brand appearance=Filled>"Filled"</Badge>
                        <Badge variant=Brand appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Success appearance=Accent>"Accent"</Badge>
                        <Badge variant=Success appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Success appearance=Filled>"Filled"</Badge>
                        <Badge variant=Success appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Warning appearance=Accent>"Accent"</Badge>
                        <Badge variant=Warning appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Warning appearance=Filled>"Filled"</Badge>
                        <Badge variant=Warning appearance=Outlined>"Outlined"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS)>
                        <Badge variant=Danger appearance=Accent>"Accent"</Badge>
                        <Badge variant=Danger appearance=FilledOutlined>"Filled-Outlined"</Badge>
                        <Badge variant=Danger appearance=Filled>"Filled"</Badge>
                        <Badge variant=Danger appearance=Outlined>"Outlined"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Badges are sized relative to the current font size. You can set "
            <code>"font-size"</code>" style or the corresponding "
            <code>"size-*"</code>
            " class on any badge (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Brand class=SIZE_EXTRA_SMALL>"Brand"</Badge>
                    <Badge variant=Brand class=SIZE_SMALL>"Brand"</Badge>
                    <Badge variant=Brand class=SIZE_MEDIUM>"Brand"</Badge>
                    <Badge variant=Brand class=SIZE_LARGE>"Brand"</Badge>
                    <Badge variant=Brand class=SIZE_EXTRA_LARGE>"Brand"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Badge>
                    <Badge variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Brand class=SIZE_EXTRA_SMALL>"Brand"</Badge>
                        <Badge variant=Brand class=SIZE_SMALL>"Brand"</Badge>
                        <Badge variant=Brand class=SIZE_MEDIUM>"Brand"</Badge>
                        <Badge variant=Brand class=SIZE_LARGE>"Brand"</Badge>
                        <Badge variant=Brand class=SIZE_EXTRA_LARGE>"Brand"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge variant=Brand style="font-size: var(--wa-font-size-xs)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-s)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-m)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-l)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-xl)">"Brand"</Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS)>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-2xs)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-2xl)">"Brand"</Badge>
                        <Badge variant=Brand style="font-size: var(--wa-font-size-3xl)">"Brand"</Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill-badges" anchor=true>
            "Pill Badges"
        </Head>
        <p>"Use the "<code>"pill"</code>" attribute to give badges rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Badge variant=Neutral pill=true>"Neutral"</Badge>
                    <Badge variant=Brand pill=true>"Brand"</Badge>
                    <Badge variant=Success pill=true>"Success"</Badge>
                    <Badge variant=Warning pill=true>"Warning"</Badge>
                    <Badge variant=Danger pill=true>"Danger"</Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Badge variant=Neutral pill=true>"Neutral"</Badge>
                    <Badge variant=Brand pill=true>"Brand"</Badge>
                    <Badge variant=Success pill=true>"Success"</Badge>
                    <Badge variant=Warning pill=true>"Warning"</Badge>
                    <Badge variant=Danger pill=true>"Danger"</Badge>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="start-end-decorations" anchor=true>
            "Start & End Decorations"
        </Head>
        <p>"Use the "<code>"start"</code>" and "<code>"end"</code>" classes to add presentational elements like icons alongside the badge's label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Badge variant=Brand>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Start"
                    </Badge>
                    <Badge variant=Brand>
                        "End"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                    <Badge variant=Brand>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Badge variant=Brand appearance=Outlined class=SIZE_EXTRA_SMALL>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class=SIZE_SMALL>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class=SIZE_MEDIUM>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class=SIZE_LARGE>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                    <Badge variant=Brand appearance=Outlined class=SIZE_EXTRA_LARGE>
                        <span class=(START, " ", ICON)>
                            (fontawesome::solid::Check)
                        </span>
                        "Both"
                        <span class=(END, " ", ICON)>
                            (fontawesome::solid::Bookmark)
                        </span>
                    </Badge>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                        <Badge>
                            <span class=START>...</span>
                            "Start"
                        </Badge>
                        <Badge>
                            "End"
                            <span class=END>...</span>
                        </Badge>
                        <Badge>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS)>
                        <Badge variant=Brand appearance=Outlined  class=SIZE_EXTRA_SMALL>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class=SIZE_SMALL>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class=SIZE_MEDIUM>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class=SIZE_LARGE>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                        <Badge variant=Brand appearance=Outlined  class=SIZE_EXTRA_LARGE>
                            <span class=START>...</span>
                            "Both"
                            <span class=END>...</span>
                        </Badge>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
