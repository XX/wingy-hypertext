use hypertext::{RenderableExt, rsx};
use iconic::fontawesome_ext;

use crate::attributes::CommonAttributeSetters;
use crate::component::head::HeadLevel::*;
use crate::component::head::HeadSize::*;
use crate::component::head::{Head, HeadLevel};

#[test]
fn default() {
    let expected = r#"<h1 class="head">Title</h1>"#;

    let head = Head::builder().children(&"Title");
    assert_eq!(head.render().as_inner(), &expected);

    let head = rsx! { <Head>"Title"</Head> };
    assert_eq!(head.render().as_inner(), &expected);
}

#[test]
fn levels() {
    for (level, tag) in [(H1, "h1"), (H2, "h2"), (H3, "h3"), (H4, "h4"), (H5, "h5"), (H6, "h6")] {
        let expected = format!(r#"<{tag} class="head">Title</{tag}>"#);
        let head = Head::builder().level(level).children(&"Title");
        assert_eq!(head.render().as_inner(), &expected);
    }
}

#[test]
fn size() {
    let expected = r#"<h2 class="head wa-heading-m">Title</h2>"#;

    let head = rsx! { <Head level=H2 size=M>"Title"</Head> };
    assert_eq!(head.render().as_inner(), &expected);

    let head = Head::builder().level(H2).size(M).children(&"Title");
    assert_eq!(head.render().as_inner(), &expected);
}

#[test]
fn no_heading() {
    let expected = r#"<div class="head wa-heading">Title</div>"#;
    let head = rsx! { <Head level=(HeadLevel::NoHeading)>"Title"</Head> };
    assert_eq!(head.render().as_inner(), &expected);

    let expected = r#"<div class="head wa-heading-5xl">Title</div>"#;
    let head = rsx! { <Head level=(HeadLevel::NoHeading) size=XL5>"Title"</Head> };
    assert_eq!(head.render().as_inner(), &expected);
}

#[test]
fn anchor() {
    let icon = fontawesome_ext::regular::Hashtag.render().into_inner();
    let expected = format!(
        concat!(
            r#"<h2 id="examples" class="head anchor-head">Examples"#,
            r##"<a href="#examples"><span class="wa-visually-hidden">Jump to heading</span>"##,
            r#"<span class="icon icon-shrink">{icon}</span></a></h2>"#,
        ),
        icon = icon,
    );

    let head = rsx! { <Head level=H2 id="examples" anchor=true>"Examples"</Head> };
    assert_eq!(head.render().as_inner(), &expected);
}

#[test]
fn additional_attributes() {
    let expected = r#"<h3 id="title" class="head test" style="color: red">Title</h3>"#;
    let head = Head::builder()
        .level(H3)
        .id("title")
        .class("test")
        .style("color: red")
        .children(&"Title");
    assert_eq!(head.render().as_inner(), &expected);
}
