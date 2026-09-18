use hypertext::{Lazy, Renderable, RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::component::button::Button;
use crate::component::button_group::ButtonGroup;
use crate::orientation::Orientation::*;

#[test]
fn default() {
    let button_group_markup = r#"<div class="button-group" role="group" aria-orientation="horizontal"></div>"#;

    let button_group = ButtonGroup::builder();
    assert_eq!(button_group.render().as_inner(), button_group_markup);

    let button_group = rsx! { <ButtonGroup/> };
    assert_eq!(button_group.render().as_inner(), button_group_markup);

    let button_group = rsx! { <ButtonGroup></ButtonGroup> };
    assert_eq!(button_group.render().as_inner(), button_group_markup);
}

#[test]
fn label_and_orientation() {
    let button_group_markup =
        r#"<div class="button-group vertical" role="group" aria-label="Options" aria-orientation="vertical"></div>"#;

    let button_group = ButtonGroup::builder().label("Options").orientation(Vertical);
    assert_eq!(button_group.render().as_inner(), button_group_markup);

    let button_group = rsx! { <ButtonGroup label="Options" orientation=Vertical/> };
    assert_eq!(button_group.render().as_inner(), button_group_markup);
}

#[test]
fn attributes() {
    let button_group_markup = r#"<div id="toolbar" class="button-group actions" style="gap: 0" role="group" aria-orientation="horizontal"></div>"#;

    let button_group = ButtonGroup::builder().id("toolbar").class("actions").style("gap: 0");
    assert_eq!(button_group.render().as_inner(), button_group_markup);

    let button_group = rsx! { <ButtonGroup id="toolbar" class="actions" style="gap: 0"></ButtonGroup> };
    assert_eq!(button_group.render().as_inner(), button_group_markup);
}

#[test]
fn buttons() {
    let button_group_markup = r#"<div class="button-group" role="group" aria-label="Alignment" aria-orientation="horizontal"><button class="button neutral filled">Left</button><button class="button neutral filled">Right</button></div>"#;

    let buttons = Lazy::dangerously_create(|buffer| {
        let left = rsx!("Left");
        let right = rsx!("Right");
        Button::builder().appearance(Filled).children(&left).render_to(buffer);
        Button::builder().appearance(Filled).children(&right).render_to(buffer);
    });
    let button_group = ButtonGroup::builder().label("Alignment").children(&buttons);
    assert_eq!(button_group.render().as_inner(), button_group_markup);

    let button_group = rsx! {
        <ButtonGroup label="Alignment">
            <Button appearance=Filled>"Left"</Button>
            <Button appearance=Filled>"Right"</Button>
        </ButtonGroup>
    };
    assert_eq!(button_group.render().as_inner(), button_group_markup);
}
