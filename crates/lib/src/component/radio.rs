use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{CONTROL, LABEL, RADIO, RADIO_BUTTON, RADIO_INDICATOR};

/// The radio's visual appearance.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum RadioAppearance {
    /// A circle next to the label.
    #[default]
    Default,
    /// A button-like box, which turns a [`RadioGroup`](super::radio_group::RadioGroup)
    /// of radios into a segmented control.
    Button,
}

impl RadioAppearance {
    pub const fn as_class(&self) -> &'static str {
        match self {
            Self::Default => "",
            Self::Button => RADIO_BUTTON,
        }
    }
}

/// A single choice of a [`RadioGroup`](super::radio_group::RadioGroup): a
/// native radio button wrapped in a label, so selecting, moving between the
/// choices with the arrow keys and submitting the value are all native.
///
/// The radios of one group are tied together by their `name`, and the selected
/// one carries `checked` — the group doesn't set either, since it only sees its
/// children as opaque markup:
///
/// ```ignore
/// rsx! {
///     <RadioGroup label="Color scheme">
///         <Radio name="scheme" value="light">"Light"</Radio>
///         <Radio name="scheme" value="dark" checked=true>"Dark"</Radio>
///     </RadioGroup>
/// }
/// ```
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = RADIO)]
#[props(builder)]
pub struct Radio<'a> {
    #[prop(impl_from)]
    pub appearance: RadioAppearance,

    pub checked: bool,

    pub disabled: bool,

    /// Marks the group of radios sharing this `name` as required. Browsers
    /// accept it on a single radio of the group, but setting it on every one
    /// keeps the markup obvious.
    pub required: bool,

    /// The name of the group, submitted as a name/value pair with form data.
    /// Radios sharing a name are mutually exclusive.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// The value submitted when this radio is selected.
    #[prop(into)]
    pub value: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The radio's label.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Radio<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS, self.appearance.as_class()]);
        let style_line = self.style_line_with(&[]);

        let checked = self.checked.then_some(true);
        let disabled = self.disabled.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <label id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <input
                    class=CONTROL
                    type="radio"
                    name=[&self.name]
                    value=[&self.value]
                    checked=[checked]
                    disabled=[disabled]
                    required=[required]
                />
                <span class=RADIO_INDICATOR></span>
                <span class=LABEL>(self.children)</span>
            </label>
        }
        .render_to(buffer);
    }
}
