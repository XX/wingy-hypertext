use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::helpers::animation::{Animation, Direction, Fill};

#[test]
fn default() {
    let animation_markup = r#"<div class="animation"></div>"#;

    let animation = Animation::builder();
    assert_eq!(animation.render().as_inner(), animation_markup);

    let animation = rsx! { <Animation /> };
    assert_eq!(animation.render().as_inner(), animation_markup);
}

#[test]
fn playing_with_children() {
    let animation_markup =
        r#"<div class="animation" data-name="bounce" data-play="" data-duration="2000"><div>Box</div></div>"#;

    let animation = rsx! {
        <Animation name="bounce" duration=2000 play=true>
            <div>"Box"</div>
        </Animation>
    };
    assert_eq!(animation.render().as_inner(), animation_markup);
}

#[test]
fn timing_attributes() {
    let animation = Animation::builder()
        .name("tada")
        .delay(500)
        .direction(Direction::AlternateReverse)
        .duration(2000)
        .easing("easeInOutCubic")
        .end_delay(250)
        .fill(Fill::Both)
        .iterations(1.0)
        .iteration_start(0.5)
        .playback_rate(2.0);
    let markup = animation.render();
    let markup = markup.as_inner();

    assert!(markup.contains(r#"data-name="tada""#));
    assert!(!markup.contains(r#"data-play=""#));
    assert!(markup.contains(r#"data-delay="500""#));
    assert!(markup.contains(r#"data-direction="alternate-reverse""#));
    assert!(markup.contains(r#"data-duration="2000""#));
    assert!(markup.contains(r#"data-easing="easeInOutCubic""#));
    assert!(markup.contains(r#"data-end-delay="250""#));
    assert!(markup.contains(r#"data-fill="both""#));
    assert!(markup.contains(r#"data-iterations="1.0""#));
    assert!(markup.contains(r#"data-iteration-start="0.5""#));
    assert!(markup.contains(r#"data-playback-rate="2.0""#));
}

#[test]
fn custom_keyframes() {
    let animation_markup = r#"<div class="animation" data-keyframes="[{&quot;offset&quot;: 0, &quot;transform&quot;: &quot;rotate(0)&quot;}]"></div>"#;

    let animation = Animation::builder().keyframes(r#"[{"offset": 0, "transform": "rotate(0)"}]"#);
    assert_eq!(animation.render().as_inner(), animation_markup);
}
