use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::class::SIZE_SMALL;
use crate::component::textarea::Resize::*;
use crate::component::textarea::{Textarea, TextareaParamBuilder};

#[test]
fn default() {
    let textarea_markup = r#"<div class="textarea outlined resize-vertical"><div class="textarea-field"><textarea class="control" rows="4"></textarea></div></div>"#;

    let textarea = Textarea::builder();
    assert_eq!(textarea.render().as_inner(), textarea_markup);

    let textarea = rsx! { <Textarea/> };
    assert_eq!(textarea.render().as_inner(), textarea_markup);
}

/// Unlike an `<input>`, a textarea carries its value as content.
#[test]
fn value_is_the_content() {
    let textarea_markup = r#"<div class="textarea outlined resize-vertical"><div class="textarea-field"><textarea class="control" name="comment" rows="4">Hello</textarea></div></div>"#;

    let textarea = Textarea::builder().name("comment").value("Hello");
    assert_eq!(textarea.render().as_inner(), textarea_markup);

    // The content is escaped like any other text
    let textarea = Textarea::builder().value("<b>&</b>").render();
    assert!(
        textarea
            .as_inner()
            .contains(r#"<textarea class="control" rows="4">&lt;b&gt;&amp;&lt;/b&gt;</textarea>"#)
    );
}

#[test]
fn states_and_appearance() {
    let textarea = Textarea::builder()
        .appearance(Filled)
        .resize(Auto)
        .class(SIZE_SMALL)
        .required(true)
        .readonly(true)
        .disabled(true)
        .placeholder("Tell us what you think")
        .minlength(5)
        .maxlength(20)
        .rows(6)
        .render();
    let markup = textarea.as_inner();

    assert!(markup.starts_with(r#"<div class="textarea required filled resize-auto size-small">"#));
    assert!(markup.contains(r#"rows="6" placeholder="Tell us what you think" minlength="5" maxlength="20""#));
    assert!(markup.contains(r#"disabled="true" readonly="true" required="true""#));
}

#[test]
fn label_and_hint() {
    let textarea = Textarea::builder().label("Comments").hint("Keep it short").render();
    let markup = textarea.as_inner();

    assert!(markup.contains(r#"<label class="label">Comments</label>"#));
    assert!(markup.contains(r#"<div class="textarea-footer"><small class="hint">Keep it short</small></div>"#));
}

/// The count starts from the rendered value and switches to the remaining
/// characters once the field carries a `maxlength`.
#[test]
fn character_count() {
    let textarea = Textarea::builder().count(true).value("Hello").render();
    assert!(
        textarea
            .as_inner()
            .contains(r#"<small class="textarea-count">5 characters</small>"#)
    );

    let textarea = Textarea::builder().count(true).value("Hello").maxlength(20).render();
    assert!(
        textarea
            .as_inner()
            .contains(r#"<small class="textarea-count">15 characters left</small>"#)
    );

    // The length is counted in UTF-16 code units, as the browser counts it for
    // `maxlength`: an emoji outside the basic plane takes two of them
    let textarea = Textarea::builder().count(true).value("🙂").render();
    assert!(
        textarea
            .as_inner()
            .contains(r#"<small class="textarea-count">2 characters</small>"#)
    );

    // Without the count there is no footer at all
    let textarea = Textarea::builder().value("Hello").render();
    assert!(!textarea.as_inner().contains("textarea-footer"));
}
