use std::borrow::Cow;
use std::ops::Deref;

use itertools::Itertools;

pub trait CommonAttributeSetters {
    fn id(mut self, id: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_id(id);
        self
    }

    fn class(mut self, class: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_class(class);
        self
    }

    fn classes(mut self, classes: impl Into<Vec<Cow<'static, str>>>) -> Self
    where
        Self: Sized,
    {
        self.set_classes(classes.into());
        self
    }

    fn style(mut self, style: impl Into<Cow<'static, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_style(style);
        self
    }

    fn set_id(&mut self, id: impl Into<Cow<'static, str>>);

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>);

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>);

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>);

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>);
}

pub trait CommonAttributeGetters {
    fn id(&self) -> Option<&Cow<'static, str>> {
        self.get_id().into_not_empty()
    }

    fn class_line_with<'a>(&'a self, first_classes: impl IntoIterator<Item = &'a str>) -> Option<String> {
        first_classes
            .into_iter()
            .chain(self.get_classes().iter().map(Deref::deref))
            .filter(|class| !class.is_empty())
            .join(" ")
            .into_not_empty()
    }

    fn style_line_with<'a>(&'a self, first_styles: impl IntoIterator<Item = &'a str>) -> Option<String> {
        first_styles
            .into_iter()
            .chain(self.get_styles().iter().map(Deref::deref))
            .filter(|style| !style.is_empty())
            .join("; ")
            .into_not_empty()
    }

    fn get_id(&self) -> &Cow<'static, str>;

    fn get_classes(&self) -> &[Cow<'static, str>];

    fn get_class_line(&self) -> String {
        self.get_classes().join(" ")
    }

    fn get_styles(&self) -> &[Cow<'static, str>];

    fn get_style_line(&self) -> String {
        self.get_styles().join("; ")
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CommonAttrs {
    pub id: Cow<'static, str>,
    pub classes: Vec<Cow<'static, str>>,
    pub styles: Vec<Cow<'static, str>>,
}

impl CommonAttrs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CommonAttributeSetters for CommonAttrs {
    fn set_id(&mut self, id: impl Into<Cow<'static, str>>) {
        self.id = id.into();
    }

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>) {
        self.classes = classes;
    }

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>) {
        self.styles = styles;
    }

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>) {
        self.classes.push(class.into());
    }

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>) {
        self.styles.push(style.into());
    }
}

impl CommonAttributeGetters for CommonAttrs {
    fn get_id(&self) -> &Cow<'static, str> {
        &self.id
    }

    fn get_classes(&self) -> &[Cow<'static, str>] {
        &self.classes
    }

    fn get_styles(&self) -> &[Cow<'static, str>] {
        &self.styles
    }
}

impl<T: AsMut<CommonAttrs>> CommonAttributeSetters for T {
    fn set_id(&mut self, id: impl Into<Cow<'static, str>>) {
        self.as_mut().set_id(id);
    }

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>) {
        self.as_mut().set_classes(classes);
    }

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>) {
        self.as_mut().set_styles(styles);
    }

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>) {
        self.as_mut().add_class(class);
    }

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>) {
        self.as_mut().add_style(style);
    }
}

impl<T: AsRef<CommonAttrs>> CommonAttributeGetters for T {
    fn get_id(&self) -> &Cow<'static, str> {
        self.as_ref().get_id()
    }

    fn get_classes(&self) -> &[Cow<'static, str>] {
        self.as_ref().get_classes()
    }

    fn get_styles(&self) -> &[Cow<'static, str>] {
        self.as_ref().get_styles()
    }
}

pub trait IntoNotEmpty: Sized {
    fn into_not_empty(self) -> Option<Self>;
}

impl<S: AsRef<str>> IntoNotEmpty for S {
    fn into_not_empty(self) -> Option<Self> {
        if self.as_ref().is_empty() { None } else { Some(self) }
    }
}
