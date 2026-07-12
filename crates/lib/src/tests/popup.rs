use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::helpers::popup::Placement::*;
use crate::helpers::popup::{AutoSize, Popup, SyncSize};

#[test]
fn default() {
    let popup_markup = r#"<div class="popup" data-placement="top"><div class="popup-body"></div></div>"#;

    let popup = Popup::builder();
    assert_eq!(popup.render().as_inner(), popup_markup);

    let popup = rsx! { <Popup /> };
    assert_eq!(popup.render().as_inner(), popup_markup);
}

#[test]
fn active_with_anchor() {
    let popup_markup = r#"<div class="popup active" data-placement="bottom-start"><span>Anchor</span><div class="popup-body"><span>Content</span></div></div>"#;

    let popup = rsx! {
        <Popup placement=BottomStart active=true anchor=(rsx! { <span>"Anchor"</span> })>
            <span>"Content"</span>
        </Popup>
    };
    assert_eq!(popup.render().as_inner(), popup_markup);
}

#[test]
fn positioning_attributes() {
    let popup = Popup::builder()
        .placement(Bottom)
        .distance(8)
        .skidding(4)
        .flip(true)
        .shift(true)
        .sync(SyncSize::Width)
        .auto_size(AutoSize::Vertical)
        .auto_size_padding(10);
    let markup = popup.render();
    let markup = markup.as_inner();

    assert!(markup.contains(r#"data-placement="bottom""#));
    assert!(markup.contains(r#"data-distance="8""#));
    assert!(markup.contains(r#"data-skidding="4""#));
    assert!(markup.contains(r#"data-flip"#));
    assert!(markup.contains(r#"data-shift"#));
    assert!(markup.contains(r#"data-sync="width""#));
    assert!(markup.contains(r#"data-auto-size="vertical""#));
    assert!(markup.contains(r#"data-auto-size-padding="10""#));
}

#[test]
fn arrow_and_external_anchor() {
    let popup_markup = r#"<div class="popup" data-placement="top" data-anchor="external-anchor"><div class="popup-body">Content<div class="arrow" role="presentation"></div></div></div>"#;

    let popup = rsx! { <Popup anchor_id="external-anchor" arrow=true>"Content"</Popup> };
    assert_eq!(popup.render().as_inner(), popup_markup);
}

#[test]
fn hover_bridge() {
    let popup_markup = r#"<div class="popup" data-placement="top"><span class="popup-hover-bridge"></span><div class="popup-body"></div></div>"#;

    let popup = Popup::builder().hover_bridge(true);
    assert_eq!(popup.render().as_inner(), popup_markup);
}
