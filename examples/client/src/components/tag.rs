use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::Event;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{
    CLUSTER, GAP_2XS, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM, SIZE_SMALL,
};
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::tag::Tag;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Tag"</Head>
        <p>"Tags label, categorize, or represent selections with a compact visual marker. "
            "Use them for status indicators, filters, or removable chips."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Tag>"Tag"</Tag>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<Tag>"Tag"</Tag>"#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="variants" anchor=true>
            "Variants"
        </Head>
        <p>"Use the "<code>variant</code>" attribute to set the tag's semantic variant."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Tag variant=Neutral>"Neutral"</Tag>
                    <Tag variant=Brand>"Brand"</Tag>
                    <Tag variant=Success>"Success"</Tag>
                    <Tag variant=Warning>"Warning"</Tag>
                    <Tag variant=Danger>"Danger"</Tag>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Tag variant=Neutral>"Neutral"</Tag>
                    <Tag variant=Brand>"Brand"</Tag>
                    <Tag variant=Success>"Success"</Tag>
                    <Tag variant=Warning>"Warning"</Tag>
                    <Tag variant=Danger>"Danger"</Tag>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" attribute to change the tag's visual appearance. The default is "
            <code>"filled-outlined"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS) style="margin-block-end: 1rem;">
                    <Tag variant=Neutral appearance=Accent>"Accent"</Tag>
                    <Tag variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Tag>
                    <Tag variant=Neutral appearance=Filled>"Filled"</Tag>
                    <Tag variant=Neutral appearance=Outlined>"Outlined"</Tag>
                </div>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Tag variant=Brand appearance=Accent>"Accent"</Tag>
                    <Tag variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Tag>
                    <Tag variant=Brand appearance=Filled>"Filled"</Tag>
                    <Tag variant=Brand appearance=Outlined>"Outlined"</Tag>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=(CLUSTER, " ", GAP_2XS)>
                        <Tag variant=Neutral appearance=Accent>"Accent"</Tag>
                        <Tag variant=Neutral appearance=FilledOutlined>"Filled-Outlined"</Tag>
                        <Tag variant=Neutral appearance=Filled>"Filled"</Tag>
                        <Tag variant=Neutral appearance=Outlined>"Outlined"</Tag>
                    </div>
                    <div class=(CLUSTER, " ", GAP_2XS)>
                        <Tag variant=Brand appearance=Accent>"Accent"</Tag>
                        <Tag variant=Brand appearance=FilledOutlined>"Filled-Outlined"</Tag>
                        <Tag variant=Brand appearance=Filled>"Filled"</Tag>
                        <Tag variant=Brand appearance=Outlined>"Outlined"</Tag>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Tags are sized relative to the current font size. You can set the corresponding "
            <code>"size-*"</code>" class on any tag (or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Tag variant=Brand class=SIZE_EXTRA_SMALL>"Extra Small"</Tag>
                    <Tag variant=Brand class=SIZE_SMALL>"Small"</Tag>
                    <Tag variant=Brand class=SIZE_MEDIUM>"Medium"</Tag>
                    <Tag variant=Brand class=SIZE_LARGE>"Large"</Tag>
                    <Tag variant=Brand class=SIZE_EXTRA_LARGE>"Extra Large"</Tag>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Tag variant=Brand class=SIZE_EXTRA_SMALL>"Extra Small"</Tag>
                    <Tag variant=Brand class=SIZE_SMALL>"Small"</Tag>
                    <Tag variant=Brand class=SIZE_MEDIUM>"Medium"</Tag>
                    <Tag variant=Brand class=SIZE_LARGE>"Large"</Tag>
                    <Tag variant=Brand class=SIZE_EXTRA_LARGE>"Extra Large"</Tag>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill" anchor=true>
            "Pill"
        </Head>
        <p>"Use the "<code>pill</code>" attribute to give tags rounded edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS)>
                    <Tag variant=Neutral pill=true>"Neutral"</Tag>
                    <Tag variant=Brand pill=true>"Brand"</Tag>
                    <Tag variant=Success pill=true>"Success"</Tag>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Tag variant=Neutral pill=true>"Neutral"</Tag>
                    <Tag variant=Brand pill=true>"Brand"</Tag>
                    <Tag variant=Success pill=true>"Success"</Tag>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="removable" anchor=true>
            "Removable"
        </Head>
        <p>"Use the "<code>with_remove</code>" attribute to add a remove button to the tag. "
            "Activating it emits a bubbling "<code>"wa-remove"</code>
            " event on the tag so you can handle the removal — the tags below fade out and come back after two seconds."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_2XS, " ", "tags-removable")>
                    <Tag with_remove=true class=SIZE_SMALL style="transition: opacity var(--wa-transition-normal);">
                        "Small"
                    </Tag>
                    <Tag with_remove=true class=SIZE_MEDIUM style="transition: opacity var(--wa-transition-normal);">
                        "Medium"
                    </Tag>
                    <Tag with_remove=true class=SIZE_LARGE style="transition: opacity var(--wa-transition-normal);">
                        "Large"
                    </Tag>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class="tags-removable">
                        <Tag with_remove=true class=SIZE_SMALL>"Small"</Tag>
                        <Tag with_remove=true class=SIZE_MEDIUM>"Medium"</Tag>
                        <Tag with_remove=true class=SIZE_LARGE>"Large"</Tag>
                    </div>

                    // The removal is handled by a "wa-remove" listener,
                    // see `listen_tag_removable_demo` in examples/client/src/components/tag.rs
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Removable demo wiring
//

/// One-time wiring: tags inside `.tags-removable` fade out on `wa-remove`
/// and come back after two seconds, like in the Web Awesome docs example.
pub fn listen_tag_removable_demo() {
    let document = dom::existing::document();

    document.add_steady_event_listener("wa-remove", |event| {
        handle_demo_remove(&event);
    });
}

fn handle_demo_remove(event: &Event) -> Option<()> {
    let tag = event.target()?.maybe_into_element()?;
    tag.closest(".tags-removable").ok()??;

    let tag = tag.maybe_into_html()?;
    tag.style().set_property("opacity", "0").ok();

    let restored = tag.clone();
    dom::set_timeout(
        move || {
            restored.style().set_property("opacity", "1").ok();
        },
        2000,
    )
    .ok();

    Some(())
}
