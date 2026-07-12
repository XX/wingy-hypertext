use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::popup::Placement::*;
use wingy_hypertext::components::popup::{AutoSize, Popup, SyncSize};
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

const ANCHOR_STYLE: &str = "display: inline-block; width: 150px; height: 150px; border: dashed 2px var(--wa-color-neutral-fill-loud); margin: 50px;";
const BOX_STYLE: &str = "width: 100px; height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);";

fn anchor() -> impl Renderable {
    rsx! { <span style=ANCHOR_STYLE></span> }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Popup"</Head>
        <p>"Popup is a low-level utility that anchors one element to another and keeps them positioned "
            "together as the page scrolls or resizes. It provides positioning only — no styles — and serves "
            "as a building block for popovers, dropdowns, and tooltips (the "<code>Select</code>
            " listbox is positioned by it). The positioning logic is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "
            <code>"listen_popups"</code>" and "<code>"init_popups"</code>
            "; the popups below are statically "<code>active</code>" for demonstration."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Top distance=8 active=true anchor=(anchor())>
                    <div style=BOX_STYLE></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Top distance=8 active=true anchor=(rsx! { <span class="anchor"></span> })>
                        <div class="box"></div>
                    </Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="placement" anchor=true>
            "Placement"
        </Head>
        <p>"Use the "<code>placement</code>" attribute to set the preferred placement of the popup: "
            "four sides with optional "<code>"-start"</code>" and "<code>"-end"</code>" alignments."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=TopStart active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"top-start"</div>
                </Popup>
                <Popup placement=Right active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"right"</div>
                </Popup>
                <Popup placement=BottomEnd active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"bottom-end"</div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=TopStart active=true anchor=(...)>...</Popup>
                    <Popup placement=Right active=true anchor=(...)>...</Popup>
                    <Popup placement=BottomEnd active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="distance-skidding" anchor=true>
            "Distance & Skidding"
        </Head>
        <p>"Use the "<code>distance</code>" attribute to offset the popup away from its anchor, and "
            <code>skidding</code>" to offset it along the anchor."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Top distance=24 active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"distance"</div>
                </Popup>
                <Popup placement=Top skidding=40 active=true anchor=(anchor())>
                    <div style=BOX_STYLE>"skidding"</div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Top distance=24 active=true anchor=(...)>...</Popup>
                    <Popup placement=Top skidding=40 active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="arrows" anchor=true>
            "Arrows"
        </Head>
        <p>"Add an arrow to your popup with the "<code>arrow</code>" attribute. Customize its size and color with the "
            <code>"--arrow-size"</code>" and "<code>"--arrow-color"</code>" custom properties, and align it with "
            <code>arrow_placement</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup
                    placement=Top
                    distance=8
                    arrow=true
                    active=true
                    style="--arrow-color: var(--wa-color-brand-fill-loud)"
                    anchor=(anchor())
                >
                    <div style=BOX_STYLE></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup
                        placement=Top
                        distance=8
                        arrow=true
                        active=true
                        style="--arrow-color: var(--wa-color-brand-fill-loud)"
                        anchor=(...)
                    >
                        ...
                    </Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="sync" anchor=true>
            "Syncing with the Anchor's Dimensions"
        </Head>
        <p>"Use the "<code>sync</code>" attribute to make the popup the same width or height as the anchor element. "
            "This is useful for controls that need the popup to stay the same size as the trigger, such as dropdowns."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Bottom distance=8 sync=(SyncSize::Width) active=true anchor=(anchor())>
                    <div style="height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Bottom distance=8 sync=(SyncSize::Width) active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="flip" anchor=true>
            "Flip"
        </Head>
        <p>"When the popup doesn't have enough room in its preferred placement, the "<code>flip</code>
            " attribute moves it to the opposite side instead of letting it overflow. "
            "Scroll the page so this anchor approaches the top of the viewport — the popup flips below."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Top distance=8 flip=true active=true anchor=(anchor())>
                    <div style=BOX_STYLE></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Top distance=8 flip=true active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="shift" anchor=true>
            "Shift"
        </Head>
        <p>"When the popup overflows the viewport along its anchor, the "<code>shift</code>
            " attribute moves it back into view. The popup below is wider than its anchor and hugs the viewport "
            "edge instead of overflowing it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup placement=Bottom distance=8 shift=true shift_padding=8 active=true anchor=(anchor())>
                    <div style="width: 300px; height: 50px; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup placement=Bottom distance=8 shift=true shift_padding=8 active=true anchor=(...)>...</Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="auto-size" anchor=true>
            "Auto-Size"
        </Head>
        <p>"Use the "<code>auto_size</code>" attribute to tell the popup to resize when necessary to prevent it from "
            "overflowing. The available space is exposed to the popup content through the read-only "
            <code>"--auto-size-available-width"</code>" and "<code>"--auto-size-available-height"</code>
            " custom properties."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Popup
                    placement=Bottom
                    distance=8
                    flip=true
                    auto_size=(AutoSize::Vertical)
                    auto_size_padding=10
                    active=true
                    anchor=(anchor())
                >
                    <div style="width: 100px; height: 400px; max-height: var(--auto-size-available-height); overflow: auto; background: var(--wa-color-brand-fill-loud); border-radius: var(--wa-border-radius-m);"></div>
                </Popup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Popup
                        placement=Bottom
                        distance=8
                        flip=true
                        auto_size=(AutoSize::Vertical)
                        auto_size_padding=10
                        active=true
                        anchor=(...)
                    >
                        <div style="max-height: var(--auto-size-available-height); overflow: auto; ...">...</div>
                    </Popup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
