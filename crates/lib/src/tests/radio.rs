use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Lazy, Renderable, RenderableExt, rsx};

use crate::attributes::CommonAttributeSetters;
use crate::class::SIZE_SMALL;
use crate::component::radio::Radio;
use crate::component::radio::RadioAppearance::*;
use crate::component::radio_group::RadioGroup;
use crate::orientation::Orientation::*;

#[test]
fn default_radio() {
    let radio_markup = r#"<label class="radio"><input class="control" type="radio"><span class="radio-indicator"></span><span class="label"></span></label>"#;

    let radio = Radio::builder();
    assert_eq!(radio.render().as_inner(), radio_markup);

    let radio = rsx! { <Radio/> };
    assert_eq!(radio.render().as_inner(), radio_markup);

    let radio = rsx! { <Radio></Radio> };
    assert_eq!(radio.render().as_inner(), radio_markup);
}

#[test]
fn radio_states() {
    let radio_markup = r#"<label class="radio"><input class="control" type="radio" name="scheme" value="dark" checked="true" required="true"><span class="radio-indicator"></span><span class="label">Dark</span></label>"#;

    let label = rsx!("Dark");
    let radio = Radio::builder()
        .name("scheme")
        .value("dark")
        .checked(true)
        .required(true)
        .children(&label);
    assert_eq!(radio.render().as_inner(), radio_markup);

    let radio = rsx! { <Radio name="scheme" value="dark" checked=true required=true>"Dark"</Radio> };
    assert_eq!(radio.render().as_inner(), radio_markup);

    let disabled_markup = r#"<label class="radio"><input class="control" type="radio" disabled="true"><span class="radio-indicator"></span><span class="label"></span></label>"#;

    let radio = rsx! { <Radio disabled=true/> };
    assert_eq!(radio.render().as_inner(), disabled_markup);
}

#[test]
fn radio_appearance() {
    let radio_markup = r#"<label class="radio radio-button"><input class="control" type="radio" value="light"><span class="radio-indicator"></span><span class="label">Light</span></label>"#;

    let label = rsx!("Light");
    let radio = Radio::builder().appearance(Button).value("light").children(&label);
    assert_eq!(radio.render().as_inner(), radio_markup);

    let radio = rsx! { <Radio appearance=Button value="light">"Light"</Radio> };
    assert_eq!(radio.render().as_inner(), radio_markup);
}

#[test]
fn default_group() {
    let group_markup = r#"<fieldset class="radio-group vertical" role="radiogroup" aria-orientation="vertical"><div class="radio-group-items"></div></fieldset>"#;

    let group = RadioGroup::builder();
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <RadioGroup/> };
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <RadioGroup></RadioGroup> };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn bare_group() {
    let group_markup =
        r#"<fieldset class="radio-group vertical" role="radiogroup" aria-orientation="vertical"></fieldset>"#;

    let group = RadioGroup::builder().bare(true);
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <RadioGroup bare=true /> };
    assert_eq!(group.render().as_inner(), group_markup);

    let group_markup = r#"<fieldset class="radio-group vertical" role="radiogroup" aria-orientation="vertical"><div class="radio-group-items"><label class="radio"><input class="control" type="radio"><span class="radio-indicator"></span><span class="label"></span></label></div></fieldset>"#;

    let group = rsx! { <RadioGroup bare=true><div class="radio-group-items"><Radio/></div></RadioGroup> };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn group_label_hint_and_states() {
    let group_markup = r#"<fieldset class="radio-group horizontal required size-small" role="radiogroup" aria-orientation="horizontal" disabled="true"><legend class="label">Color scheme</legend><div class="radio-group-items"></div><small class="hint">Choose one</small></fieldset>"#;

    let group = RadioGroup::builder()
        .orientation(Horizontal)
        .required(true)
        .disabled(true)
        .label("Color scheme")
        .hint("Choose one")
        .class(SIZE_SMALL);
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! {
        <RadioGroup
            orientation=Horizontal
            required=true
            disabled=true
            label="Color scheme"
            hint="Choose one"
            class=SIZE_SMALL
        />
    };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn group_radios() {
    let group_markup = r#"<fieldset class="radio-group vertical" role="radiogroup" aria-orientation="vertical"><legend class="label">Color scheme</legend><div class="radio-group-items"><label class="radio"><input class="control" type="radio" name="scheme" value="light"><span class="radio-indicator"></span><span class="label">Light</span></label><label class="radio"><input class="control" type="radio" name="scheme" value="dark" checked="true"><span class="radio-indicator"></span><span class="label">Dark</span></label></div></fieldset>"#;

    let radios = Lazy::dangerously_create(|buffer| {
        let light = rsx!("Light");
        let dark = rsx!("Dark");
        Radio::builder()
            .name("scheme")
            .value("light")
            .children(&light)
            .render_to(buffer);
        Radio::builder()
            .name("scheme")
            .value("dark")
            .checked(true)
            .children(&dark)
            .render_to(buffer);
    });
    let group = RadioGroup::builder().label("Color scheme").children(&radios);
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! {
        <RadioGroup label="Color scheme">
            <Radio name="scheme" value="light">"Light"</Radio>
            <Radio name="scheme" value="dark" checked=true>"Dark"</Radio>
        </RadioGroup>
    };
    assert_eq!(group.render().as_inner(), group_markup);
}
