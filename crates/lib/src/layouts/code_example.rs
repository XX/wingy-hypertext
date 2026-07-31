use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::class::{
    CODE_EXAMPLE, CODE_EXAMPLE_BUTTONS, CODE_EXAMPLE_PREVIEW, CODE_EXAMPLE_RESIZER, CODE_EXAMPLE_SOURCE,
    CODE_EXAMPLE_TOGGLE, DARK, ICON, NO_ANIMATION, OPEN,
};
use crate::components::copy_button::CopyButton;

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = CODE_EXAMPLE)]
#[props(builder)]
pub struct CodeExample<'a> {
    pub open: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for CodeExample<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let classes = [Self::CLASS, if self.open { OPEN } else { "" }];
        let class_line = self.class_line_with(&classes);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = CODE_EXAMPLE_PREVIEW)]
#[props(builder)]
pub struct CodeExamplePreview<'a> {
    pub resize: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for CodeExamplePreview<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
                @if self.resize {
                    <div class=CODE_EXAMPLE_RESIZER>
                        <span class=ICON>
                            (fontawesome_ext::regular::GripLinesVertical)
                        </span>
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = CODE_EXAMPLE_SOURCE)]
#[props(builder)]
pub struct CodeExampleSource<'a> {
    #[prop(into)]
    pub code_block_id: Option<Cow<'static, str>>,

    pub copy_button: bool,

    pub is_not_animated: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for CodeExampleSource<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS, if self.is_not_animated { NO_ANIMATION } else { "" }]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            @let code_block_id = self
                .code_block_id
                .clone()
                .or_else(|| id.map(|id| Cow::Owned(format!("{id}-code-block"))));

            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <pre id=[code_block_id.as_ref()]>
                    (self.children)
                    @if self.copy_button {
                        <CopyButton class=DARK from=(code_block_id.unwrap_or_default()) />
                    }
                </pre>
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = CODE_EXAMPLE_BUTTONS)]
#[props(builder)]
pub struct CodeExampleButton<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for CodeExampleButton<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <button class=CODE_EXAMPLE_TOGGLE type="button">
                    (self.children)
                    " "
                    <span class=ICON>
                        (fontawesome_ext::regular::ChevronDown)
                    </span>
                </button>
            </div>
        }
        .render_to(buffer);
    }
}
