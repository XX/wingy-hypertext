use derive_more::{AsMut, AsRef};
use hypertext::prelude::{
    AriaAttributes, GlobalAttributes, SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements,
};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{PILL, TAG, TAG_CONTENT, TAG_REMOVE};
use crate::variant::Variant;

/// A compact visual marker mirroring Web Awesome's `wa-tag`: use it for status
/// indicators, filters, or removable chips. With `with_remove`, the tag gets a
/// remove button; activating it emits a bubbling `wa-remove` event on the tag
/// (the tag does not remove itself — handle the event to decide). The event is
/// dispatched by `wingy-hypertext-web` (`components::tag`), wired up on the
/// client with `listen_remove_tags`.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = TAG)]
#[props(builder)]
pub struct Tag<R: Renderable = ()> {
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

    #[prop(convert)]
    pub children: Option<R>,
}

// Manual impl: unlike the enum default (`Accent`), a tag defaults to the
// `filled-outlined` appearance, as in Web Awesome.
impl<R: Renderable> Default for Tag<R> {
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

impl<R: Renderable> Renderable for Tag<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.pill { PILL } else { "" },
            self.variant.into_str(),
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with([]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line]>
                <span class=TAG_CONTENT>(self.children)</span>
                @if self.with_remove {
                    <button class=TAG_REMOVE type="button" tabindex="-1" aria-label="Remove">
                        // Xmark icon
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512">
                            // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                            // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                            <path fill="currentColor" d="M55.1 73.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3L147.2 256 9.9 393.4c-12.5 12.5-12.5 32.8 0 45.3s32.8 12.5 45.3 0L192.5 301.3 329.9 438.6c12.5 12.5 32.8 12.5 45.3 0s12.5-32.8 0-45.3L237.8 256 375.1 118.6c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L192.5 210.7 55.1 73.4z"/>
                        </svg>
                    </button>
                }
            </span>
        }
        .render_to(buffer);
    }
}
