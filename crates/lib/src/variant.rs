use strum::{AsRefStr, IntoStaticStr};

use crate::attributes::CommonAttributeSetters;

#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Variant {
    #[default]
    Neutral,
    Brand,
    Success,
    Warning,
    Danger,
}

impl Variant {
    pub const NEUTRAL: &str = Self::Neutral.into_str();
    pub const BRAND: &str = Self::Brand.into_str();
    pub const SUCCESS: &str = Self::Success.into_str();
    pub const WARNING: &str = Self::Warning.into_str();
    pub const DANGER: &str = Self::Danger.into_str();
}

pub trait VariantConstructor {
    fn neutral() -> Self;
    fn brand() -> Self;
    fn success() -> Self;
    fn warning() -> Self;
    fn danger() -> Self;
}

impl<T: From<Variant>> VariantConstructor for T {
    fn neutral() -> Self {
        Variant::Neutral.into()
    }

    fn brand() -> Self {
        Variant::Brand.into()
    }

    fn success() -> Self {
        Variant::Success.into()
    }

    fn warning() -> Self {
        Variant::Warning.into()
    }

    fn danger() -> Self {
        Variant::Danger.into()
    }
}

pub trait VariantSetters {
    fn variant(mut self, variant: Variant) -> Self
    where
        Self: Sized,
    {
        self.set_variant(variant);
        self
    }

    fn set_variant(&mut self, variant: Variant);
}

pub trait UseVariant {}

impl<'a, T: CommonAttributeSetters<'a> + UseVariant> VariantSetters for T {
    fn set_variant(&mut self, variant: Variant) {
        self.add_class(variant.into_str());
    }
}
