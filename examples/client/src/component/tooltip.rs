use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::CastToElement;
use web_sys::Event;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::ICON;
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::tooltip::Tooltip;
use wingy_hypertext::component::tooltip::TooltipTrigger::*;
use wingy_hypertext::helper::popup::PopupPlacement;
use wingy_hypertext::helper::popup::PopupPlacement::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext_web::component::tooltip::{is_open, set_tooltip_open};

/// A button of the placement example: the label is the tooltip's, so the
/// buttons themselves are empty squares.
fn placement_button(id: &'static str) -> impl Renderable {
    rsx! { <Button appearance=Filled id=(id)/> }
}

fn placement_tooltip(anchor_id: &'static str, placement: PopupPlacement, label: &'static str) -> impl Renderable {
    rsx! { <Tooltip anchor_id=(anchor_id) placement=(placement)>(label)</Tooltip> }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Tooltip"</Head>
        <p>"Tooltips display brief contextual information when the user hovers, focuses, or taps a target element. "
            "Point the "<code>anchor_id</code>" property at the "<code>id</code>" of the element the tooltip "
            "describes — the tooltip is rendered as its sibling, positioned with the "<code>Popup</code>
            " helper, and it labels the anchor through "<code>"aria-labelledby"</code>". The behavior (triggers "
            "and their delays, "<code>Escape</code>", repositioning) is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "<code>"listen_tooltips"</code>" and "
            <code>"init_tooltips"</code>"; showing and hiding dispatch the cancelable "<code>"wg-show"</code>"/"
            <code>"wg-hide"</code>" events, followed by "<code>"wg-after-show"</code>"/"<code>"wg-after-hide"</code>"."
        </p>
        <p>"Keep tooltips to text and presentational content: they can't be reliably focused or operated with a "
            "keyboard, so buttons, links and form controls belong into a "<code>Dropdown</code>" instead."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button appearance=Filled id="tooltip-overview-button">"Hover Me"</Button>
                <Tooltip anchor_id="tooltip-overview-button">"This is a tooltip"</Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Filled id="my-button">"Hover Me"</Button>
                    <Tooltip anchor_id="my-button">"This is a tooltip"</Tooltip>
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
        <p>"Use the "<code>placement</code>" property to set the tooltip's preferred position: "<code>Top</code>
            " (default), "<code>Right</code>", "<code>Bottom</code>" or "<code>Left</code>", each with optional "
            <code>Start</code>" and "<code>End</code>" alignments. The actual placement may shift to keep the "
            "tooltip inside the viewport."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="tooltip-placement-example">
                    <div class="tooltip-placement-example-row">
                        (placement_button("tooltip-top-start"))
                        (placement_button("tooltip-top"))
                        (placement_button("tooltip-top-end"))
                    </div>
                    <div class="tooltip-placement-example-row">
                        (placement_button("tooltip-left-start"))
                        (placement_button("tooltip-right-start"))
                    </div>
                    <div class="tooltip-placement-example-row">
                        (placement_button("tooltip-left"))
                        (placement_button("tooltip-right"))
                    </div>
                    <div class="tooltip-placement-example-row">
                        (placement_button("tooltip-left-end"))
                        (placement_button("tooltip-right-end"))
                    </div>
                    <div class="tooltip-placement-example-row">
                        (placement_button("tooltip-bottom-start"))
                        (placement_button("tooltip-bottom"))
                        (placement_button("tooltip-bottom-end"))
                    </div>
                </div>

                (placement_tooltip("tooltip-top-start", TopStart, "top-start"))
                (placement_tooltip("tooltip-top", Top, "top"))
                (placement_tooltip("tooltip-top-end", TopEnd, "top-end"))
                (placement_tooltip("tooltip-left-start", LeftStart, "left-start"))
                (placement_tooltip("tooltip-right-start", RightStart, "right-start"))
                (placement_tooltip("tooltip-left", Left, "left"))
                (placement_tooltip("tooltip-right", Right, "right"))
                (placement_tooltip("tooltip-left-end", LeftEnd, "left-end"))
                (placement_tooltip("tooltip-right-end", RightEnd, "right-end"))
                (placement_tooltip("tooltip-bottom-start", BottomStart, "bottom-start"))
                (placement_tooltip("tooltip-bottom", Bottom, "bottom"))
                (placement_tooltip("tooltip-bottom-end", BottomEnd, "bottom-end"))
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="tooltip-placement-example-row">
                        <Button appearance=Filled id="tooltip-top-start"/>
                        <Button appearance=Filled id="tooltip-top"/>
                        <Button appearance=Filled id="tooltip-top-end"/>
                    </div>
                    ...

                    <Tooltip anchor_id="tooltip-top-start" placement=TopStart>"top-start"</Tooltip>
                    <Tooltip anchor_id="tooltip-top" placement=Top>"top"</Tooltip>
                    <Tooltip anchor_id="tooltip-top-end" placement=TopEnd>"top-end"</Tooltip>
                    ...
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="triggers" anchor=true>
            "Triggers"
        </Head>
        <p>"The "<code>trigger</code>" property controls how a tooltip is activated. Combine the triggers with "
            <code>"|"</code>" — the default is "<code>"Hover | Focus"</code>", which shows the tooltip on pointer "
            "hover and on keyboard focus. "<code>Click</code>" shows it when the anchor is clicked and dismisses "
            "it on the next click, and "<code>Manual</code>" leaves the tooltip to be shown programmatically. "
            "Use "<code>show_delay</code>" and "<code>hide_delay</code>" to change how long the tooltip waits "
            "before it appears (150 ms by default) and before it goes away (immediately by default)."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button appearance=Filled id="tooltip-toggle-button">"Click to Toggle"</Button>
                <Tooltip anchor_id="tooltip-toggle-button" trigger=Click>"Click again to dismiss"</Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Filled id="toggle-button">"Click to Toggle"</Button>
                    <Tooltip anchor_id="toggle-button" trigger=Click>"Click again to dismiss"</Tooltip>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="html-in-tooltips" anchor=true>
            "HTML in Tooltips"
        </Head>
        <p>"The content of a tooltip is its children, so it can hold presentational markup, "
            "such as emphasis and line breaks."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button appearance=Filled id="tooltip-rich-button">"Hover me"</Button>
                <Tooltip anchor_id="tooltip-rich-button">
                    <div>"This tooltip includes "<strong>"formatted"</strong>" content, such as "
                        <em>"emphasis"</em>" and line breaks."
                    </div>
                </Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Filled id="rich-tooltip">"Hover me"</Button>
                    <Tooltip anchor_id="rich-tooltip">
                        <div>"This tooltip includes "<strong>"formatted"</strong>" content, such as "
                            <em>"emphasis"</em>" and line breaks."
                        </div>
                    </Tooltip>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="customizing" anchor=true>
            "Customizing"
        </Head>
        <p>"Use the "<code>"--max-width"</code>" custom property to set the width at which the tooltip's content "
            "wraps, and the "<code>distance</code>" and "<code>skidding</code>" properties to offset the tooltip "
            "away from and along its anchor."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button appearance=Filled id="tooltip-wrapping-button">"Hover me"</Button>
                <Tooltip anchor_id="tooltip-wrapping-button" style="--max-width: 80px;">
                    "This tooltip will wrap after only 80 pixels."
                </Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Filled id="wrapping-tooltip">"Hover me"</Button>
                    <Tooltip anchor_id="wrapping-tooltip" style="--max-width: 80px;">
                        "This tooltip will wrap after only 80 pixels."
                    </Tooltip>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <p>"Set the "<code>arrow</code>" property to "<code>false</code>" to remove the arrow from a single tooltip. "
           "To resize the arrow of every tooltip, set the "<code>"--wa-tooltip-arrow-size"</code>" design token — "
            <code>0</code>" removes the arrows globally."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Button appearance=Filled id="tooltip-no-arrow-button">"No Arrow"</Button>
                <Tooltip anchor_id="tooltip-no-arrow-button" arrow=false>
                    "This is a tooltip with no arrow"
                </Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Button appearance=Filled id="no-arrow">"No Arrow"</Button>
                    <Tooltip anchor_id="no-arrow" arrow=false>"This is a tooltip with no arrow"</Tooltip>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="showing-and-hiding-manually" anchor=true>
            "Showing & Hiding Manually"
        </Head>
        <p>"Set "<code>"trigger=Manual"</code>" and drive the tooltip yourself with "<code>set_tooltip_open</code>
            " — handy for onboarding hints or for surfacing a tooltip in response to your own logic. Rendering it "
            "with "<code>"open=true"</code>" makes "<code>"init_tooltips"</code>" show it right after the page "
            "settles."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="tooltip-manual-demo">
                    <span id="tooltip-manual-anchor" class=ICON style="font-size: 2em;">
                        (fontawesome::solid::CircleInfo)
                    </span>
                    <Tooltip anchor_id="tooltip-manual-anchor" trigger=Manual>"This is an info icon!"</Tooltip>
                    <Divider/>
                    <Button appearance=Filled class="tooltip-manual-toggle">"Toggle Manually"</Button>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <span id="manual-anchor" class=ICON>(fontawesome::solid::CircleInfo)</span>
                    <Tooltip anchor_id="manual-anchor" trigger=Manual>"This is an info icon!"</Tooltip>
                    <Button appearance=Filled class="tooltip-manual-toggle">"Toggle Manually"</Button>

                    // Toggle the tooltip from the button
                    set_tooltip_open(&tooltip, !is_open(&tooltip), &toggle);
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring for the manual demo: the toggle button shows and hides the
/// tooltip that has no trigger of its own.
pub fn listen_tooltip_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        toggle_manual_tooltip(&event);
    });
}

fn toggle_manual_tooltip(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let toggle = target.closest(".tooltip-manual-toggle").ok()??;
    let demo = toggle.closest(".tooltip-manual-demo").ok()??;
    let tooltip = demo.query_selector(".tooltip").ok()??;

    set_tooltip_open(&tooltip, !is_open(&tooltip), &toggle);

    Some(())
}
