use std::borrow::Cow;
use std::ops::Deref;

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

    fn common_attrs_mut(&mut self) -> &mut CommonAttrs;

    fn set_id(&mut self, id: impl Into<Cow<'static, str>>) {
        self.common_attrs_mut().id = id.into();
    }

    fn set_classes(&mut self, classes: Vec<Cow<'static, str>>) {
        self.common_attrs_mut().classes = classes;
    }

    fn set_styles(&mut self, styles: Vec<Cow<'static, str>>) {
        self.common_attrs_mut().styles = styles;
    }

    fn add_class(&mut self, class: impl Into<Cow<'static, str>>) {
        self.common_attrs_mut().classes.push(class.into());
    }

    fn add_style(&mut self, style: impl Into<Cow<'static, str>>) {
        self.common_attrs_mut().styles.push(style.into());
    }
}

pub trait CommonAttributeGetters {
    fn id(&self) -> Option<&Cow<'static, str>> {
        self.get_id().into_not_empty()
    }

    fn class_line_with(&self, first_classes: &[&str]) -> Option<String> {
        join_not_empty(first_classes, self.get_classes(), " ")
    }

    fn style_line_with(&self, first_styles: &[&str]) -> Option<String> {
        join_not_empty(first_styles, self.get_styles(), "; ")
    }

    fn common_attrs_ref(&self) -> &CommonAttrs;

    fn get_id(&self) -> &Cow<'static, str> {
        &self.common_attrs_ref().id
    }

    fn get_classes(&self) -> &[Cow<'static, str>] {
        &self.common_attrs_ref().classes
    }

    fn get_styles(&self) -> &[Cow<'static, str>] {
        &self.common_attrs_ref().styles
    }

    fn get_class_line(&self) -> String {
        self.get_classes().join(" ")
    }

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
    fn common_attrs_mut(&mut self) -> &mut CommonAttrs {
        self
    }
}

impl CommonAttributeGetters for CommonAttrs {
    fn common_attrs_ref(&self) -> &CommonAttrs {
        self
    }
}

impl<T: AsMut<CommonAttrs>> CommonAttributeSetters for T {
    fn common_attrs_mut(&mut self) -> &mut CommonAttrs {
        self.as_mut()
    }
}

impl<T: AsRef<CommonAttrs>> CommonAttributeGetters for T {
    fn common_attrs_ref(&self) -> &CommonAttrs {
        self.as_ref()
    }
}

// Non-generic on purpose: a single compiled copy serves every component type, instead of one
// instantiation per component type and argument array length in the final binary.
fn join_not_empty(first: &[&str], rest: &[Cow<'static, str>], separator: &str) -> Option<String> {
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
