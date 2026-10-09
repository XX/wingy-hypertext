use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    ANCHOR_HEAD, HEAD, HEADING, HEADING_2XL, HEADING_2XS, HEADING_3XL, HEADING_3XS, HEADING_4XL, HEADING_5XL,
    HEADING_L, HEADING_M, HEADING_S, HEADING_XL, HEADING_XS, ICON, ICON_SHRINK, VISUALLY_HIDDEN,
};
use crate::link::{Link, LinkSetters};

#[derive(Default, AsRef, AsMut, Props)]
#[props(builder)]
pub struct Anchor<'a> {
    #[as_ref]
    #[as_mut]
    pub link: Link<'a>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl Renderable for Anchor<'_> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
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

/// The semantic level of a heading: the `<h1>`–`<h6>` tag it renders, which
/// places it in the document outline that screen reader users navigate by.
/// It doesn't choose the look on its own: the native styles size each level,
/// and [`HeadSize`] overrides that without changing the level.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum HeadLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    /// Not a heading: text styled like one, outside the document outline.
    NoHeading,
}

/// The visual size of a heading, a `wa-heading-*` class, independent of its level.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum HeadSize {
    XS3,
    XS2,
    XS,
    S,
    M,
    L,
    XL,
    XL2,
    XL3,
    XL4,
    XL5,
}

impl HeadSize {
    pub const fn class(self) -> &'static str {
        match self {
            Self::XS3 => HEADING_3XS,
            Self::XS2 => HEADING_2XS,
            Self::XS => HEADING_XS,
            Self::S => HEADING_S,
            Self::M => HEADING_M,
            Self::L => HEADING_L,
            Self::XL => HEADING_XL,
            Self::XL2 => HEADING_2XL,
            Self::XL3 => HEADING_3XL,
            Self::XL4 => HEADING_4XL,
            Self::XL5 => HEADING_5XL,
        }
    }
}

/// A heading, rendered as the native `<h1>`–`<h6>` of its `level`: the native
/// styles size each level, and `size` restyles it without touching the outline.
/// With [`HeadLevel::NoHeading`] it is a `<div>` styled like a heading.
///
/// With `anchor`, a "jump to heading" link to its own id follows the text.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = HEAD)]
#[props(builder)]
pub struct Head<'a> {
    pub anchor: bool,

    #[prop(impl_from)]
    pub level: HeadLevel,

    pub size: Option<HeadSize>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Head<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let size_class = match (self.size, self.level) {
            (Some(size), _) => size.class(),
            // A `<div>` has no native heading styles to fall back on
            (None, HeadLevel::NoHeading) => HEADING,
            (None, _) => "",
        };
        let class_line = self.class_line_with(&[Self::CLASS, size_class, if self.anchor { ANCHOR_HEAD } else { "" }]);
        let style_line = self.style_line_with(&[]);

        let content = rsx! {
            (self.children)
            @if self.anchor {
                @let href = format!("#{}", id.map(|id| id.as_ref()).unwrap_or_default());

                <Anchor href />
            }
        };

        rsx! {
            @match self.level {
                HeadLevel::H1 => <h1 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h1>,
                HeadLevel::H2 => <h2 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h2>,
                HeadLevel::H3 => <h3 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h3>,
                HeadLevel::H4 => <h4 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h4>,
                HeadLevel::H5 => <h5 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h5>,
                HeadLevel::H6 => <h6 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</h6>,
                HeadLevel::NoHeading => <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>(content)</div>,
            }
        }
        .render_to(buffer);
    }
}
