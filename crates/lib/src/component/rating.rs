use std::borrow::Cow;
use std::cell::Cell;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    DISABLED, RATING, RATING_SYMBOL, RATING_SYMBOL_EMPTY, RATING_SYMBOL_FILLED, RATING_SYMBOLS, RATING_VALUE, READONLY,
};
use crate::convert::{bool_to_str, number_to_string};

/// A row of symbols, stars by default, to capture a quick score or to show an
/// average rating:
///
/// ```ignore
/// rsx! {
///     <Rating label="Rating" name="rating" value=3.0/>
/// }
/// ```
///
/// The current value is rendered server-side: every symbol is drawn twice — an
/// empty layer and a filled one on top of it — and the stylesheet clips them by
/// the `--value` custom property of the rating and the `--position` of the
/// symbol, so fractional values show partially filled symbols. Hovering,
/// clicking and the keyboard are implemented in `wingy-hypertext-web`
/// (`component::rating`) and wired up with `listen_ratings`; they only update
/// `--value` and the exposed value.
///
/// The value is submitted by a visually hidden input, which is empty while the
/// value is 0, so `required` is validated natively.
///
/// `children` replace the default stars, see [`RatingSymbol`].
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = RATING)]
#[props(builder)]
pub struct Rating<'a> {
    /// The current rating, from 0 to `max`.
    pub value: f64,

    /// The highest rating, which is also the number of symbols.
    pub max: u32,

    /// The step the rating changes by: 0.5 allows half-star ratings.
    pub precision: f64,

    /// Shows the value without letting the user change it. Unlike `disabled`,
    /// the value is still submitted with the form.
    pub readonly: bool,

    pub disabled: bool,

    /// Blocks submitting the form until a value is chosen.
    pub required: bool,

    /// The name the value is submitted under.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// Describes the rating to assistive devices. It isn't displayed, since a
    /// rating is usually identified by its context.
    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The symbol drawn at every position, made of [`RatingSymbol`]s.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Rating<'a> {
    fn default() -> Self {
        Self {
            value: 0.0,
            max: 5,
            precision: 1.0,
            readonly: false,
            disabled: false,
            required: false,
            name: None,
            label: None,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Rating<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.readonly { READONLY } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);

        let value = self.value.clamp(0.0, f64::from(self.max));
        let value_text = number_to_string(value);
        let style_line = self.style_line_with(&[&format!("--value: {value_text}")]);
        let interactive = !self.disabled && !self.readonly;

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="slider"
                tabindex=(if interactive { "0" } else { "-1" })
                aria-label=[&self.label]
                aria-valuemin="0"
                aria-valuenow=(&value_text)
                aria-valuemax=(self.max)
                aria-readonly=(bool_to_str(self.readonly))
                aria-disabled=(bool_to_str(self.disabled))
                data-precision=(number_to_string(self.precision))
                (self.get_attrs())
            >
                <span class=RATING_SYMBOLS>
                    @for position in 1..=self.max {
                        <span class=RATING_SYMBOL role="presentation" style=(format!("--position: {position}"))>
                            <span class=RATING_SYMBOL_EMPTY>
                                @if let Some(children) = self.children {
                                    (SymbolLayer { position, state: RatingSymbolState::Empty, children })
                                } @else {
                                    (fontawesome::regular::Star)
                                }
                            </span>
                            <span class=RATING_SYMBOL_FILLED>
                                @if let Some(children) = self.children {
                                    (SymbolLayer { position, state: RatingSymbolState::Full, children })
                                } @else {
                                    (fontawesome::solid::Star)
                                }
                            </span>
                        </span>
                    }
                </span>
                <input
                    class=RATING_VALUE
                    type="text"
                    tabindex="-1"
                    autocomplete="off"
                    aria-hidden="true"
                    name=[&self.name]
                    value=(if value > 0.0 { &value_text } else { "" })
                    required=[self.required.then_some(true)]
                    disabled=[self.disabled.then_some(true)]
                />
            </div>
        }
        .render_to(buffer);
    }
}

/// The state of a symbol a [`RatingSymbol`] is drawn in.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RatingSymbolState {
    /// Both the empty and the filled symbol.
    #[default]
    Any,
    /// The symbol beyond the value.
    Empty,
    /// The symbol within the value.
    Full,
}

/// A custom symbol of a [`Rating`], the counterpart of `getSymbol` in
/// `wa-rating`. The rating renders its children once per position and state,
/// and every `RatingSymbol` among them shows up only where it matches:
///
/// ```ignore
/// rsx! {
///     // The same symbol everywhere
///     <Rating label="Rating">
///         <RatingSymbol>(fontawesome::solid::Heart)</RatingSymbol>
///     </Rating>
///
///     // A different symbol for the empty and the filled state
///     <Rating label="Rating">
///         <RatingSymbol state=Empty>(fontawesome::regular::Heart)</RatingSymbol>
///         <RatingSymbol state=Full>(fontawesome::solid::Heart)</RatingSymbol>
///     </Rating>
///
///     // A different symbol at every position
///     <Rating label="Rating">
///         <RatingSymbol value=1>(fontawesome::regular::FaceAngry)</RatingSymbol>
///         <RatingSymbol value=2>(fontawesome::regular::FaceFrown)</RatingSymbol>
///         ...
///     </Rating>
/// }
/// ```
///
/// It renders no element of its own. Outside of a rating it always renders its
/// children.
#[derive(Default, Props)]
#[props(builder)]
pub struct RatingSymbol<'a> {
    /// The only position the symbol is drawn at, from 1 to `max`.
    pub value: Option<u32>,

    #[prop(impl_from)]
    pub state: RatingSymbolState,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for RatingSymbol<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let shown = CURRENT_SYMBOL.get().is_none_or(|(position, state)| {
            self.value.is_none_or(|value| value == position)
                && (self.state == RatingSymbolState::Any || self.state == state)
        });

        if shown && let Some(children) = self.children {
            children.render_to(buffer);
        }
    }
}

thread_local! {
    /// The position and the state the children of a [`Rating`] are being
    /// rendered for, read by the [`RatingSymbol`]s among them.
    static CURRENT_SYMBOL: Cell<Option<(u32, RatingSymbolState)>> = const { Cell::new(None) };
}

/// Renders the children of a [`Rating`] for one position and state.
struct SymbolLayer<'a> {
    position: u32,
    state: RatingSymbolState,
    children: &'a dyn Renderable,
}

impl<'a> Renderable for SymbolLayer<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let outer = CURRENT_SYMBOL.replace(Some((self.position, self.state)));
        self.children.render_to(buffer);
        CURRENT_SYMBOL.set(outer);
    }
}
