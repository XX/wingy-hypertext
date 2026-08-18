use hypertext::{RenderableExt, rsx};

use crate::action::ActionSetters;
use crate::appearance::Appearance::*;
use crate::appearance::AppearanceConstructor;
use crate::attributes::CommonAttributeSetters;
use crate::component::button::Button;
use crate::htmx::{Htmx, HtmxSetters};
use crate::link::LinkSetters;
use crate::variant::Variant::*;
use crate::variant::VariantConstructor;

#[test]
fn default() {
    let button_markup = r#"<button class="button neutral accent"></button>"#;

    let button = Button::builder();
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! { <Button/> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! { <Button></Button> };
    assert_eq!(button.render().as_inner(), button_markup);
}

#[test]
fn variant() {
    let button_markup = r#"<button class="button success accent"></button>"#;

    let button = Button::success();
    assert_eq!(button.render().as_inner(), button_markup);

    let button = Button::builder().variant(Success);
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! { <Button variant=Success/> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! { <Button variant=Success></Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_link_markup = r##"<a class="button success accent" href="#"></a>"##;

    let button_link = Button::success().href("#");
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let button_link = Button::builder().href("#").variant(Success);
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let button_link = rsx! { <Button variant=Success href="#"/> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let button_link = rsx! { <Button variant=Success href="#"></Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);
}

#[test]
fn children() {
    let button_markup = r#"<button class="button neutral accent">Hello, world!</button>"#;

    let button = rsx! { <Button>"Hello, world!"</Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let label = "Hello, world!";
    let button = rsx! { <Button>(label)</Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_link_markup = r#"<a class="button neutral accent" href="">Hello, world!</a>"#;

    let button_link = rsx! { <Button href="">"Hello, world!"</Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let label = "Hello, world!";
    let button_link = rsx! { <Button href="">(label)</Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);
}

#[test]
fn nested() {
    let button_markup = r#"<button class="button neutral accent"><button class="button success accent">Hello, world!</button></button>"#;

    let button = rsx! { <Button><Button variant=Success>"Hello, world!"</Button></Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let label = "Hello, world!";
    let button = rsx! { <Button><Button variant=Success>(label)</Button></Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_link_markup =
        r#"<a class="button neutral accent" href=""><button class="button success accent">Hello, world!</button></a>"#;

    let button_link = rsx! { <Button href=""><Button variant=Success>"Hello, world!"</Button></Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let label = "Hello, world!";
    let button_link = rsx! { <Button href=""><Button variant=Success>(label)</Button></Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);
}

#[test]
fn additional_attributes() {
    let button_markup =
        r#"<button id="the-button" class="button neutral accent test alarm" style="color: red"></button>"#;

    let button = Button::builder()
        .id("the-button")
        .class("test")
        .class("alarm")
        .style("color: red");
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! {
        <Button id="the-button" class="test" class="alarm" style="color: red"></Button>
    };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_markup = r#"<button id="bad" class="button danger filled-outlined test" style="color: red; background-color: green"></button>"#;

    let button = Button::filled_outlined()
        .variant(Danger)
        .id("bad")
        .class("test")
        .style("color: red")
        .style("background-color: green");
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! {
        <Button
            appearance=FilledOutlined
            variant=Danger
            id="bad"
            class="test"
            style="color: red"
            style="background-color: green"
        >
        </Button>
    };
    assert_eq!(button.render().as_inner(), button_markup);
}

#[test]
fn htmx() {
    // Attributes render in the canonical order (hx-get, hx-swap, hx-target), not insertion order.
    let button_markup =
        r##"<button class="button neutral accent" hx-get="/items" hx-swap="innerHTML" hx-target="#list"></button>"##;

    let button = Button::builder()
        .hx_target("#list")
        .hx_get("/items")
        .hx_swap("innerHTML");
    assert_eq!(button.render().as_inner(), button_markup);

    let button = rsx! {
        <Button htmx=(Htmx::new().hx_get("/items").hx_target("#list").hx_swap("innerHTML")) />
    };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_markup = r##"<button class="button neutral accent" hx-get="/items" hx-swap="innerHTML" hx-target="#list">Load</button>"##;

    let button = rsx! {
        <Button hx_get="/items" hx_target="#list" hx_swap="innerHTML">"Load"</Button>
    };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_link_markup = r#"<a class="button neutral accent" href="/items" hx-boost="true">Load</a>"#;

    let button_link = rsx! { <Button href="/items" hx_boost="true">"Load"</Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);
}

#[test]
fn action() {
    let button_markup = r#"<button class="button neutral accent" data-action="alert" data-action-args="{&quot;msg&quot;:&quot;Hello&quot;}">Hello</button>"#;

    let button = rsx! { <Button action="alert" action_args=r#"{"msg":"Hello"}"#>"Hello"</Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let label = "Hello";
    let button = rsx! { <Button action="alert" action_args=r#"{"msg":"Hello"}"#>(label)</Button> };
    assert_eq!(button.render().as_inner(), button_markup);

    let button_link_markup = r#"<a class="button neutral accent" href="" data-action="alert" data-action-args="{&quot;msg&quot;:&quot;Hello&quot;}">Hello</a>"#;

    let button_link = rsx! { <Button href="" action="alert" action_args=r#"{"msg":"Hello"}"#>"Hello"</Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);

    let label = "Hello";
    let button_link = rsx! { <Button href="" action="alert" action_args=r#"{"msg":"Hello"}"#>(label)</Button> };
    assert_eq!(button_link.render().as_inner(), button_link_markup);
}
