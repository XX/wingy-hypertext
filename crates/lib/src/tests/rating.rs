use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome;

use crate::component::rating::RatingSymbolState::*;
use crate::component::rating::{Rating, RatingSymbol};

fn star_symbol(position: u32) -> String {
    format!(
        r#"<span class="rating-symbol" role="presentation" style="--position: {position}"><span class="rating-symbol-empty">{}</span><span class="rating-symbol-filled">{}</span></span>"#,
        fontawesome::regular::Star.render().as_inner(),
        fontawesome::solid::Star.render().as_inner(),
    )
}

fn custom_symbol(position: u32, empty: &str, filled: &str) -> String {
    format!(
        r#"<span class="rating-symbol" role="presentation" style="--position: {position}"><span class="rating-symbol-empty">{empty}</span><span class="rating-symbol-filled">{filled}</span></span>"#
    )
}

#[test]
fn default_rating() {
    let rating_markup = format!(
        r#"<div class="rating" style="--value: 0" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="0" aria-valuemax="5" aria-readonly="false" aria-disabled="false" data-precision="1"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value=""></div>"#,
        (1..=5).map(star_symbol).collect::<String>()
    );

    let rating = Rating::builder();
    assert_eq!(rating.render().as_inner(), &rating_markup);

    let rating = rsx! { <Rating/> };
    assert_eq!(rating.render().as_inner(), &rating_markup);
}

#[test]
fn rating_value() {
    let rating_markup = format!(
        r#"<div class="rating" style="--value: 2.5" role="slider" tabindex="0" aria-label="Rating" aria-valuemin="0" aria-valuenow="2.5" aria-valuemax="3" aria-readonly="false" aria-disabled="false" data-precision="0.5"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" name="score" value="2.5" required="true"></div>"#,
        (1..=3).map(star_symbol).collect::<String>()
    );

    let rating = Rating::builder()
        .label("Rating")
        .name("score")
        .value(2.5)
        .max(3)
        .precision(0.5)
        .required(true);
    assert_eq!(rating.render().as_inner(), &rating_markup);

    let rating = rsx! { <Rating label="Rating" name="score" value=2.5 max=3 precision=0.5 required=true/> };
    assert_eq!(rating.render().as_inner(), &rating_markup);

    // The value is kept within the symbols
    let rating = rsx! { <Rating max=3 value=7.0/> };
    assert!(rating.render().as_inner().contains(r#"aria-valuenow="3""#));
}

#[test]
fn rating_states() {
    let readonly_markup = format!(
        r#"<div class="rating readonly" style="--value: 3" role="slider" tabindex="-1" aria-valuemin="0" aria-valuenow="3" aria-valuemax="5" aria-readonly="true" aria-disabled="false" data-precision="1"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value="3"></div>"#,
        (1..=5).map(star_symbol).collect::<String>()
    );
    let rating = rsx! { <Rating readonly=true value=3.0/> };
    assert_eq!(rating.render().as_inner(), &readonly_markup);

    let disabled_markup = format!(
        r#"<div class="rating disabled" style="--value: 3" role="slider" tabindex="-1" aria-valuemin="0" aria-valuenow="3" aria-valuemax="5" aria-readonly="false" aria-disabled="true" data-precision="1"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value="3" disabled="true"></div>"#,
        (1..=5).map(star_symbol).collect::<String>()
    );
    let rating = rsx! { <Rating disabled=true value=3.0/> };
    assert_eq!(rating.render().as_inner(), &disabled_markup);
}

#[test]
fn rating_custom_symbols() {
    let heart = rsx! { <b>"♥"</b> }.render().into_inner();
    let empty = rsx! { <i>"○"</i> }.render().into_inner();
    let filled = rsx! { <i>"●"</i> }.render().into_inner();

    // The same symbol in both states
    let rating_markup = format!(
        r#"<div class="rating" style="--value: 0" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="0" aria-valuemax="2" aria-readonly="false" aria-disabled="false" data-precision="1"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value=""></div>"#,
        (1..=2)
            .map(|position| custom_symbol(position, &heart, &heart))
            .collect::<String>()
    );
    let rating = rsx! {
        <Rating max=2>
            <RatingSymbol><b>"♥"</b></RatingSymbol>
        </Rating>
    };
    assert_eq!(rating.render().as_inner(), &rating_markup);

    // A symbol per state
    let rating_markup = format!(
        r#"<div class="rating" style="--value: 0" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="0" aria-valuemax="2" aria-readonly="false" aria-disabled="false" data-precision="1"><span class="rating-symbols">{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value=""></div>"#,
        (1..=2)
            .map(|position| custom_symbol(position, &empty, &filled))
            .collect::<String>()
    );
    let rating = rsx! {
        <Rating max=2>
            <RatingSymbol state=Empty><i>"○"</i></RatingSymbol>
            <RatingSymbol state=Full><i>"●"</i></RatingSymbol>
        </Rating>
    };
    assert_eq!(rating.render().as_inner(), &rating_markup);
}

#[test]
fn rating_symbol_per_value() {
    let rating_markup = format!(
        r#"<div class="rating" style="--value: 0" role="slider" tabindex="0" aria-valuemin="0" aria-valuenow="0" aria-valuemax="3" aria-readonly="false" aria-disabled="false" data-precision="1"><span class="rating-symbols">{}{}{}</span><input class="rating-value" type="text" tabindex="-1" autocomplete="off" aria-hidden="true" value=""></div>"#,
        custom_symbol(1, "one", "one"),
        custom_symbol(2, "two", "TWO"),
        custom_symbol(3, "", ""),
    );
    let rating = rsx! {
        <Rating max=3>
            <RatingSymbol value=1>"one"</RatingSymbol>
            <RatingSymbol value=2 state=Empty>"two"</RatingSymbol>
            <RatingSymbol value=2 state=Full>"TWO"</RatingSymbol>
        </Rating>
    };
    assert_eq!(rating.render().as_inner(), &rating_markup);

    // Outside of a rating the symbol is always rendered
    let symbol = rsx! { <RatingSymbol value=2 state=Full>"two"</RatingSymbol> };
    assert_eq!(symbol.render().as_inner(), "two");
}
