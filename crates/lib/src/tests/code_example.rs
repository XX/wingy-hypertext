use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Lazy, Renderable, RenderableExt, rsx};

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

    let code_example_button_markup = r#"
        <div class="code-example-buttons">
            <button class="code-example-toggle" type="button" onclick="this.closest('.code-example').classList.toggle('open')"> 
                <span class="icon">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                        <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                    </svg>
                </span>
            </button>
        </div>
    "#
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
    let code_example_markup = r#"
        <div class="code-example">
            <div class="code-example-preview">
            </div>
            <div class="code-example-source">
                <pre></pre>
            </div>
            <div class="code-example-buttons">
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                > 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
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
    let code_example_markup = r#"
        <div class="code-example open">
            <div id="preview" class="code-example-preview">
                <div class="code-example-resizer">
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                            <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                        </svg>
                    </span>
                </div>
            </div>
            <div id="source" class="code-example-source code">
                <pre id="source-code-block">
                    <button class="button neutral plain copy-button wa-dark" onclick="handle_copy(event, 'source-code-block')">
                        <span class="icon copy-button-copy">
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                                <path fill="currentColor" d="M384 336l-192 0c-8.8 0-16-7.2-16-16l0-256c0-8.8 7.2-16 16-16l133.5 0c4.2 0 8.3 1.7 11.3 4.7l58.5 58.5c3 3 4.7 7.1 4.7 11.3L400 320c0 8.8-7.2 16-16 16zM192 384l192 0c35.3 0 64-28.7 64-64l0-197.5c0-17-6.7-33.3-18.7-45.3L370.7 18.7C358.7 6.7 342.5 0 325.5 0L192 0c-35.3 0-64 28.7-64 64l0 256c0 35.3 28.7 64 64 64zM64 128c-35.3 0-64 28.7-64 64L0 448c0 35.3 28.7 64 64 64l192 0c35.3 0 64-28.7 64-64l0-16-48 0 0 16c0 8.8-7.2 16-16 16L64 464c-8.8 0-16-7.2-16-16l0-256c0-8.8 7.2-16 16-16l16 0 0-48-16 0z"/>
                            </svg>
                        </span>
                        <span class="icon copy-button-success" hidden>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                                <path fill="currentColor" d="M434.8 70.1c14.3 10.4 17.5 30.4 7.1 44.7l-256 352c-5.5 7.6-14 12.3-23.4 13.1s-18.5-2.7-25.1-9.3l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l101.5 101.5 234-321.7c10.4-14.3 30.4-17.5 44.7-7.1z"/>
                            </svg>
                        </span>
                        <span class="icon copy-button-error" hidden>
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512">
                                <path fill="currentColor" d="M55.1 73.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L147.2 256 9.9 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192.5 301.3 329.9 438.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.8 256 375.1 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192.5 210.7 55.1 73.4z"/>
                            </svg>
                        </span>
                    </button>
                </pre>
            </div>
            <div class="code-example-buttons toggle" style="color: red">
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                > 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
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
    let code_example_markup = r#"
        <div class="code-example">
            <div class="code-example-preview">
                <div class="badge neutral accent">Badge</div>
                <div class="code-example-resizer">
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                            <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                        </svg>
                    </span>
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
                <button 
                    class="code-example-toggle" 
                    type="button" 
                    onclick="this.closest('.code-example').classList.toggle('open')"
                >Code 
                    <span class="icon">
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        </div>
    "#
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
