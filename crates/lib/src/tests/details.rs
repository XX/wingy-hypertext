use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{RenderableExt, rsx};
use iconic::{fontawesome, fontawesome_ext};

use crate::appearance::Appearance::*;
use crate::appearance::AppearanceConstructor;
use crate::attributes::CommonAttributeSetters;
use crate::class::{DETAILS_COLLAPSE_ICON, DETAILS_EXPAND_ICON, DETAILS_ICON, DETAILS_SUMMARY};
use crate::icon_placement::ExpandIconPlacement::*;
use crate::layout::details::{Details, DetailsBody, DetailsHeader};

fn chevron() -> String {
    fontawesome_ext::regular::ChevronRight.render().into_inner()
}

/// The icon slots of the header with the default chevron in both.
fn default_icons() -> String {
    format!(
        concat!(
            r#"<span class="details-icon" aria-hidden="true">"#,
            r#"<span class="details-expand-icon details-default-icon">{chevron}</span>"#,
            r#"<span class="details-collapse-icon details-default-icon">{chevron}</span>"#,
            r#"</span>"#,
        ),
        chevron = chevron(),
    )
}

/// The whole markup of a details without an id. `attrs` go on the
/// `<details>`, `header_attrs` on the `<summary>`.
fn details(classes: &str, attrs: &str, header_attrs: &str, summary: &str, icons: &str, content: &str) -> String {
    format!(
        concat!(
            r#"<details class="details {classes}"{attrs}>"#,
            r#"<summary class="details-header" role="button"{header_attrs}>"#,
            r#"<span class="details-summary">{summary}</span>{icons}</summary>"#,
            r#"<div class="details-body" role="region"><div class="details-content">{content}</div></div>"#,
            r#"</details>"#,
        ),
        classes = classes,
        attrs = attrs,
        header_attrs = header_attrs,
        summary = summary,
        icons = icons,
        content = content,
    )
}

const COLLAPSED: &str = r#" aria-expanded="false""#;

#[test]
fn default() {
    let markup = "<details class=\"details outlined\"></details>";

    let details_ = Details::builder();
    assert_eq!(details_.render().as_inner(), markup);

    let details_ = rsx! { <Details/> };
    assert_eq!(details_.render().as_inner(), markup);

    let details_ = rsx! { <Details></Details> };
    assert_eq!(details_.render().as_inner(), markup);
}

#[test]
fn summary_and_content() {
    let markup = details("outlined", "", COLLAPSED, "", &default_icons(), "");

    let details_ = Details::builder().summary("");
    assert_eq!(details_.render().as_inner(), markup.as_str());

    let details_ = rsx! { <Details summary=""/> };
    assert_eq!(details_.render().as_inner(), markup.as_str());

    let details_ = rsx! { <Details summary=""></Details> };
    assert_eq!(details_.render().as_inner(), markup.as_str());

    let markup = details(
        "outlined",
        "",
        COLLAPSED,
        "Toggle Me",
        &default_icons(),
        "Hello, world!",
    );

    let details_ = rsx! { <Details summary="Toggle Me">"Hello, world!"</Details> };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn html_in_summary() {
    let markup = details(
        "outlined",
        "",
        COLLAPSED,
        "<strong>Bold</strong> summary",
        &default_icons(),
        "Content",
    );

    let details_ = rsx! {
        <Details>
            <DetailsHeader><strong>"Bold"</strong>" summary"</DetailsHeader>
            <DetailsBody>"Content"</DetailsBody>
        </Details>
    };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn open() {
    let markup = details(
        "outlined open",
        r#" open="""#,
        r#" aria-expanded="true""#,
        "Toggle Me",
        &default_icons(),
        "",
    );

    let details_ = rsx! { <Details summary="Toggle Me" open=true/> };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn disabled() {
    let markup = details(
        "outlined disabled",
        "",
        r#" aria-expanded="false" aria-disabled="true" tabindex="-1""#,
        "Disabled",
        &default_icons(),
        "",
    );

    let details_ = rsx! { <Details summary="Disabled" disabled=true/> };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn name() {
    let markup = r#"<details class="details outlined" name="group-1"></details>"#;

    let details_ = rsx! { <Details name="group-1"/> };
    assert_eq!(details_.render().as_inner(), markup);
}

#[test]
fn appearance() {
    for (appearance, class) in [
        (Filled, "filled"),
        (FilledOutlined, "filled-outlined"),
        (Plain, "plain"),
        (Outlined, "outlined"),
    ] {
        let markup = format!(r#"<details class="details {class}"></details>"#);

        let details_ = rsx! { <Details appearance=(appearance)/> };
        assert_eq!(details_.render().as_inner(), markup.as_str());
    }

    let details_ = Details::filled();
    assert_eq!(
        details_.render().as_inner(),
        r#"<details class="details filled"></details>"#
    );
}

#[test]
fn icon_placement() {
    let markup = r#"<details class="details outlined icon-start"></details>"#;

    let details_ = rsx! { <Details icon_placement=Start/> };
    assert_eq!(details_.render().as_inner(), markup);

    let markup = r#"<details class="details outlined"></details>"#;

    let details_ = rsx! { <Details icon_placement=End/> };
    assert_eq!(details_.render().as_inner(), markup);
}

#[test]
fn custom_icons() {
    let plus = fontawesome::solid::Plus.render().into_inner();
    let minus = fontawesome::solid::Minus.render().into_inner();

    let icons = format!(
        concat!(
            r#"<span class="details-icon" aria-hidden="true">"#,
            r#"<span class="details-expand-icon">{plus}</span>"#,
            r#"<span class="details-collapse-icon">{minus}</span>"#,
            r#"</span>"#,
        ),
        plus = plus,
        minus = minus,
    );
    let markup = details("outlined", "", COLLAPSED, "", &icons, "");

    let details_ = rsx! {
        <Details>
            <DetailsHeader bare=true>
                <span class=DETAILS_SUMMARY></span>
                <span class=DETAILS_ICON aria-hidden="true">
                    <span class=DETAILS_EXPAND_ICON>
                        (fontawesome::solid::Plus)
                    </span>
                    <span class=DETAILS_COLLAPSE_ICON>
                        (fontawesome::solid::Minus)
                    </span>
                </span>
            </DetailsHeader>
            <DetailsBody></DetailsBody>
        </Details>
    };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn id() {
    let markup = format!(
        concat!(
            r#"<details id="faq" class="details outlined">"#,
            r#"<summary id="faq-header" class="details-header" role="button" aria-expanded="false" aria-controls="faq-body">"#,
            r#"<span class="details-summary">Question</span>{icons}</summary>"#,
            r#"<div id="faq-body" class="details-body" role="region" aria-labelledby="faq-header">"#,
            r#"<div class="details-content">Answer</div></div>"#,
            r#"</details>"#,
        ),
        icons = default_icons(),
    );

    let details_ = rsx! { <Details id="faq" summary="Question">"Answer"</Details> };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}

#[test]
fn additional_attributes() {
    let markup = r#"<details class="details outlined faq" style="--spacing: 2rem;"></details>"#;

    let details_ = rsx! { <Details class="faq" style="--spacing: 2rem;"/> };
    assert_eq!(details_.render().as_inner(), markup);

    let details_ = Details::builder().class("faq").style("--spacing: 2rem;");
    assert_eq!(details_.render().as_inner(), markup);
}

#[test]
fn nested_markup() {
    let markup = details("outlined", "", COLLAPSED, "", &default_icons(), "<p>Paragraph</p>");

    let details_ = rsx! {
        <Details>
            <DetailsHeader></DetailsHeader>
            <DetailsBody>
                <p>"Paragraph"</p>
            </DetailsBody>
        </Details>
    };
    assert_eq!(details_.render().as_inner(), markup.as_str());
}
