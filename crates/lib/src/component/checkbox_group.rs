use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{CHECKBOX_GROUP, CHECKBOX_GROUP_ITEMS, HINT, LABEL, REQUIRED};
use crate::orientation::Orientation;

/// A set of [`Checkbox`](super::checkbox::Checkbox)es presented as one control,
/// with a label and an optional hint. It is a native `<fieldset>`, so `disabled`
/// disables every checkbox inside.
///
/// Unlike the choices of a [`RadioGroup`](super::radio_group::RadioGroup), the
/// items keep their own width, so a click lands on the label rather than on the
/// whole row. [`Switch`](super::switch::Switch)es can be grouped the same way:
///
/// ```ignore
/// rsx! {
///     <CheckboxGroup label="Notifications" hint="Choose what you want to hear about." class=SIZE_SMALL>
///         <Checkbox name="news" value="weekly" checked=true>"Weekly digest"</Checkbox>
///         <Checkbox name="news" value="offers">"Offers"</Checkbox>
///     </CheckboxGroup>
/// }
/// ```
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = CHECKBOX_GROUP)]
#[props(builder)]
pub struct CheckboxGroup<'a> {
    #[prop(impl_from)]
    pub orientation: Orientation,

    /// Renders the required marker next to the label. The validation itself is
    /// native, so the checkboxes of the group carry `required` as well.
    pub required: bool,

    /// Disables the whole group, the checkboxes inside included.
    pub disabled: bool,

    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    /// Renders `children` as the body markup as is, instead of wrapping them
    /// into the element laying the items out.
    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The checkboxes of the group.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for CheckboxGroup<'a> {
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

impl<'a> Renderable for CheckboxGroup<'a> {
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
                role="group"
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
                    <div class=CHECKBOX_GROUP_ITEMS>
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
