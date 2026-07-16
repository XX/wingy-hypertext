use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{BADGE, PILL};
use crate::variant::Variant;

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = BADGE)]
#[props(builder)]
pub struct Badge<R: Renderable = ()> {
    #[prop(impl_from)]
    pub variant: Variant,

    #[prop(impl_from)]
    pub appearance: Appearance,

    pub pill: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for Badge<R> {
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
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
