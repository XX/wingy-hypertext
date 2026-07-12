use hypertext::{RenderableExt, rsx};

use crate::attributes::CommonAttributeSetters;
use crate::layouts::divider::Divider;
use crate::orientation::Orientation::*;

#[test]
fn default() {
    let divider_markup = r#"<div class="divider" role="separator" aria-orientation="horizontal"></div>"#;

    let divider = Divider::builder();
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider = rsx! { <Divider/> };
    assert_eq!(divider.render().as_inner(), divider_markup);
}

#[test]
fn orientation() {
    let divider_markup = r#"<div class="divider" role="separator" aria-orientation="horizontal"></div>"#;

    let divider = Divider::builder().orientation(Horizontal);
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider = rsx! { <Divider orientation=Horizontal/> };
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider_markup = r#"<div class="divider vertical" role="separator" aria-orientation="vertical"></div>"#;

    let divider = Divider::builder().orientation(Vertical);
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider = rsx! { <Divider orientation=Vertical/> };
    assert_eq!(divider.render().as_inner(), divider_markup);
}

#[test]
fn additional_attributes() {
    let divider_markup = r#"<div id="the-divider" class="divider test" style="--width: 4px" role="separator" aria-orientation="horizontal"></div>"#;

    let divider = Divider::builder().id("the-divider").class("test").style("--width: 4px");
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider = rsx! { <Divider id="the-divider" class="test" style="--width: 4px"/> };
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider_markup = r#"<div class="divider vertical" style="--color: red; --spacing: 2rem" role="separator" aria-orientation="vertical"></div>"#;

    let divider = Divider::builder()
        .orientation(Vertical)
        .style("--color: red")
        .style("--spacing: 2rem");
    assert_eq!(divider.render().as_inner(), divider_markup);

    let divider = rsx! { <Divider orientation=Vertical style="--color: red" style="--spacing: 2rem"/> };
    assert_eq!(divider.render().as_inner(), divider_markup);
}
