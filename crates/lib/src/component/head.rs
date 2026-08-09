use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    ANCHOR_HEAD, HEAD, HEADING_2XL, HEADING_2XS, HEADING_3XL, HEADING_3XS, HEADING_4XL, HEADING_5XL, HEADING_L,
    HEADING_M, HEADING_S, HEADING_XL, HEADING_XS, ICON, ICON_SHRINK, VISUALLY_HIDDEN,
};
use crate::link::{Link, LinkSetters};

#[derive(Default, AsRef, AsMut, Props)]
#[props(builder)]
pub struct Anchor<'a> {
    #[as_ref]
    #[as_mut]
    pub link: Link,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl Renderable for Anchor<'_> {
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
                (self.get_attrs())
            >
                <span class=VISUALLY_HIDDEN>"Jump to heading"</span>
                <span class=(ICON, " ", ICON_SHRINK)>
                    (fontawesome_ext::regular::Hashtag)
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

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = HEAD)]
#[props(builder)]
pub struct Head<'a> {
    pub anchor: bool,

    #[prop(impl_from)]
    pub level: HeadLevel,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Head<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.level.class(),
            if self.anchor { ANCHOR_HEAD } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
                @if self.anchor {
                    @let href = format!("#{}", id.map(|id| id.as_ref()).unwrap_or_default());

                    <Anchor href />
                }
            </div>
        }
        .render_to(buffer);
    }
}
