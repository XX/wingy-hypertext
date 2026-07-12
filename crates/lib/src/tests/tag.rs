use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::components::tag::Tag;
use crate::variant::Variant::*;

#[test]
fn default() {
    let tag_markup = r#"<span class="tag neutral filled-outlined"><span class="tag-content"></span></span>"#;

    let tag = Tag::builder();
    assert_eq!(tag.render().as_inner(), tag_markup);

    let tag = rsx! { <Tag/> };
    assert_eq!(tag.render().as_inner(), tag_markup);
}

#[test]
fn variant_and_appearance() {
    let tag_markup = r#"<span class="tag success accent"><span class="tag-content">Done</span></span>"#;

    let tag = rsx! { <Tag variant=Success appearance=Accent>"Done"</Tag> };
    assert_eq!(tag.render().as_inner(), tag_markup);
}

#[test]
fn pill() {
    let tag_markup = r#"<span class="tag pill neutral filled-outlined"><span class="tag-content">Pill</span></span>"#;

    let tag = rsx! { <Tag pill=true>"Pill"</Tag> };
    assert_eq!(tag.render().as_inner(), tag_markup);
}

#[test]
fn with_remove() {
    let tag = rsx! { <Tag with_remove=true>"Removable"</Tag> };
    let markup = tag.render();
    let markup = markup.as_inner();

    assert!(markup.contains(r#"<span class="tag-content">Removable</span>"#));
    assert!(markup.contains(r#"<button class="tag-remove" type="button" tabindex="-1" aria-label="Remove">"#));
}

#[test]
fn additional_attributes() {
    let tag_markup = r#"<span id="the-tag" class="tag danger filled test" style="color: red"><span class="tag-content"></span></span>"#;

    let tag = Tag::builder()
        .variant(Danger)
        .appearance(Filled)
        .id("the-tag")
        .class("test")
        .style("color: red");
    assert_eq!(tag.render().as_inner(), tag_markup);
}
