use strum::{AsRefStr, IntoStaticStr};

use crate::attributes::CommonAttributeSetters;

#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Appearance {
    #[default]
    Accent,
    FilledOutlined,
    Filled,
    Outlined,
    Plain,
}

impl Appearance {
    pub const ACCENT: &str = Self::Accent.into_str();
    pub const FILLED_OUTLINED: &str = Self::FilledOutlined.into_str();
    pub const FILLED: &str = Self::Filled.into_str();
    pub const OUTLINED: &str = Self::Outlined.into_str();
    pub const PLAIN: &str = Self::Plain.into_str();
}

pub trait AppearanceConstructor {
    fn accent() -> Self;
    fn filled_outlined() -> Self;
    fn filled() -> Self;
    fn outlined() -> Self;
    fn plain() -> Self;
}

impl<T: From<Appearance>> AppearanceConstructor for T {
    fn accent() -> Self {
        Appearance::Accent.into()
    }

    fn filled_outlined() -> Self {
        Appearance::FilledOutlined.into()
    }

    fn filled() -> Self {
        Appearance::Filled.into()
    }

    fn outlined() -> Self {
        Appearance::Outlined.into()
    }

    fn plain() -> Self {
        Appearance::Plain.into()
    }
}

pub trait AppearanceSetters {
    fn appearance(mut self, appearance: Appearance) -> Self
    where
        Self: Sized,
    {
        self.set_appearance(appearance);
        self
    }

    fn set_appearance(&mut self, appearance: Appearance);
}

pub trait UseAppearance {}

impl<'a, T: CommonAttributeSetters<'a> + UseAppearance> AppearanceSetters for T {
    fn set_appearance(&mut self, appearance: Appearance) {
        self.add_class(appearance.into_str());
    }
}
