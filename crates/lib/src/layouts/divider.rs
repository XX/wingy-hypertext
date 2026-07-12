use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{DIVIDER, VERTICAL};
use crate::orientation::Orientation;

/// A horizontal or vertical separator line that visually separates or groups
/// adjacent elements. The line's look is adjusted with the `--color`, `--width`
/// and `--spacing` custom properties.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIVIDER)]
#[props(builder)]
pub struct Divider {
    #[prop(impl_from)]
    pub orientation: Orientation,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,
}

impl Renderable for Divider {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.orientation == Orientation::Vertical {
                VERTICAL
            } else {
                ""
            },
        ]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="separator"
                aria-orientation=(self.orientation.into_str())
            >
            </div>
        }
        .render_to(buffer);
    }
}
