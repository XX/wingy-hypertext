use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{PILL, TAG, TAG_CONTENT, TAG_REMOVE};
use crate::variant::Variant;

/// A compact visual marker: use it for status indicators, filters, or removable
/// chips. With `with_remove`, the tag gets a remove button; activating it emits
/// a bubbling `wg-remove` event on the tag (the tag does not remove itself —
/// handle the event to decide). The event is dispatched by `wingy-hypertext-web`,
/// wired up on the client with `listen_remove_tags`.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = TAG)]
#[props(builder)]
pub struct Tag<'a> {
    #[prop(impl_from)]
    pub variant: Variant,

    #[prop(impl_from)]
    pub appearance: Appearance,

    pub pill: bool,

    /// Makes the tag removable and shows a remove button.
    pub with_remove: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Tag<'a> {
    fn default() -> Self {
        Self {
            variant: Variant::Neutral,
            appearance: Appearance::FilledOutlined,
            pill: false,
            with_remove: false,
            attrs: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Tag<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.pill { PILL } else { "" },
            self.variant.into_str(),
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line]>
                <span class=TAG_CONTENT>(self.children)</span>
                @if self.with_remove {
                    <button class=TAG_REMOVE type="button" tabindex="-1" aria-label="Remove">
                        (fontawesome::solid::Xmark)
                    </button>
                }
            </span>
        }
        .render_to(buffer);
    }
}
