use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Lazy, Renderable, RenderableExt, rsx};
use iconic::{fontawesome, fontawesome_ext};

use crate::attributes::CommonAttributeSetters;
use crate::components::badge::Badge;
use crate::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

#[test]
fn default() {
    let code_example_markup = r#"<div class="code-example"></div>"#;

    let code_example = CodeExample::builder();
    assert_eq!(code_example.render().as_inner(), code_example_markup);

    let code_example = rsx! { <CodeExample></CodeExample> };
    assert_eq!(code_example.render().as_inner(), code_example_markup);

    let code_example_preview_markup = r#"<div class="code-example-preview"></div>"#;

    let code_example_preview = CodeExamplePreview::builder();
    assert_eq!(code_example_preview.render().as_inner(), code_example_preview_markup);

    let code_example_preview = rsx! { <CodeExamplePreview></CodeExamplePreview> };
    assert_eq!(code_example_preview.render().as_inner(), code_example_preview_markup);

    let code_example_source_markup = r#"<div class="code-example-source"><pre></pre></div>"#;

    let code_example_source = CodeExampleSource::builder();
    assert_eq!(code_example_source.render().as_inner(), code_example_source_markup);

    let code_example_source = rsx! { <CodeExampleSource></CodeExampleSource> };
    assert_eq!(code_example_source.render().as_inner(), code_example_source_markup);

    let code_example_button_markup = format!(
        r#"
            <div class="code-example-buttons">
                <button class="code-example-toggle" type="button"> 
                    <span class="icon">{}</span>
                </button>
            </div>
        "#,
        fontawesome_ext::regular::ChevronDown.render().as_inner()
    )
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example_button = CodeExampleButton::builder();
    assert_eq!(code_example_button.render().as_inner(), &code_example_button_markup);

    let code_example_button = rsx! { <CodeExampleButton></CodeExampleButton> };
    assert_eq!(code_example_button.render().as_inner(), &code_example_button_markup);
}

#[test]
fn empty() {
    let code_example_markup = format!(
        r#"
            <div class="code-example">
                <div class="code-example-preview">
                </div>
                <div class="code-example-source">
                    <pre></pre>
                </div>
                <div class="code-example-buttons">
                    <button class="code-example-toggle" type="button"> 
                        <span class="icon">{}</span>
                    </button>
                </div>
            </div>
        "#,
        fontawesome_ext::regular::ChevronDown.render().as_inner()
    )
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = rsx! {
        <CodeExample>
            <CodeExamplePreview>
            </CodeExamplePreview>
            <CodeExampleSource>
            </CodeExampleSource>
            <CodeExampleButton></CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}

#[test]
fn attributes() {
    let code_example_markup = format!(
        r#"
            <div class="code-example open">
                <div id="preview" class="code-example-preview">
                    <div class="code-example-resizer">
                        <span class="icon">{}</span>
                    </div>
                </div>
                <div id="source" class="code-example-source code">
                    <pre id="source-code-block">
                        <button class="button neutral plain copy-button wa-dark" data-action="copy" data-args="{{&quot;from&quot;:&quot;source-code-block&quot;}}">
                            <span class="icon copy-button-copy">{}</span>
                            <span class="icon copy-button-success" hidden>{}</span>
                            <span class="icon copy-button-error" hidden>{}</span>
                        </button>
                    </pre>
                </div>
                <div class="code-example-buttons toggle" style="color: red">
                    <button class="code-example-toggle" type="button"> 
                        <span class="icon">{}</span>
                    </button>
                </div>
            </div>
        "#,
        fontawesome_ext::regular::GripLinesVertical.render().as_inner(),
        fontawesome::regular::Copy.render().as_inner(),
        fontawesome::solid::Check.render().as_inner(),
        fontawesome::solid::Xmark.render().as_inner(),
        fontawesome_ext::regular::ChevronDown.render().as_inner(),
    )
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = CodeExample::builder()
        .open(true)
        .children(Lazy::dangerously_create(|buffer| {
            CodeExamplePreview::builder()
                .resize(true)
                .id("preview")
                .render_to(buffer);
            CodeExampleSource::builder()
                .id("source")
                .class("code")
                .copy_button(true)
                .render_to(buffer);
            CodeExampleButton::builder()
                .class("toggle")
                .style("color: red")
                .render_to(buffer);
        }));
    assert_eq!(code_example.render().as_inner(), &code_example_markup);

    let code_example = rsx! {
        <CodeExample open=true>
            <CodeExamplePreview resize=true id="preview">
            </CodeExamplePreview>
            <CodeExampleSource id="source" class="code" copy_button=true>
            </CodeExampleSource>
            <CodeExampleButton class="toggle" style="color: red">
            </CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}

#[test]
fn children() {
    let code_example_markup = format!(
        r#"
        <div class="code-example">
            <div class="code-example-preview">
                <div class="badge neutral accent">Badge</div>
                <div class="code-example-resizer">
                    <span class="icon">{}</span>
                </div>
            </div>
            <div class="code-example-source">
                <pre>
                    <code class="language-html">
                        &lt;Badge&gt;"Badge"&lt;/Badge&gt;
                    </code>
                </pre>
            </div>
            <div class="code-example-buttons">
                <button class="code-example-toggle" type="button">Code 
                    <span class="icon">{}</span>
                </button>
            </div>
        </div>
    "#,
        fontawesome_ext::regular::GripLinesVertical.render().as_inner(),
        fontawesome_ext::regular::ChevronDown.render().as_inner(),
    )
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let code_example = CodeExample::builder().children(Lazy::dangerously_create(|buffer| {
        CodeExamplePreview::builder()
            .resize(true)
            .children(Lazy::dangerously_create(|buffer| {
                rsx!(<Badge>"Badge"</Badge>).render_to(buffer)
            }))
            .render_to(buffer);
        CodeExampleSource::builder()
            .children(Lazy::dangerously_create(|buffer| {
                rsx! {
                    <code class="language-html">
                        r#"<Badge>"Badge"</Badge>"#
                    </code>
                }
                .render_to(buffer);
            }))
            .render_to(buffer);
        CodeExampleButton::builder()
            .children(Lazy::dangerously_create(|buffer| "Code".render_to(buffer)))
            .render_to(buffer);
    }));
    assert_eq!(code_example.render().as_inner(), &code_example_markup);

    let code_example = rsx! {
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Badge>"Badge"</Badge>
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">
                    r#"<Badge>"Badge"</Badge>"#
                </code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);

    let badge = rsx! { <Badge>"Badge"</Badge> };
    let code = r#"<Badge>"Badge"</Badge>"#;
    let code_example = rsx! {
        <CodeExample>
            <CodeExamplePreview resize=true>
                (badge)
            </CodeExamplePreview>
            <CodeExampleSource>
                <code class="language-html">
                    (code)
                </code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    };
    assert_eq!(code_example.render().as_inner(), &code_example_markup);
}
