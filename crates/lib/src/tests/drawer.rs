use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome;

use crate::attributes::CommonAttributeSetters;
use crate::class::{BOTTOM, START, TOP};
use crate::layouts::INVISIBLE;
use crate::layouts::drawer::DrawerPlacement::*;
use crate::layouts::drawer::{Drawer, DrawerBody, DrawerFooter, DrawerHeader};

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
    let expected = r#"<dialog class="drawer start"></dialog>"#;

    let drawer = Drawer::builder();
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer/> };
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer></Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn title() {
    let expected = format!(r#"<dialog class="drawer start">{}</dialog>"#, header("Drawer"));

    let drawer_header = DrawerHeader::builder().children(&"Drawer");
    let drawer = Drawer::builder().children(&drawer_header);
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer><DrawerHeader>"Drawer"</DrawerHeader></Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn placement() {
    for placement_class in [START, BOTTOM, TOP] {
        let expected = format!(r#"<dialog class="drawer {placement_class}"></dialog>"#);
        let actual = match placement_class {
            START => rsx! { <Drawer placement=Start/> }.render().into_inner(),
            BOTTOM => rsx! { <Drawer placement=Bottom/> }.render().into_inner(),
            TOP => rsx! { <Drawer placement=Top/> }.render().into_inner(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    let expected = r#"<dialog class="drawer start"></dialog>"#;
    let drawer = Drawer::builder().placement(Start);
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn with_empty_body() {
    let expected = r#"<dialog class="drawer start"><div class="drawer-body"></div></dialog>"#;

    let drawer_body = DrawerBody::builder();
    let drawer = Drawer::builder().children(&drawer_body);
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer><DrawerBody/></Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn flags() {
    let expected = r#"<dialog class="drawer start" data-open="" data-light-dismiss=""></dialog>"#;

    let drawer = rsx! { <Drawer open=true light_dismiss=true/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn header_and_body() {
    let expected = format!(
        r#"<dialog class="drawer start">{}<div class="drawer-body">Hello, world!</div></dialog>"#,
        header(INVISIBLE)
    );

    let drawer = rsx! { <Drawer><DrawerHeader/><DrawerBody>"Hello, world!"</DrawerBody></Drawer> };
    assert_eq!(drawer.render().as_inner(), &expected);
}

#[test]
fn footer_and_header_actions() {
    let drawer = rsx! {
        <Drawer>
            <DrawerHeader actions=(rsx! { <span class="new-window"></span> })>
                "Drawer"
            </DrawerHeader>
            <DrawerBody>"Body"</DrawerBody>
            <DrawerFooter>
                <button>"Close"</button>
            </DrawerFooter>
        </Drawer>
    };

    let icon = fontawesome::solid::Xmark.render().into_inner();
    let expected = format!(
        concat!(
            r#"<dialog class="drawer start"><header class="drawer-header">"#,
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
    let expected = r#"<dialog id="the-drawer" class="drawer start test" style="--size: 50vw"></dialog>"#;

    let drawer = Drawer::builder().id("the-drawer").class("test").style("--size: 50vw");
    assert_eq!(drawer.render().as_inner(), &expected);

    let drawer = rsx! { <Drawer id="the-drawer" class="test" style="--size: 50vw"/> };
    assert_eq!(drawer.render().as_inner(), &expected);
}
