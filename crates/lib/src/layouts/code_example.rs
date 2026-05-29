use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::components::copy_button::CopyButton;

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "code-example")]
#[props(builder)]
pub struct CodeExample<R: Renderable = ()> {
    pub open: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for CodeExample<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let classes = [Self::CLASS, if self.open { "open" } else { "" }];
        let class_line = self.class_line_with(classes);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "code-example-preview")]
#[props(builder)]
pub struct CodeExamplePreview<R: Renderable = ()> {
    pub resize: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for CodeExamplePreview<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
                @if self.resize {
                    <div class=(CodeExample::class(), "-resizer")>
                        <span class="icon">
                            // grip-lines-vertical
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 192 512">
                                // ! Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M64 64c0-17.7-14.3-32-32-32S0 46.3 0 64L0 448c0 17.7 14.3 32 32 32s32-14.3 32-32L64 64zm128 0c0-17.7-14.3-32-32-32s-32 14.3-32 32l0 384c0 17.7 14.3 32 32 32s32-14.3 32-32l0-384z"></path>
                            </svg>
                        </span>
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "code-example-source")]
#[props(builder)]
pub struct CodeExampleSource<R: Renderable = ()> {
    #[prop(into)]
    pub code_block_id: Option<Cow<'static, str>>,

    pub copy_button: bool,

    pub is_not_animated: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for CodeExampleSource<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = if self.is_not_animated {
            self.class_line_with([Self::CLASS, "no-animation"])
        } else {
            self.class_line_with([Self::CLASS])
        };
        let style_line = self.style_line_with([]);

        rsx! {
            @let code_block_id = self
                .code_block_id
                .clone()
                .or_else(|| id.map(|id| Cow::Owned(format!("{id}-code-block"))));

            <div id=[id] class=[&class_line] style=[&style_line]>
                <pre id=[code_block_id.as_ref()]>
                    (self.children)
                    @if self.copy_button {
                        <CopyButton class="wa-dark" from=(code_block_id.unwrap_or_default()) />
                    }
                </pre>
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = "code-example-buttons")]
#[props(builder)]
pub struct CodeExampleButton<R: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Renderable for CodeExampleButton<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([Self::CLASS]);
        let style_line = self.style_line_with([]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                <button class=(CodeExample::class(), "-toggle") type="button">
                    (self.children)
                    " "
                    <span class="icon">
                        // chevron-down
                        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                            // ! Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                            // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                            <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"></path>
                        </svg>
                    </span>
                </button>
            </div>
        }
        .render_to(buffer);
    }
}
