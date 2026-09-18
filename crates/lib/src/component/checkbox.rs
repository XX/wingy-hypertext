use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CHECKBOX, CHECKBOX_CHECK, CHECKBOX_INDETERMINATE, CHECKBOX_INDICATOR, CHECKBOX_TOGGLE, CONTROL, HINT, LABEL,
};

/// A native checkbox wrapped in a label, with an optional hint below it, so
/// toggling and submitting the value are native. Checkboxes work on their own
/// and inside a [`CheckboxGroup`](super::checkbox_group::CheckboxGroup):
///
/// ```ignore
/// rsx! {
///     <Checkbox name="terms" value="accepted" required=true>"I agree to the terms"</Checkbox>
/// }
/// ```
///
/// The indeterminate state cannot be expressed in markup — it is a DOM property
/// rather than an attribute — so it is rendered as `data-indeterminate` and
/// applied on the client by `init_checkboxes` from `wingy-hypertext-web`
/// (`component::checkbox`). Clicking the checkbox clears it natively.
///
/// The structure mirrors [`Switch`](super::switch::Switch): the label holds the
/// control, and the hint follows it inside the wrapper.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = CHECKBOX)]
#[props(builder)]
pub struct Checkbox<'a> {
    pub checked: bool,

    /// Draws the checkbox in an indeterminate state, the one a "select
    /// all/none" checkbox takes while its group is partially selected. A
    /// `checked` checkbox is never indeterminate.
    pub indeterminate: bool,

    pub disabled: bool,

    pub required: bool,

    /// The name of the checkbox, submitted as a name/value pair with form data.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// The value submitted when the checkbox is checked. Defaults to the native
    /// checkbox value `on` when not set.
    #[prop(into)]
    pub value: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The checkbox's label.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Checkbox<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let checked = self.checked.then_some(true);
        let disabled = self.disabled.then_some(true);
        let required = self.required.then_some(true);
        let indeterminate = (!self.checked && self.indeterminate).then_some("");

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <label class=CHECKBOX_TOGGLE>
                    <input
                        class=CONTROL
                        type="checkbox"
                        name=[&self.name]
                        value=[&self.value]
                        checked=[checked]
                        disabled=[disabled]
                        required=[required]
                        data-indeterminate=[indeterminate]
                    />
                    <span class=CHECKBOX_INDICATOR>
                        <span class=CHECKBOX_CHECK>
                            (fontawesome::solid::Check)
                        </span>
                        <span class=CHECKBOX_INDETERMINATE>
                            (fontawesome::solid::Minus)
                        </span>
                    </span>
                    <span class=LABEL>(self.children)</span>
                </label>
                @if let Some(hint) = &self.hint {
                    <small class=HINT>(hint)</small>
                }
            </div>
        }
        .render_to(buffer);
    }
}
