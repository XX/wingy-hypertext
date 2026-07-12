use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{RenderableExt, rsx};

use crate::attributes::CommonAttributeSetters;
use crate::class::HINT;
use crate::components::switch::{Switch, Toggle};

const TRACK_MARKUP: &str = r#"<span class="track"><span class="thumb"></span></span>"#;

#[test]
fn default() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder();
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch></Switch> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn label() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch">{TRACK_MARKUP}<span class="label">Enable notifications</span></label></div>"#
    );

    let switch = rsx! { <Switch>"Enable notifications"</Switch> };
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let label = "Enable notifications";
    let switch = rsx! { <Switch>(label)</Switch> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn hint() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch">{TRACK_MARKUP}<span class="label">Email me about new releases</span></label><small class="hint">You can change this at any time.</small></div>"#
    );

    let switch = rsx! { <Switch hint="You can change this at any time.">"Email me about new releases"</Switch> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn states() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch" checked="true">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder().checked(true);
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch checked=true/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch_markup = format!(
        r#"<div class="switch disabled"><label class="switch-toggle"><input class="control" type="checkbox" role="switch" disabled="true">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder().disabled(true);
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch disabled=true/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch_markup = format!(
        r#"<div class="switch required"><label class="switch-toggle"><input class="control" type="checkbox" role="switch" required="true">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder().required(true);
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch required=true/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn name_and_value() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch" name="notifications" value="enabled" checked="true">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder().name("notifications").value("enabled").checked(true);
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch name="notifications" value="enabled" checked=true/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn additional_attributes() {
    let switch_markup = format!(
        r#"<div id="the-switch" class="switch test" style="--width: 80px"><label class="switch-toggle"><input class="control" type="checkbox" role="switch">{TRACK_MARKUP}<span class="label"></span></label></div>"#
    );

    let switch = Switch::builder().id("the-switch").class("test").style("--width: 80px");
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch = rsx! { <Switch id="the-switch" class="test" style="--width: 80px"/> };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}

#[test]
fn toggle() {
    let toggle_markup = format!(
        r#"<label class="switch-toggle"><input class="control" type="checkbox" role="switch" name="releases" checked="true">{TRACK_MARKUP}<span class="label">Email me about new releases</span></label>"#
    );

    let toggle = rsx! { <Toggle name="releases" checked=true>"Email me about new releases"</Toggle> };
    assert_eq!(toggle.render().as_inner(), &toggle_markup);
}

#[test]
fn bare() {
    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch">{TRACK_MARKUP}<span class="label">Email me about new releases</span></label><small class="hint">You can change this <strong>at any time</strong> in settings.</small></div>"#
    );

    let switch = rsx! {
        <Switch bare=true>
            <Toggle>
                "Email me about new releases"
            </Toggle>
            <small class=HINT>"You can change this "<strong>"at any time"</strong>" in settings."</small>
        </Switch>
    };
    assert_eq!(switch.render().as_inner(), &switch_markup);

    let switch_markup = format!(
        r#"<div class="switch"><label class="switch-toggle"><input class="control" type="checkbox" role="switch" name="releases" checked="true">{TRACK_MARKUP}<span class="label">Email me about new releases</span></label></div>"#
    );

    let switch = rsx! {
        <Switch bare=true>
            <Toggle name="releases" checked=true>
                "Email me about new releases"
            </Toggle>
        </Switch>
    };
    assert_eq!(switch.render().as_inner(), &switch_markup);
}
