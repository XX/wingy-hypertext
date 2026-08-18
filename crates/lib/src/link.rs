use std::borrow::Cow;

use hypertext::context::AttributeValue;
use hypertext::{Buffer, Renderable};

pub trait LinkSetters<'a> {
    fn href(mut self, href: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_href(href);
        self
    }

    fn target(mut self, target: Target) -> Self
    where
        Self: Sized,
    {
        self.set_target(target);
        self
    }

    fn download(mut self, download: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_download(download);
        self
    }

    fn rel(mut self, rel: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_rel(rel);
        self
    }

    fn link_mut(&mut self) -> &mut Link<'a>;

    fn set_href(&mut self, href: impl Into<Cow<'a, str>>) {
        self.link_mut().href = Some(href.into());
    }

    fn set_target(&mut self, target: Target) {
        self.link_mut().target = Some(target);
    }

    fn set_download(&mut self, download: impl Into<Cow<'a, str>>) {
        self.link_mut().download = Some(download.into());
    }

    fn set_rel(&mut self, rel: impl Into<Cow<'a, str>>) {
        self.link_mut().rel = Some(rel.into());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Target {
    Blank,
    Parent,
    Self_,
    Top,
}

impl Target {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Blank => "_blank",
            Self::Parent => "_parent",
            Self::Self_ => "_self",
            Self::Top => "_top",
        }
    }
}

impl Renderable<AttributeValue> for Target {
    fn render_to(&self, buffer: &mut Buffer<AttributeValue>) {
        self.as_str().render_to(buffer);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Link<'a> {
    pub href: Option<Cow<'a, str>>,
    pub target: Option<Target>,
    pub download: Option<Cow<'a, str>>,
    pub rel: Option<Cow<'a, str>>,
}

impl<'a> Link<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> LinkSetters<'a> for Link<'a> {
    fn link_mut(&mut self) -> &mut Link<'a> {
        self
    }
}

impl<'a, T: AsMut<Link<'a>>> LinkSetters<'a> for T {
    fn link_mut(&mut self) -> &mut Link<'a> {
        self.as_mut()
    }
}
