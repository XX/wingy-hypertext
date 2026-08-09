use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::appearance::AppearanceConstructor;
use crate::attributes::CommonAttributeSetters;
use crate::component::badge::Badge;
use crate::variant::Variant::*;
use crate::variant::VariantConstructor;

#[test]
fn default() {
    let badge_markup = r#"<div class="badge neutral accent"></div>"#;

    let badge = Badge::builder();
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge/> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn variant() {
    let badge_markup = r#"<div class="badge success accent"></div>"#;

    let badge = Badge::success();
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = Badge::builder().variant(Success);
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge variant=Success/> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! { <Badge variant=Success></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn children() {
    let badge_markup = r#"<div class="badge neutral accent">Hello, world!</div>"#;

    let badge = rsx! { <Badge>"Hello, world!"</Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let label = "Hello, world!";
    let badge = rsx! { <Badge>(label)</Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn nested() {
    let badge_markup =
        r#"<div class="badge neutral accent"><div class="badge success accent">Hello, world!</div></div>"#;

    let badge = rsx! { <Badge><Badge variant=Success>"Hello, world!"</Badge></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let label = "Hello, world!";
    let badge = rsx! { <Badge><Badge variant=Success>(label)</Badge></Badge> };
    assert_eq!(badge.render().as_inner(), badge_markup);
}

#[test]
fn additional_attributes() {
    let badge_markup = r#"<div id="the-badge" class="badge neutral accent test alarm" style="color: red"></div>"#;

    let badge = Badge::builder()
        .id("the-badge")
        .class("test")
        .class("alarm")
        .style("color: red");
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! {
        <Badge id="the-badge" class="test" class="alarm" style="color: red"></Badge>
    };
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge_markup =
        r#"<div id="bad" class="badge danger filled-outlined test" style="color: red; background-color: green"></div>"#;

    let badge = Badge::filled_outlined()
        .variant(Danger)
        .id("bad")
        .class("test")
        .style("color: red")
        .style("background-color: green");
    assert_eq!(badge.render().as_inner(), badge_markup);

    let badge = rsx! {
        <Badge
            appearance=FilledOutlined
            variant=Danger
            id="bad"
            class="test"
            style="color: red"
            style="background-color: green"
        >
        </Badge>
    };
    assert_eq!(badge.render().as_inner(), badge_markup);
}
