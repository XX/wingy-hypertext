use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome;

use crate::attributes::CommonAttributeSetters;
use crate::layouts::drawer::Drawer;
use crate::layouts::drawer::DrawerPlacement::*;

// A zero-width space keeps the header from collapsing when there is no label.
const EMPTY_LABEL: &str = "\u{200B}";

/// The close button icon is rendered from the `iconic` crate; build the
/// expected header markup dynamically so the tests don't hardcode the SVG.
fn header(title: &str) -> String {
    let icon = rsx! { (fontawesome::solid::Xmark) }.render().into_inner();
    format!(
        concat!(
            r#"<header class="drawer-header"><h2 class="drawer-title">{title}</h2>"#,
            r#"<div class="drawer-header-actions">"#,
            r#"<button class="button plain drawer-close" type="button" data-drawer="close" aria-label="Close">"#,
            r#"<span class="icon">{icon}</span></button></div></header>"#,
        ),
        title = title,
        icon = icon,
    )
}

#[test]
fn default() {
    let expected = format!(
        r#"<dialog class="drawer end">{}<div class="drawer-body"></div></dialog>"#,
        header(EMPTY_LABEL)
    );

    let drawer = Drawer::builder();
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer/> };
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer></Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn label() {
    let expected = format!(
        r#"<dialog class="drawer end">{}<div class="drawer-body"></div></dialog>"#,
        header("Drawer")
    );

    let drawer = Drawer::builder().label("Drawer");
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer label="Drawer"/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn placement() {
    for (placement_markup, class) in [("start", "start"), ("bottom", "bottom"), ("top", "top")] {
        let expected = format!(
            r#"<dialog class="drawer {class}">{}<div class="drawer-body"></div></dialog>"#,
            header(EMPTY_LABEL)
        );
        let actual = match placement_markup {
            "start" => rsx! { <Drawer placement=Start/> }.render().into_inner(),
            "bottom" => rsx! { <Drawer placement=Bottom/> }.render().into_inner(),
            _ => rsx! { <Drawer placement=Top/> }.render().into_inner(),
        };
        assert_eq!(actual, expected);
    }

    let expected = format!(
        r#"<dialog class="drawer start">{}<div class="drawer-body"></div></dialog>"#,
        header(EMPTY_LABEL)
    );
    let drawer = Drawer::builder().placement(Start);
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn without_header() {
    let expected = r#"<dialog class="drawer end"><div class="drawer-body"></div></dialog>"#;

    let drawer = Drawer::builder().without_header(true);
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer without_header=true/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn flags() {
    let expected =
        r#"<dialog class="drawer end" data-open="" data-light-dismiss=""><div class="drawer-body"></div></dialog>"#;

    let drawer = rsx! { <Drawer without_header=true open=true light_dismiss=true/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn children() {
    let expected = format!(
        r#"<dialog class="drawer end">{}<div class="drawer-body">Hello, world!</div></dialog>"#,
        header(EMPTY_LABEL)
    );

    let drawer = rsx! { <Drawer>"Hello, world!"</Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn footer_and_header_actions() {
    let footer = rsx! { <button>"Close"</button> };
    let actions = rsx! { <span class="new-window"></span> };
    let drawer = rsx! {
        <Drawer label="Drawer" header_actions=(actions) footer=(footer)>"Body"</Drawer>
    };

    let icon = rsx! { (fontawesome::solid::Xmark) }.render().into_inner();
    let expected = format!(
        concat!(
            r#"<dialog class="drawer end"><header class="drawer-header">"#,
            r#"<h2 class="drawer-title">Drawer</h2><div class="drawer-header-actions">"#,
            r#"<span class="new-window"></span>"#,
            r#"<button class="button plain drawer-close" type="button" data-drawer="close" aria-label="Close">"#,
            r#"<span class="icon">{icon}</span></button></div></header>"#,
            r#"<div class="drawer-body">Body</div>"#,
            r#"<footer class="drawer-footer"><button>Close</button></footer></dialog>"#,
        ),
        icon = icon,
    );
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn additional_attributes() {
    let expected = format!(
        r#"<dialog id="the-drawer" class="drawer end test" style="--size: 50vw">{}<div class="drawer-body"></div></dialog>"#,
        header(EMPTY_LABEL)
    );

    let drawer = Drawer::builder().id("the-drawer").class("test").style("--size: 50vw");
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer id="the-drawer" class="test" style="--size: 50vw"/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}
