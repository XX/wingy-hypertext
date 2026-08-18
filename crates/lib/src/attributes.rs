use std::borrow::Cow;
use std::ops::Deref;

use hypertext::Renderable;
use hypertext::context::AttributeValue;

pub type NamedAttribute<'a> = hypertext::NamedAttribute<Cow<'a, str>, &'a dyn Renderable<AttributeValue>>;

#[macro_export]
macro_rules! attrs {
    ($($name:tt = $value:expr),*) => {
        [$($crate::attributes::NamedAttribute::new(($name).into(), ($value) as _)),*]
    };
}

pub trait CommonAttributeSetters<'a> {
    fn id(mut self, id: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.set_id(id);
        self
    }

    fn class(mut self, class: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_class(class);
        self
    }

    fn classes(mut self, classes: impl Into<Vec<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.set_classes(classes.into());
        self
    }

    fn style(mut self, style: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.add_style(style);
        self
    }

    fn attr(mut self, attr: impl Into<NamedAttribute<'a>>) -> Self
    where
        Self: Sized,
    {
        self.add_attr(attr);
        self
    }

    fn attrs(mut self, attrs: impl Into<Vec<NamedAttribute<'a>>>) -> Self
    where
        Self: Sized,
    {
        self.set_attrs(attrs.into());
        self
    }

    fn common_attrs_mut(&mut self) -> &mut CommonAttrs<'a>;

    fn set_id(&mut self, id: impl Into<Cow<'a, str>>) {
        self.common_attrs_mut().id = id.into();
    }

    fn set_classes(&mut self, classes: Vec<Cow<'a, str>>) {
        self.common_attrs_mut().classes = classes;
    }

    fn set_styles(&mut self, styles: Vec<Cow<'a, str>>) {
        self.common_attrs_mut().styles = styles;
    }

    fn set_attrs(&mut self, attrs: Vec<NamedAttribute<'a>>) {
        self.common_attrs_mut().attrs = attrs;
    }

    fn add_class(&mut self, class: impl Into<Cow<'a, str>>) {
        self.common_attrs_mut().classes.push(class.into());
    }

    fn add_style(&mut self, style: impl Into<Cow<'a, str>>) {
        self.common_attrs_mut().styles.push(style.into());
    }

    fn add_attr(&mut self, attr: impl Into<NamedAttribute<'a>>) {
        self.common_attrs_mut().attrs.push(attr.into());
    }
}

pub trait CommonAttributeGetters<'a> {
    fn id(&'a self) -> Option<&'a Cow<'a, str>> {
        self.get_id().into_not_empty()
    }

    fn class_line_with(&'a self, first_classes: &[&str]) -> Option<String> {
        join_not_empty(first_classes, self.get_classes(), " ")
    }

    fn style_line_with(&'a self, first_styles: &[&str]) -> Option<String> {
        join_not_empty(first_styles, self.get_styles(), "; ")
    }

    fn common_attrs_ref(&self) -> &CommonAttrs<'a>;

    fn get_id(&'a self) -> &'a Cow<'a, str> {
        &self.common_attrs_ref().id
    }

    fn get_classes(&'a self) -> &'a [Cow<'a, str>] {
        &self.common_attrs_ref().classes
    }

    fn get_styles(&'a self) -> &'a [Cow<'a, str>] {
        &self.common_attrs_ref().styles
    }

    fn get_class_line(&'a self) -> String {
        self.get_classes().join(" ")
    }

    fn get_style_line(&'a self) -> String {
        self.get_styles().join("; ")
    }

    fn get_attrs(&self) -> &[NamedAttribute<'a>] {
        &self.common_attrs_ref().attrs
    }

    fn get_attr(&self, name: Cow<'a, str>) -> Option<&'a dyn Renderable<AttributeValue>> {
        self.common_attrs_ref().attrs.iter().find_map(|attr| {
            if attr.name() == name {
                attr.value().copied()
            } else {
                None
            }
        })
    }
}

#[derive(Clone, Default)]
pub struct CommonAttrs<'a> {
    pub id: Cow<'a, str>,
    pub classes: Vec<Cow<'a, str>>,
    pub styles: Vec<Cow<'a, str>>,
    pub attrs: Vec<NamedAttribute<'a>>,
}

impl<'a> CommonAttrs<'a> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<'a> CommonAttributeSetters<'a> for CommonAttrs<'a> {
    fn common_attrs_mut(&mut self) -> &mut CommonAttrs<'a> {
        self
    }
}

impl<'a> CommonAttributeGetters<'a> for CommonAttrs<'a> {
    fn common_attrs_ref(&self) -> &CommonAttrs<'a> {
        self
    }
}

impl<'a, T: AsMut<CommonAttrs<'a>>> CommonAttributeSetters<'a> for T {
    fn common_attrs_mut(&mut self) -> &mut CommonAttrs<'a> {
        self.as_mut()
    }
}

impl<'a, T: AsRef<CommonAttrs<'a>>> CommonAttributeGetters<'a> for T {
    fn common_attrs_ref(&self) -> &CommonAttrs<'a> {
        self.as_ref()
    }
}

// Non-generic on purpose: a single compiled copy serves every component type, instead of one
// instantiation per component type and argument array length in the final binary.
fn join_not_empty(first: &[&str], rest: &[Cow<'_, str>], separator: &str) -> Option<String> {
    let mut line = String::new();
    for part in first.iter().copied().chain(rest.iter().map(Deref::deref)) {
        if part.is_empty() {
            continue;
        }
        if !line.is_empty() {
            line.push_str(separator);
        }
        line.push_str(part);
    }
    line.into_not_empty()
}

pub trait IntoNotEmpty: Sized {
    fn into_not_empty(self) -> Option<Self>;
}

impl<S: AsRef<str>> IntoNotEmpty for S {
    fn into_not_empty(self) -> Option<Self> {
        if self.as_ref().is_empty() { None } else { Some(self) }
    }
}
