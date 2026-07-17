use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    ANCHOR_HEAD, HEAD, HEADING_2XL, HEADING_2XS, HEADING_3XL, HEADING_3XS, HEADING_4XL, HEADING_5XL, HEADING_L,
    HEADING_M, HEADING_S, HEADING_XL, HEADING_XS, ICON, ICON_SHRINK, VISUALLY_HIDDEN,
};
use crate::link::{Link, LinkSetters};

#[derive(Default, AsRef, AsMut, Props)]
#[props(builder)]
pub struct Anchor {
    #[as_ref]
    #[as_mut]
    pub link: Link,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,
}

impl Renderable for Anchor {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <a
                id=[id]
                class=[&class_line]
                style=[&style_line]
                href=[&self.link.href]
                target=[&self.link.target]
                download=[&self.link.download]
                rel=[&self.link.rel]
            >
                <span class=VISUALLY_HIDDEN>"Jump to heading"</span>
                <span class=(ICON, " ", ICON_SHRINK)>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                        <path fill="currentColor" d="M226.9 29c2.8-13-5.5-25.7-18.4-28.5S182.7 6 180 18.9L156.6 128 68.5 128c-13.3 0-24 10.7-24 24s10.7 24 24 24l77.8 0-34.3 160-88 0c-13.3 0-24 10.7-24 24s10.7 24 24 24l77.7 0-21.2 99c-2.8 13 5.5 25.7 18.4 28.5s25.7-5.5 28.5-18.4l23.4-109 155.4 0-21.2 99c-2.8 13 5.5 25.7 18.4 28.5s25.7-5.5 28.5-18.4l23.4-109 88 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-77.7 0 34.3-160 88.1 0c13.3 0 24-10.7 24-24s-10.7-24-24-24l-77.8 0 21.2-99c2.8-13-5.5-25.7-18.4-28.5S387.2 6 384.4 18.9l-23.4 109.1-155.4 0 21.2-99zM195.4 176l155.4 0-34.3 160-155.4 0 34.3-160z"/>
                    </svg>
                </span>
            </a>
        }
        .render_to(buffer);
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum HeadLevel {
    HXL,
    XL5,

    HL,
    XL4,

    #[default]
    H1,
    XL3,

    H2,
    XL2,

    H3,
    XL,

    H4,
    L,

    H5,
    M,

    H6,
    S,

    HS,
    XS,

    HXS,
    XS2,

    HXXS,
    XS3,
}

impl HeadLevel {
    pub fn class(&self) -> &'static str {
        match self {
            Self::HXL | Self::XL5 => HEADING_5XL,
            Self::HL | Self::XL4 => HEADING_4XL,
            Self::H1 | Self::XL3 => HEADING_3XL,
            Self::H2 | Self::XL2 => HEADING_2XL,
            Self::H3 | Self::XL => HEADING_XL,
            Self::H4 | Self::L => HEADING_L,
            Self::H5 | Self::M => HEADING_M,
            Self::H6 | Self::S => HEADING_S,
            Self::HS | Self::XS => HEADING_XS,
            Self::HXS | Self::XS2 => HEADING_2XS,
            Self::HXXS | Self::XS3 => HEADING_3XS,
        }
    }
}

#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = HEAD)]
#[props(builder)]
pub struct Head<R: Renderable = ()> {
    pub anchor: bool,

    #[prop(impl_from)]
    pub level: HeadLevel,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Head<R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.level.class(),
            if self.anchor { ANCHOR_HEAD } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                (children)
                @if self.anchor {
                    @let href = format!("#{}", id.map(|id| id.as_ref()).unwrap_or_default());

                    <Anchor href />
                }
            </div>
        }
        .render_to(buffer);
    }
}
