use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{BUTTON_GROUP, VERTICAL};
use crate::orientation::Orientation;

/// A group of related buttons — a toolbar, a split button or any set of actions
/// that belong together. The buttons keep their own props; the group only lays
/// them out, collapsing the borders between neighbours and rounding the outer
/// edges of the first and the last one:
///
/// ```ignore
/// rsx! {
///     <ButtonGroup label="Alignment">
///         <Button appearance=Filled>"Left"</Button>
///         <Button appearance=Filled>"Center"</Button>
///         <Button appearance=Filled>"Right"</Button>
///     </ButtonGroup>
/// }
/// ```
///
/// Anything that renders a button works as a child, a
/// [`Dropdown`](crate::component::dropdown::Dropdown) with a button trigger
/// included, since the group passes its layout down through inherited custom
/// properties.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = BUTTON_GROUP)]
#[props(builder)]
pub struct ButtonGroup<'a> {
    /// A label for the group, announced by assistive devices. It is not
    /// displayed, but every group should carry one.
    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[prop(impl_from)]
    pub orientation: Orientation,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for ButtonGroup<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.orientation == Orientation::Vertical {
                VERTICAL
            } else {
                ""
            },
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="group"
                aria-label=[&self.label]
                aria-orientation=(self.orientation.into_str())
                (self.get_attrs())
            >
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
