use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance;
use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::layout::card::{
    Card, CardActions, CardBody, CardFooter, CardFooterActions, CardHeader, CardHeaderActions, CardMedia,
};
use crate::orientation::Orientation::*;

#[test]
fn default() {
    let expected = r#"<div class="card outlined"></div>"#;

    let card = Card::builder();
    assert_eq!(card.render().as_inner(), &expected);

    let card = rsx! { <Card/> };
    assert_eq!(card.render().as_inner(), &expected);

    let card = rsx! { <Card orientation=Vertical></Card> };
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn body() {
    let expected = r#"<div class="card outlined"><div class="card-body">Content</div></div>"#;

    let card_body = CardBody::builder().children(&"Content");
    let card = Card::builder().children(&card_body);
    assert_eq!(card.render().as_inner(), &expected);

    let card = rsx! { <Card><CardBody>"Content"</CardBody></Card> };
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn header_with_actions() {
    let expected = concat!(
        r#"<div class="card outlined"><div class="card-header"><h3>Title</h3>"#,
        r#"<div class="card-header-actions"><button>Settings</button></div></div>"#,
        r#"<div class="card-body">Content</div></div>"#,
    );

    let card = rsx! {
        <Card>
            <CardHeader>
                <h3>"Title"</h3>
                <CardHeaderActions><button>"Settings"</button></CardHeaderActions>
            </CardHeader>
            <CardBody>"Content"</CardBody>
        </Card>
    };
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn footer_with_actions() {
    let expected = concat!(
        r#"<div class="card outlined"><div class="card-body">Content</div>"#,
        r#"<div class="card-footer">Summary"#,
        r#"<div class="card-footer-actions"><button>Preview</button></div></div></div>"#,
    );

    let card = rsx! {
        <Card>
            <CardBody>"Content"</CardBody>
            <CardFooter>
                "Summary"
                <CardFooterActions><button>"Preview"</button></CardFooterActions>
            </CardFooter>
        </Card>
    };
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn media() {
    let expected = concat!(
        r#"<div class="card outlined"><div class="card-media"><img src="kitten.jpg" alt="A kitten"></div>"#,
        r#"<div class="card-body">Content</div></div>"#,
    );

    let card = rsx! {
        <Card>
            <CardMedia><img src="kitten.jpg" alt="A kitten"></CardMedia>
            <CardBody>"Content"</CardBody>
        </Card>
    };
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn appearance() {
    for appearance_class in [
        Appearance::ACCENT,
        Appearance::FILLED,
        Appearance::FILLED_OUTLINED,
        Appearance::OUTLINED,
        Appearance::PLAIN,
    ] {
        let expected = format!(r#"<div class="card {appearance_class}"></div>"#);
        let actual = match appearance_class {
            Appearance::ACCENT => rsx! { <Card appearance=Accent/> }.render().into_inner(),
            Appearance::FILLED => rsx! { <Card appearance=Filled/> }.render().into_inner(),
            Appearance::FILLED_OUTLINED => rsx! { <Card appearance=FilledOutlined/> }.render().into_inner(),
            Appearance::OUTLINED => rsx! { <Card appearance=Outlined/> }.render().into_inner(),
            Appearance::PLAIN => rsx! { <Card appearance=Plain/> }.render().into_inner(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    let expected = r#"<div class="card plain"></div>"#;
    let card = Card::builder().appearance(Plain);
    assert_eq!(card.render().as_inner(), &expected);
}

#[test]
fn horizontal() {
    let expected = concat!(
        r#"<div class="card outlined horizontal"><div class="card-media"><img src="kitten.jpg" alt="A kitten"></div>"#,
        r#"<div class="card-body">Content</div><div class="card-actions"><button>Actions</button></div></div>"#,
    );

    let card = rsx! {
        <Card orientation=Horizontal>
            <CardMedia><img src="kitten.jpg" alt="A kitten"></CardMedia>
            <CardBody>"Content"</CardBody>
            <CardActions><button>"Actions"</button></CardActions>
        </Card>
    };
    assert_eq!(card.render().as_inner(), &expected);

    let card = Card::builder().orientation(Horizontal);
    assert_eq!(
        card.render().as_inner(),
        r#"<div class="card outlined horizontal"></div>"#
    );
}

#[test]
fn common_attributes() {
    let expected = concat!(
        r#"<div id="card" class="card filled product" style="max-width: 300px">"#,
        r#"<div id="body" class="card-body extra">Content</div></div>"#,
    );

    let card_body = CardBody::builder().id("body").class("extra").children(&"Content");
    let card = Card::builder()
        .appearance(Filled)
        .id("card")
        .class("product")
        .style("max-width: 300px")
        .children(&card_body);
    assert_eq!(card.render().as_inner(), &expected);
}
