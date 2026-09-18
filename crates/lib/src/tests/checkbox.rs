use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Lazy, Renderable, RenderableExt, rsx};
use iconic::fontawesome;

use crate::attributes::CommonAttributeSetters;
use crate::class::SIZE_SMALL;
use crate::component::checkbox::Checkbox;
use crate::component::checkbox_group::CheckboxGroup;
use crate::orientation::Orientation::*;

fn indicator_markup() -> String {
    format!(
        r#"<span class="checkbox-indicator"><span class="checkbox-check">{}</span><span class="checkbox-indeterminate">{}</span></span>"#,
        fontawesome::solid::Check.render().as_inner(),
        fontawesome::solid::Minus.render().as_inner(),
    )
}

#[test]
fn default_checkbox() {
    let checkbox_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox">{}<span class="label"></span></label></div>"#,
        indicator_markup()
    );

    let checkbox = Checkbox::builder();
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checkbox = rsx! { <Checkbox/> };
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checkbox = rsx! { <Checkbox></Checkbox> };
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);
}

#[test]
fn checkbox_states() {
    let checkbox_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" name="terms" value="accepted" checked="true" required="true">{}<span class="label">I agree</span></label></div>"#,
        indicator_markup()
    );

    let label = rsx!("I agree");
    let checkbox = Checkbox::builder()
        .name("terms")
        .value("accepted")
        .checked(true)
        .required(true)
        .children(&label);
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checkbox = rsx! { <Checkbox name="terms" value="accepted" checked=true required=true>"I agree"</Checkbox> };
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let disabled_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" disabled="true">{}<span class="label"></span></label></div>"#,
        indicator_markup()
    );

    let checkbox = rsx! { <Checkbox disabled=true/> };
    assert_eq!(checkbox.render().as_inner(), &disabled_markup);
}

/// The indeterminate state is a DOM property, so the markup only carries the
/// marker `init_checkboxes` reads — and a checked checkbox never gets it.
#[test]
fn checkbox_indeterminate() {
    let checkbox_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" data-indeterminate="">{}<span class="label"></span></label></div>"#,
        indicator_markup()
    );

    let checkbox = Checkbox::builder().indeterminate(true);
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checkbox = rsx! { <Checkbox indeterminate=true/> };
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checked_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" checked="true">{}<span class="label"></span></label></div>"#,
        indicator_markup()
    );

    let checkbox = rsx! { <Checkbox indeterminate=true checked=true/> };
    assert_eq!(checkbox.render().as_inner(), &checked_markup);
}

#[test]
fn checkbox_hint() {
    let checkbox_markup = format!(
        r#"<div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox">{}<span class="label">Weekly digest</span></label><small class="hint">Sent every Monday</small></div>"#,
        indicator_markup()
    );

    let label = rsx!("Weekly digest");
    let checkbox = Checkbox::builder().hint("Sent every Monday").children(&label);
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);

    let checkbox = rsx! { <Checkbox hint="Sent every Monday">"Weekly digest"</Checkbox> };
    assert_eq!(checkbox.render().as_inner(), &checkbox_markup);
}

#[test]
fn default_group() {
    let group_markup = r#"<fieldset class="checkbox-group vertical" role="group" aria-orientation="vertical"><div class="checkbox-group-items"></div></fieldset>"#;

    let group = CheckboxGroup::builder();
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <CheckboxGroup/> };
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <CheckboxGroup></CheckboxGroup> };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn group_label_hint_and_states() {
    let group_markup = r#"<fieldset class="checkbox-group horizontal required size-small" role="group" aria-orientation="horizontal" disabled="true"><legend class="label">Notifications</legend><div class="checkbox-group-items"></div><small class="hint">Choose what you want to hear about</small></fieldset>"#;

    let group = CheckboxGroup::builder()
        .orientation(Horizontal)
        .required(true)
        .disabled(true)
        .label("Notifications")
        .hint("Choose what you want to hear about")
        .class(SIZE_SMALL);
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! {
        <CheckboxGroup
            orientation=Horizontal
            required=true
            disabled=true
            label="Notifications"
            hint="Choose what you want to hear about"
            class=SIZE_SMALL
        />
    };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn bare_group() {
    let group_markup = r#"<fieldset class="checkbox-group vertical" role="group" aria-orientation="vertical"><div class="checkbox-group-items"></div></fieldset>"#;

    let items = rsx! { <div class="checkbox-group-items"></div> };
    let group = CheckboxGroup::builder().bare(true).children(&items);
    assert_eq!(group.render().as_inner(), group_markup);

    let group = rsx! { <CheckboxGroup bare=true><div class="checkbox-group-items"></div></CheckboxGroup> };
    assert_eq!(group.render().as_inner(), group_markup);
}

#[test]
fn group_checkboxes() {
    let group_markup = format!(
        r#"<fieldset class="checkbox-group vertical" role="group" aria-orientation="vertical"><legend class="label">Notifications</legend><div class="checkbox-group-items"><div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" name="news" value="weekly" checked="true">{indicator}<span class="label">Weekly digest</span></label></div><div class="checkbox"><label class="checkbox-toggle"><input class="control" type="checkbox" name="news" value="offers">{indicator}<span class="label">Offers</span></label></div></div></fieldset>"#,
        indicator = indicator_markup()
    );

    let checkboxes = Lazy::dangerously_create(|buffer| {
        let weekly = rsx!("Weekly digest");
        let offers = rsx!("Offers");
        Checkbox::builder()
            .name("news")
            .value("weekly")
            .checked(true)
            .children(&weekly)
            .render_to(buffer);
        Checkbox::builder()
            .name("news")
            .value("offers")
            .children(&offers)
            .render_to(buffer);
    });
    let group = CheckboxGroup::builder().label("Notifications").children(&checkboxes);
    assert_eq!(group.render().as_inner(), &group_markup);

    let group = rsx! {
        <CheckboxGroup label="Notifications">
            <Checkbox name="news" value="weekly" checked=true>"Weekly digest"</Checkbox>
            <Checkbox name="news" value="offers">"Offers"</Checkbox>
        </CheckboxGroup>
    };
    assert_eq!(group.render().as_inner(), &group_markup);
}
