use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::action::Action;
use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::link::Link;
use crate::variant::Variant;

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "button")]
#[props(builder)]
pub struct Button<R: Renderable = ()> {
    #[prop(from)]
    pub variant: Variant,

    #[prop(from)]
    pub appearance: Appearance,

    pub pill: bool,

    pub loading: bool,

    pub disabled: bool,

    #[as_ref]
    #[as_mut]
    pub link: Link,

    #[as_ref]
    #[as_mut]
    pub action_data: Action,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for Button<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.pill { "pill" } else { "" },
            self.variant.into_str(),
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with([]);

        if let Some(href) = &self.link.href {
            rsx! {
                <a
                    id=[id]
                    class=[&class_line]
                    style=[&style_line]
                    href=(href)
                    target=[&self.link.target]
                    download=[&self.link.download]
                    rel=[&self.link.rel]
                    data-action=[&self.action_data.action]
                    data-args=[&self.action_data.args]
                >
                    (self.children)
                </a>
            }
            .render_to(buffer);
        } else {
            let disabled = self.disabled.then_some(true);
            rsx! {
                <button
                    id=[id]
                    class=[&class_line]
                    style=[&style_line]
                    disabled=[disabled]
                    data-action=[&self.action_data.action]
                    data-args=[&self.action_data.args]
                >
                    (self.children)
                </button>
            }
            .render_to(buffer);
        }
    }
}
