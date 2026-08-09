use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{CALLOUT, CALLOUT_ICON, CALLOUT_MESSAGE};
use crate::variant::Variant;

#[derive(AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = CALLOUT)]
#[props(builder)]
pub struct Callout<'a, I: Renderable = ()> {
    #[prop(impl_from)]
    pub variant: Variant,

    #[prop(impl_from)]
    pub appearance: Appearance,

    /// Render `children` as the body markup as is, without the message container.
    /// The icon container is still emitted when the `icon` prop is set explicitly.
    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    #[prop(convert)]
    pub icon: Option<I>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a, I: Renderable> Default for Callout<'a, I> {
    fn default() -> Self {
        Self {
            variant: Variant::Brand,
            appearance: Appearance::FilledOutlined,
            bare: false,
            attributes: CommonAttrs::default(),
            icon: None,
            children: None,
        }
    }
}

impl<'a, I: Renderable> Callout<'a, I> {
    fn render_to(&self, buffer: &mut Buffer, icon: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS, self.variant.into_str(), self.appearance.into_str()]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if let Some(icon) = icon {
                    <div class=CALLOUT_ICON>(icon)</div>
                }
                @if self.bare {
                    (self.children)
                } @else {
                    <div class=CALLOUT_MESSAGE>
                        (self.children)
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}
