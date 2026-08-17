use hypertext::{RenderableExt, rsx};

use crate::attributes::CommonAttributeSetters;
use crate::component::tooltip::TooltipTrigger::*;
use crate::component::tooltip::{Tooltip, TooltipTriggers};
use crate::helper::popup::PopupPlacement::RightStart;

// The positioning options every tooltip is rendered with, after the placement,
// the optional anchor, the distance and the optional skidding.
const POPUP_OPTIONS: &str = r#"data-flip="" data-shift="" data-shift-padding="8""#;

// The tooltip host (`host_attrs`) wrapping the popup that anchors the body.
fn tooltip(host_attrs: &str, popup_attrs: &str, body: &str) -> String {
    format!(
        r#"<div {host_attrs}><div class="popup" {popup_attrs} {POPUP_OPTIONS}>"#,
        host_attrs = host_attrs,
        popup_attrs = popup_attrs,
    ) + r#"<span class="popup-hover-bridge"></span><div class="popup-body"><div class="tooltip-body">"#
        + body
        + r#"</div><div class="arrow" role="presentation"></div></div></div></div>"#
}

#[test]
fn default() {
    let expected = tooltip(
        r#"class="tooltip" role="tooltip" data-trigger="hover focus""#,
        r#"data-placement="top" data-distance="8""#,
        "",
    );

    let tooltip = Tooltip::builder();
    assert_eq!(tooltip.render().as_inner(), &expected);

    let tooltip = rsx! { <Tooltip/> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn anchor_and_content() {
    let expected = tooltip(
        r#"class="tooltip" role="tooltip" data-trigger="hover focus""#,
        r#"data-placement="top" data-anchor="my-button" data-distance="8""#,
        "This is a tooltip",
    );

    let tooltip = Tooltip::builder().anchor_id("my-button").children(&"This is a tooltip");
    assert_eq!(tooltip.render().as_inner(), &expected);

    let tooltip = rsx! { <Tooltip anchor_id="my-button">"This is a tooltip"</Tooltip> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn positioning() {
    let expected = tooltip(
        r#"class="tooltip" role="tooltip" data-trigger="hover focus""#,
        r#"data-placement="right-start" data-anchor="anchor" data-distance="30" data-skidding="-10""#,
        "",
    );

    let tooltip = Tooltip::builder()
        .placement(RightStart)
        .anchor_id("anchor")
        .distance(30)
        .skidding(-10);
    assert_eq!(tooltip.render().as_inner(), &expected);

    let tooltip = rsx! { <Tooltip placement=RightStart anchor_id="anchor" distance=30 skidding=-10/> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn triggers_and_delays() {
    let expected = tooltip(
        r#"class="tooltip" role="tooltip" data-trigger="hover click" data-show-delay="500" data-hide-delay="200""#,
        r#"data-placement="top" data-distance="8""#,
        "",
    );

    let tooltip = Tooltip::builder()
        .trigger(Hover | Click)
        .show_delay(500)
        .hide_delay(200);
    assert_eq!(tooltip.render().as_inner(), &expected);

    let tooltip = rsx! { <Tooltip trigger=(Hover | Click) show_delay=500 hide_delay=200/> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn manual_and_open() {
    let expected = tooltip(
        r#"class="tooltip" role="tooltip" data-trigger="manual" data-open="""#,
        r#"data-placement="top" data-distance="8""#,
        "",
    );

    let tooltip = rsx! { <Tooltip trigger=Manual open=true/> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn disabled() {
    let expected = tooltip(
        r#"class="tooltip disabled" role="tooltip" data-trigger="hover focus""#,
        r#"data-placement="top" data-distance="8""#,
        "",
    );

    let tooltip = rsx! { <Tooltip disabled=true/> };
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn without_arrow() {
    let expected = concat!(
        r#"<div class="tooltip" role="tooltip" data-trigger="hover focus">"#,
        r#"<div class="popup" data-placement="top" data-distance="8" "#,
        r#"data-flip="" data-shift="" data-shift-padding="8">"#,
        r#"<span class="popup-hover-bridge"></span>"#,
        r#"<div class="popup-body"><div class="tooltip-body">No arrow</div></div></div></div>"#,
    );

    let tooltip = rsx! { <Tooltip arrow=false>"No arrow"</Tooltip> };
    assert_eq!(tooltip.render().as_inner(), expected);
}

#[test]
fn additional_attributes() {
    let expected = tooltip(
        r#"id="the-tooltip" class="tooltip test" style="--max-width: 80px;" role="tooltip" data-trigger="hover focus""#,
        r#"data-placement="top" data-distance="8""#,
        "",
    );

    let tooltip = Tooltip::builder()
        .id("the-tooltip")
        .class("test")
        .style("--max-width: 80px;");
    assert_eq!(tooltip.render().as_inner(), &expected);
}

#[test]
fn trigger_flags() {
    assert_eq!(TooltipTriggers::default().into_str(), "hover focus");
    assert_eq!(TooltipTriggers::from(Manual).into_str(), "manual");
    assert_eq!((Hover | Focus | Click).into_str(), "hover focus click");

    let triggers = Hover | Click;
    assert!(triggers.contains(Hover));
    assert!(triggers.contains(Click));
    assert!(!triggers.contains(Focus));
}
