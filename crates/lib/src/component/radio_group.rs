use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{HINT, LABEL, RADIO_GROUP, RADIO_GROUP_ITEMS, REQUIRED};
use crate::orientation::Orientation;

/// A set of [`Radio`](super::radio::Radio)s presented as one control, with a
/// label and an optional hint. It is a native `<fieldset>`, so `disabled`
/// disables every radio inside, and the arrow keys move between them on their
/// own.
///
/// The group's size is set with a `size-*` class, and it applies to the radios
/// inside:
///
/// ```ignore
/// rsx! {
///     <RadioGroup label="Color scheme" hint="Choose how the interface should appear." class=SIZE_SMALL>
///         <Radio name="scheme" value="light">"Light"</Radio>
///         <Radio name="scheme" value="dark">"Dark"</Radio>
///     </RadioGroup>
/// }
/// ```
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = RADIO_GROUP)]
#[props(builder)]
pub struct RadioGroup<'a> {
    #[prop(impl_from)]
    pub orientation: Orientation,

    /// Renders the required marker next to the label. The validation itself is
    /// native, so the radios of the group carry `required` as well.
    pub required: bool,

    /// Disables the whole group, the radios inside included.
    pub disabled: bool,

    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The radios of the group.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for RadioGroup<'a> {
    fn default() -> Self {
        Self {
            orientation: Orientation::Vertical,
            required: false,
            disabled: false,
            label: None,
            hint: None,
            bare: false,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for RadioGroup<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            self.orientation.into_str(),
            if self.required { REQUIRED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);
        let disabled = self.disabled.then_some(true);

        rsx! {
            <fieldset
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="radiogroup"
                aria-orientation=(self.orientation.into_str())
                disabled=[disabled]
                (self.get_attrs())
            >
                @if let Some(label) = &self.label {
                    <legend class=LABEL>(label)</legend>
                }
                @if self.bare {
                    (self.children)
                } @else {
                    <div class=RADIO_GROUP_ITEMS>
                        (self.children)
                    </div>
                }
                @if let Some(hint) = &self.hint {
                    <small class=HINT>(hint)</small>
                }
            </fieldset>
        }
        .render_to(buffer);
    }
}
