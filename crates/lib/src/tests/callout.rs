use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::appearance::AppearanceConstructor;
use crate::attributes::CommonAttributeSetters;
use crate::components::callout::Callout;
use crate::variant::Variant::*;
use crate::variant::VariantConstructor;

#[test]
fn default() {
    let callout_markup = r#"<div class="callout brand filled-outlined"><div class="callout-message"></div></div>"#;

    let callout = Callout::builder();
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout/> };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout></Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);
}

#[test]
fn variant() {
    let callout_markup = r#"<div class="callout success filled-outlined"><div class="callout-message"></div></div>"#;

    let callout = Callout::success();
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = Callout::builder().variant(Success);
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout variant=Success/> };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout variant=Success></Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);
}

#[test]
fn appearance() {
    let callout_markup = r#"<div class="callout brand accent"><div class="callout-message"></div></div>"#;

    let callout = Callout::accent();
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = Callout::builder().appearance(Accent);
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout appearance=Accent/> };
    assert_eq!(callout.render().as_inner(), callout_markup);
}

#[test]
fn children() {
    let callout_markup =
        r#"<div class="callout brand filled-outlined"><div class="callout-message">Hello, world!</div></div>"#;

    let callout = rsx! { <Callout>"Hello, world!"</Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let message = "Hello, world!";
    let callout = rsx! { <Callout>(message)</Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);
}

#[test]
fn icon() {
    let callout_markup = r#"<div class="callout brand filled-outlined"><div class="callout-icon">*</div><div class="callout-message">Hello, world!</div></div>"#;

    let callout = Callout::builder().icon("*").children(&"Hello, world!");
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout icon="*">"Hello, world!"</Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);
}

#[test]
fn bare() {
    let callout_markup = r#"<div class="callout brand filled-outlined"><span>Hello, world!</span></div>"#;

    let children = rsx! {
        <span>"Hello, world!"</span>
    };
    let callout = Callout::builder().bare(true).children(&children);
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout bare=true><span>"Hello, world!"</span></Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout_markup = r#"<div class="callout brand filled-outlined"><div class="callout-icon">*</div><span>Hello, world!</span></div>"#;

    let callout = rsx! { <Callout bare=true icon="*"><span>"Hello, world!"</span></Callout> };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! { <Callout bare=true/> };
    assert_eq!(
        callout.render().as_inner(),
        r#"<div class="callout brand filled-outlined"></div>"#
    );
}

#[test]
fn additional_attributes() {
    let callout_markup = r#"<div id="the-callout" class="callout brand filled-outlined test alarm" style="color: red"><div class="callout-message"></div></div>"#;

    let callout = Callout::builder()
        .id("the-callout")
        .class("test")
        .class("alarm")
        .style("color: red");
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! {
        <Callout id="the-callout" class="test" class="alarm" style="color: red"></Callout>
    };
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout_markup = r#"<div id="note" class="callout danger plain test" style="color: red; background-color: green"><div class="callout-message"></div></div>"#;

    let callout = Callout::plain()
        .variant(Danger)
        .id("note")
        .class("test")
        .style("color: red")
        .style("background-color: green");
    assert_eq!(callout.render().as_inner(), callout_markup);

    let callout = rsx! {
        <Callout
            appearance=Plain
            variant=Danger
            id="note"
            class="test"
            style="color: red; background-color: green"
        >
        </Callout>
    };
    assert_eq!(callout.render().as_inner(), callout_markup);
}
