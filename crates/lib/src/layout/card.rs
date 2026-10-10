use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CARD, CARD_ACTIONS, CARD_BODY, CARD_FOOTER, CARD_FOOTER_ACTIONS, CARD_HEADER, CARD_HEADER_ACTIONS, CARD_MEDIA,
};
use crate::div_component;
use crate::orientation::Orientation;

/// A bordered container grouping related content and actions: a product, an
/// article, a user profile or any self-contained unit of information. Its
/// children are the card's sections, in this order:
///
/// ```ignore
/// rsx! {
///     <Card>
///         <CardMedia><img src="…" alt="…"/></CardMedia>
///         <CardHeader>
///             <h3>"Title"</h3>
///             <CardHeaderActions>…</CardHeaderActions>
///         </CardHeader>
///         <CardBody>"The card's main content."</CardBody>
///         <CardFooter>
///             …
///             <CardFooterActions>…</CardFooterActions>
///         </CardFooter>
///     </Card>
/// }
/// ```
///
/// Every section is optional and is simply left out when unused: unlike
/// `wa-card` there are no `with-*` flags, the styles follow the sections that
/// are present. A horizontal card is composed of [`CardMedia`], [`CardBody`]
/// and [`CardActions`]. The size follows the card's font size, e.g. a
/// `wa-size-*` class.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = CARD)]
#[props(builder)]
pub struct Card<'a> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    pub orientation: Orientation,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Card<'a> {
    fn default() -> Self {
        Self {
            appearance: Appearance::Outlined,
            orientation: Orientation::Vertical,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Card<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.appearance.into_str(),
            self.orientation.as_horizontal_class(),
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

div_component!(
    /// The card's media: an image or a video stretched to the card's width,
    /// rendered at the start of the card.
    CardMedia,
    CARD_MEDIA
);

div_component!(
    /// The card's header, e.g. a title. Put a [`CardHeaderActions`] last to
    /// push the actions to the end of the header.
    CardHeader,
    CARD_HEADER
);

div_component!(
    /// The actions in the card's header.
    CardHeaderActions,
    CARD_HEADER_ACTIONS
);

div_component!(
    /// The card's main content.
    CardBody,
    CARD_BODY
);

div_component!(
    /// The card's footer. Put a [`CardFooterActions`] last to push the actions
    /// to the end of the footer.
    CardFooter,
    CARD_FOOTER
);

div_component!(
    /// The actions in the card's footer.
    CardFooterActions,
    CARD_FOOTER_ACTIONS
);

div_component!(
    /// The actions at the end of a horizontal card.
    CardActions,
    CARD_ACTIONS
);
