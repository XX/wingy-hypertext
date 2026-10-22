use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome_ext;
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance::Plain;
use crate::attributes::{CommonAttributeSetters, CommonAttrs};
use crate::attrs;
use crate::class::{COLOR_SCHEME_SELECTOR, ICON, SIZE_SMALL};
use crate::component::button::Button;
use crate::component::dropdown::{Dropdown, DropdownItem, DropdownItemIcon, DropdownItemLabel, DropdownMenu};
use crate::helper::popup::PopupPlacement::BottomEnd;
use crate::layout::divider::Divider;

#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum ColorScheme {
    Light,
    Dark,
    /// Follows the system preference.
    #[default]
    Auto,
}

impl ColorScheme {
    pub fn parse(value: &str) -> Option<Self> {
        match value {
            value if value == ColorScheme::Light.as_ref() => Some(Self::Light),
            value if value == ColorScheme::Dark.as_ref() => Some(Self::Dark),
            value if value == ColorScheme::Auto.as_ref() => Some(Self::Auto),
            _ => None,
        }
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = COLOR_SCHEME_SELECTOR)]
#[props(builder)]
pub struct ColorSchemeSelector<'a> {
    #[prop(into)]
    pub label_light: Option<Cow<'a, str>>,

    #[prop(into)]
    pub label_dark: Option<Cow<'a, str>>,

    #[prop(into)]
    pub label_auto: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl<'a> Renderable for ColorSchemeSelector<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let label_light = self.label_light.as_deref().unwrap_or("Light");
        let label_dark = self.label_dark.as_deref().unwrap_or("Dark");
        let label_auto = self.label_auto.as_deref().unwrap_or("System");

        rsx! {
            <Dropdown attributes=(self.attributes.clone()) class=(Self::CLASS) placement=BottomEnd>
                <Button
                    class=SIZE_SMALL
                    appearance=Plain
                    pill=true
                    attrs=(attrs!["title" = &"Toggle color scheme", "aria-label" = &"Toggle color scheme"])
                >
                    <span class=(ICON, " ", "only-light")>
                        (fontawesome_ext::regular::SunBright)
                    </span>
                    <span class=(ICON, " ", "only-dark")>
                        (fontawesome_ext::regular::MoonStars)
                    </span>
                </Button>
                <DropdownMenu>
                    <DropdownItem checkbox=true value=(ColorScheme::Light.as_ref())>
                        <DropdownItemIcon>(fontawesome_ext::regular::SunBright)</DropdownItemIcon>
                        <DropdownItemLabel>(label_light)</DropdownItemLabel>
                    </DropdownItem>
                    <DropdownItem checkbox=true value=(ColorScheme::Dark.as_ref())>
                        <DropdownItemIcon>(fontawesome_ext::regular::MoonStars)</DropdownItemIcon>
                        <DropdownItemLabel>(label_dark)</DropdownItemLabel>
                    </DropdownItem>
                    <Divider/>
                    <DropdownItem checkbox=true value=(ColorScheme::Auto.as_ref())>
                        <DropdownItemLabel>(label_auto)</DropdownItemLabel>
                    </DropdownItem>
                </DropdownMenu>
            </Dropdown>
        }
        .render_to(buffer);
    }
}
