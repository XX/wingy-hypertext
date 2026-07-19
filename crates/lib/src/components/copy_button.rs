use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::action::ActionSetters;
use crate::appearance::Appearance::Plain;
use crate::attributes::{CommonAttributeSetters, CommonAttrs};
use crate::class::{COPY_BUTTON, COPY_BUTTON_COPY, COPY_BUTTON_ERROR, COPY_BUTTON_SUCCESS, ICON};
use crate::components::button::Button;

#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = COPY_BUTTON)]
#[props(builder)]
pub struct CopyButton<R: Renderable = ()> {
    pub disabled: bool,

    #[prop(into)]
    pub from: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> CopyButton<R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let content = rsx! {
            @if let Some(children) = children {
                (children)
            }
            @else {
                <span class=(ICON, " ", COPY_BUTTON_COPY)>
                    (fontawesome::regular::Copy)
                </span>
                <span class=(ICON, " ", COPY_BUTTON_SUCCESS) hidden>
                    (fontawesome::solid::Check)
                </span>
                <span class=(ICON, " ", COPY_BUTTON_ERROR) hidden>
                    (fontawesome::solid::Xmark)
                </span>
            }
        };

        rsx! {
            @let classes = {
                let mut classes = self.attrs.classes.clone();
                classes.insert(0, Self::CLASS.into());
                classes
            };
            @let from = format!(r#"{{"from":"{}"}}"#, self.from.as_deref().unwrap_or(""));

            <Button attrs=(self.attrs.clone()) classes appearance=Plain disabled=(self.disabled) action="copy" args=from>
                (content)
            </Button>
        }
        .render_to(buffer);
    }
}
