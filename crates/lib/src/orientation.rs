use strum::{AsRefStr, IntoStaticStr};

use crate::attributes::CommonAttributeSetters;

#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum Orientation {
    #[default]
    Horizontal,
    Vertical,
}

impl Orientation {
    pub const HORIZONTAL: &str = Self::Horizontal.into_str();
    pub const VERTICAL: &str = Self::Vertical.into_str();
}

pub trait OrientationConstructor {
    fn horizontal() -> Self;
    fn vertical() -> Self;
}

impl<T: From<Orientation>> OrientationConstructor for T {
    fn horizontal() -> Self {
        Orientation::Horizontal.into()
    }

    fn vertical() -> Self {
        Orientation::Vertical.into()
    }
}

pub trait OrientationSetters {
    fn variant(mut self, orientation: Orientation) -> Self
    where
        Self: Sized,
    {
        self.set_variant(orientation);
        self
    }

    fn set_variant(&mut self, orientation: Orientation);
}

pub trait UseOrientation {}

impl<T: CommonAttributeSetters + UseOrientation> OrientationSetters for T {
    fn set_variant(&mut self, orientation: Orientation) {
        self.add_class(orientation.into_str());
    }
}
