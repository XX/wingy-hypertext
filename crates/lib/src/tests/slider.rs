use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::attributes::CommonAttributeSetters;
use crate::component::slider::Slider;
use crate::orientation::Orientation::*;

#[test]
fn default() {
    let slider_markup = r#"<div class="slider horizontal" data-min="0" data-max="100" data-step="1"><div class="slider-control" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="0" aria-valuetext="0" aria-valuemax="100" aria-orientation="horizontal" aria-disabled="false" aria-readonly="false"><div class="slider-track"><div class="slider-indicator" style="--start: 0%; --end: 0%"></div><span class="slider-thumb" style="--position: 0%"></span></div></div><input type="hidden" value="0"></div>"#;

    let slider = Slider::builder();
    assert_eq!(slider.render().as_inner(), slider_markup);

    let slider = rsx! { <Slider/> };
    assert_eq!(slider.render().as_inner(), slider_markup);
}

/// The positions of the thumb and of the filled part are rendered server-side.
#[test]
fn positions() {
    let slider_markup = r#"<div class="slider horizontal" data-min="0" data-max="100" data-step="5"><div class="slider-control" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="25" aria-valuetext="25" aria-valuemax="100" aria-orientation="horizontal" aria-disabled="false" aria-readonly="false"><div class="slider-track"><div class="slider-indicator" style="--start: 0%; --end: 25%"></div><span class="slider-thumb" style="--position: 25%"></span></div></div><input type="hidden" name="volume" value="25"></div>"#;

    let slider = Slider::builder().value(25.0).step(5.0).name("volume");
    assert_eq!(slider.render().as_inner(), slider_markup);

    let slider = rsx! { <Slider value=25.0 step=5.0 name="volume"/> };
    assert_eq!(slider.render().as_inner(), slider_markup);
}

/// The filled part starts from `indicator_offset` instead of the minimum.
#[test]
fn indicator_offset() {
    let slider = Slider::builder()
        .min(-100.0)
        .max(100.0)
        .value(50.0)
        .indicator_offset(0.0)
        .render();

    assert!(slider.as_inner().contains(r#"style="--start: 50%; --end: 75%""#));
}

#[test]
fn label_hint_and_states() {
    let slider_markup = r#"<div id="volume" class="slider vertical disabled size-small" data-min="0" data-max="100" data-step="1" data-readonly=""><label class="label">Volume</label><div class="slider-control" role="slider" tabindex="-1" aria-valuemin="0" aria-valuenow="0" aria-valuetext="0" aria-valuemax="100" aria-describedby="volume-hint" aria-label="Volume" aria-orientation="vertical" aria-disabled="true" aria-readonly="true"><div class="slider-track"><div class="slider-indicator" style="--start: 0%; --end: 0%"></div><span id="volume-thumb" class="slider-thumb" style="--position: 0%"></span></div></div><input type="hidden" value="0"><small id="volume-hint" class="hint">Louder to the right</small></div>"#;

    let slider = Slider::builder()
        .id("volume")
        .class(crate::class::SIZE_SMALL)
        .orientation(Vertical)
        .disabled(true)
        .readonly(true)
        .label("Volume")
        .hint("Louder to the right");
    assert_eq!(slider.render().as_inner(), slider_markup);
}

/// Markers are rendered once per step; the stylesheet hides the ones at the ends.
#[test]
fn markers() {
    let slider = Slider::builder().step(20.0).markers(true).render();
    let markup = slider.as_inner();

    assert_eq!(markup.matches("slider-marker\"").count(), 6);
    assert!(markup.contains(r#"<div class="slider-markers" aria-hidden="true">"#));
    assert!(markup.contains(r#"<span class="slider-marker" style="--position: 40%"></span>"#));
}

/// A range slider carries two thumbs, each with its own value, and submits two
/// values under the `-min` and `-max` names.
#[test]
fn range() {
    let slider = Slider::builder().range(20.0..=60.0).name("price").render();
    let markup = slider.as_inner();

    assert!(markup.contains(r#"<div class="slider-indicator" style="--start: 20%; --end: 60%"></div>"#));
    assert!(markup.contains(
        r#"<span class="slider-thumb slider-thumb-min" style="--position: 20%" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="20" aria-valuetext="20" aria-valuemax="100" aria-label="Minimum value""#
    ));
    assert!(
        markup.contains(r#"aria-valuenow="60" aria-valuetext="60" aria-valuemax="100" aria-label="Maximum value""#)
    );
    assert!(markup.contains(r#"<input type="hidden" name="price-min" value="20">"#));
    assert!(markup.contains(r#"<input type="hidden" name="price-max" value="60">"#));

    // The control isn't the slider itself when the thumbs are
    assert!(!markup.contains(r#"<div class="slider-control" role="slider""#));
}

/// The range thumbs get the label of the slider, as in `wa-slider`.
#[test]
fn range_labels() {
    let slider = Slider::builder().range(0.0..=50.0).label("Price").render();
    let markup = slider.as_inner();

    assert!(markup.contains(r#"aria-label="Price (minimum value)""#));
    assert!(markup.contains(r#"aria-label="Price (maximum value)""#));
}

/// The tooltip anchors to the thumb, so it is only rendered for a slider with
/// an `id`.
#[test]
fn tooltip() {
    let slider = Slider::builder()
        .id("volume")
        .value(30.0)
        .tooltip(Default::default())
        .render();
    let markup = slider.as_inner();

    assert!(markup.contains(r#"<span id="volume-thumb" class="slider-thumb""#));
    assert!(markup.contains(r#"data-trigger="manual""#));
    assert!(markup.contains(r#"data-anchor="volume-thumb""#));
    assert!(markup.contains(r#"<div class="tooltip-body">30</div>"#));

    let without_id = Slider::builder().value(30.0).tooltip(Default::default()).render();
    assert!(!without_id.as_inner().contains("tooltip"));
}

#[test]
fn references() {
    let references = rsx! {
        <span>"0"</span>
        <span>"100"</span>
    };
    let slider = Slider::builder().children(&references).render();

    assert!(
        slider
            .as_inner()
            .contains(r#"<div class="slider-references" aria-hidden="true"><span>0</span><span>100</span></div>"#)
    );
}
