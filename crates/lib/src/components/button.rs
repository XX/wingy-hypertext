use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, HtmxAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable};
use wingy_hypertext_macros::{Props, const_str, htmx_rsx};

use crate::action::Action;
use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{BUTTON, PILL};
use crate::htmx::Htmx;
use crate::link::Link;
use crate::renderable;
use crate::variant::Variant;

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = BUTTON)]
#[props(builder)]
pub struct Button<R: Renderable = ()> {
    #[prop(impl_from)]
    pub variant: Variant,

    #[prop(impl_from)]
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

    #[as_ref]
    #[as_mut]
    pub htmx: Htmx,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for Button<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let children = renderable::as_dyn(&self.children);
        self.render_to(buffer, children)
    }
}

impl<R: Renderable> Button<R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.pill { PILL } else { "" },
            self.variant.into_str(),
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        if let Some(href) = &self.link.href {
            htmx_rsx! {
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
                    htmx=[self.htmx]
                >
                    (children)
                </a>
            }
            .render_to(buffer);
        } else {
            let disabled = self.disabled.then_some(true);
            htmx_rsx! {
                <button
                    id=[id]
                    class=[&class_line]
                    style=[&style_line]
                    disabled=[disabled]
                    data-action=[&self.action_data.action]
                    data-args=[&self.action_data.args]
                    htmx=[self.htmx]
                >
                    (children)
                </button>
            }
            .render_to(buffer);
        }
    }
}
