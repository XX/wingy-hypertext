use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome_ext;

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::component::head::HeadLevel;
use crate::icon_placement::ExpandIconPlacement::*;
use crate::layout::accordion::AccordionMode::*;
use crate::layout::accordion::{
    Accordion, AccordionItem, AccordionItemIcon, AccordionItemLabel, AccordionItemPanel, AccordionItemTrigger,
};

/// The chevron is rendered from the `iconic` crate; build the expected markup
/// dynamically so the tests don't hardcode the SVG.
fn chevron() -> String {
    fontawesome_ext::regular::ChevronRight.render().into_inner()
}

fn default_icon() -> String {
    format!(
        r#"<span class="accordion-item-icon accordion-item-default-icon" aria-hidden="true">{}</span>"#,
        chevron()
    )
}

/// A trigger with the default label and icon, without ids.
fn trigger(tag: &str, label: &str, expanded: bool, disabled: bool) -> String {
    let button = format!(
        concat!(
            r#"<button class="accordion-item-trigger" type="button" aria-expanded="{expanded}" "#,
            r#"aria-disabled="{disabled}" tabindex="{tabindex}">"#,
            r#"<span class="accordion-item-label">{label}</span>{icon}</button>"#,
        ),
        expanded = expanded,
        disabled = disabled,
        tabindex = if disabled { "-1" } else { "0" },
        label = label,
        icon = default_icon(),
    );

    if tag.is_empty() {
        button
    } else {
        format!(r#"<{tag} class="head accordion-item-heading">{button}</{tag}>"#)
    }
}

fn panel(content: &str, expanded: bool) -> String {
    format!(
        r#"<div class="accordion-item-panel" role="region"{hidden}><div class="accordion-item-content">{content}</div></div>"#,
        hidden = if expanded { "" } else { r#" hidden="until-found""# },
    )
}

#[test]
fn default() {
    let expected = r#"<div class="accordion outlined"></div>"#;

    let accordion = Accordion::builder();
    assert_eq!(accordion.render().as_inner(), &expected);

    let accordion = rsx! { <Accordion/> };
    assert_eq!(accordion.render().as_inner(), &expected);
}

#[test]
fn accordion_props() {
    let expected = r#"<div class="accordion filled icon-start" data-mode="single"></div>"#;
    let accordion = rsx! { <Accordion appearance=Filled icon_placement=Start mode=Single/> };
    assert_eq!(accordion.render().as_inner(), &expected);

    let expected = r#"<div class="accordion plain" data-mode="single-collapsible"></div>"#;
    let accordion = Accordion::builder().appearance(Plain).mode(SingleCollapsible);
    assert_eq!(accordion.render().as_inner(), &expected);
}

#[test]
fn item() {
    let expected = format!(
        r#"<div class="accordion outlined"><div class="accordion-item">{}{}</div></div>"#,
        trigger("h3", "First", false, false),
        panel("Content", false),
    );

    let accordion = rsx! {
        <Accordion>
            <AccordionItem label="First">"Content"</AccordionItem>
        </Accordion>
    };
    assert_eq!(accordion.render().as_inner(), &expected);

    let item = AccordionItem::builder().label("First").children(&"Content");
    let accordion = Accordion::builder().children(&item);
    assert_eq!(accordion.render().as_inner(), &expected);
}

#[test]
fn expanded_and_disabled() {
    let expected = format!(
        r#"<div class="accordion-item expanded">{}{}</div>"#,
        trigger("h3", "First", true, false),
        panel("Content", true),
    );
    let item = rsx! { <AccordionItem label="First" expanded=true>"Content"</AccordionItem> };
    assert_eq!(item.render().as_inner(), &expected);

    let expected = format!(
        r#"<div class="accordion-item disabled">{}{}</div>"#,
        trigger("h3", "First", false, true),
        panel("Content", false),
    );
    let item = rsx! { <AccordionItem label="First" disabled=true>"Content"</AccordionItem> };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn heading_level() {
    for (level, tag) in [
        (HeadLevel::H1, "h1"),
        (HeadLevel::H2, "h2"),
        (HeadLevel::H4, "h4"),
        (HeadLevel::H6, "h6"),
        (HeadLevel::NoHeading, ""),
    ] {
        let expected = format!(
            r#"<div class="accordion-item">{}{}</div>"#,
            trigger(tag, "Label", false, false),
            panel("", false),
        );
        let item = AccordionItem::builder().label("Label").heading_level(level);
        assert_eq!(item.render().as_inner(), &expected);
    }
}

#[test]
fn ids() {
    let expected = format!(
        concat!(
            r#"<div id="faq" class="accordion-item">"#,
            r#"<h3 class="head accordion-item-heading"><button id="faq-trigger" class="accordion-item-trigger" "#,
            r#"type="button" aria-expanded="false" aria-controls="faq-panel" aria-disabled="false" tabindex="0">"#,
            r#"<span class="accordion-item-label">Label</span>{icon}</button></h3>"#,
            r#"<div id="faq-panel" class="accordion-item-panel" role="region" aria-labelledby="faq-trigger" "#,
            r#"hidden="until-found"><div class="accordion-item-content">Content</div></div></div>"#,
        ),
        icon = default_icon(),
    );

    let item = rsx! { <AccordionItem id="faq" label="Label">"Content"</AccordionItem> };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn composed() {
    let expected = format!(
        concat!(
            r#"<div class="accordion-item expanded">"#,
            r#"<h3 class="head accordion-item-heading"><button class="accordion-item-trigger" type="button" "#,
            r#"aria-expanded="true" aria-disabled="false" tabindex="0">"#,
            r#"<span class="accordion-item-label"><strong>Tasks</strong></span>{icon}</button></h3>{panel}</div>"#,
        ),
        icon = default_icon(),
        panel = panel("Content", true),
    );

    let item = rsx! {
        <AccordionItem expanded=true>
            <AccordionItemTrigger expanded=true><strong>"Tasks"</strong></AccordionItemTrigger>
            <AccordionItemPanel expanded=true>"Content"</AccordionItemPanel>
        </AccordionItem>
    };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn custom_icon() {
    let expected = concat!(
        r#"<h3 class="head accordion-item-heading"><button class="accordion-item-trigger" type="button" "#,
        r#"aria-expanded="false" aria-disabled="false" tabindex="0">"#,
        r#"<span class="accordion-item-label">Label</span>"#,
        r#"<span class="accordion-item-icon plus" aria-hidden="true"><i></i></span></button></h3>"#,
    );

    let trigger = rsx! {
        <AccordionItemTrigger bare=true>
            <AccordionItemLabel>"Label"</AccordionItemLabel>
            <AccordionItemIcon class="plus"><i></i></AccordionItemIcon>
        </AccordionItemTrigger>
    };
    assert_eq!(trigger.render().as_inner(), &expected);
}

#[test]
fn additional_attributes() {
    let expected = r#"<div id="the-accordion" class="accordion outlined test" style="font-size: 0.875rem"></div>"#;
    let accordion = Accordion::builder()
        .id("the-accordion")
        .class("test")
        .style("font-size: 0.875rem");
    assert_eq!(accordion.render().as_inner(), &expected);
}
