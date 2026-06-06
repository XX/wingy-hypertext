use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::components::input::Input;
use crate::components::input::InputType::*;

#[test]
fn default() {
    let input_markup =
        r#"<div class="input accent"><div class="text-field"><input class="control" type="text"></div></div>"#;

    let input = Input::builder();
    assert_eq!(input.render().as_inner(), input_markup);

    let input = rsx! { <Input /> };
    assert_eq!(input.render().as_inner(), input_markup);
}

#[test]
fn input_type() {
    let input_markup =
        r#"<div class="input accent"><div class="text-field"><input class="control" type="email"></div></div>"#;

    let input = Input::builder().input_type(Email);
    assert_eq!(input.render().as_inner(), input_markup);

    let input = rsx! { <Input input_type=Email /> };
    assert_eq!(input.render().as_inner(), input_markup);
}

#[test]
fn label_and_hint() {
    let input_markup = r#"<div class="input accent"><label class="label">Name</label><div class="text-field"><input class="control" type="text" placeholder="Enter your name"></div><small class="hint">What should we call you?</small></div>"#;

    let input = Input::builder()
        .label("Name")
        .hint("What should we call you?")
        .placeholder("Enter your name");
    assert_eq!(input.render().as_inner(), input_markup);
}

#[test]
fn states() {
    let input_markup = r#"<div class="input required outlined"><div class="text-field"><input class="control" type="text" disabled="true" readonly="true" required="true"></div></div>"#;

    let input = Input::builder()
        .appearance(Outlined)
        .disabled(true)
        .readonly(true)
        .required(true);
    assert_eq!(input.render().as_inner(), input_markup);
}

#[test]
fn additional_attributes() {
    let input_markup = r#"<div id="email" class="input accent test" style="color: red"><div class="text-field"><input class="control" type="email" name="email" value="a@b.c"></div></div>"#;

    let input = Input::builder()
        .input_type(Email)
        .name("email")
        .value("a@b.c")
        .id("email")
        .class("test")
        .style("color: red");
    assert_eq!(input.render().as_inner(), input_markup);
}
