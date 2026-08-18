use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{CONTROL, DISABLED, HINT, LABEL, REQUIRED, SWITCH, SWITCH_TOGGLE, THUMB, TRACK};

/// A toggle control: a native checkbox with `role="switch"` wrapped in a label,
/// followed by an optional hint. Toggling is fully native (the checked state is
/// styled with `:checked`), so no client-side behavior is needed. The switch's
/// look is adjusted with the `--width`, `--height` and `--thumb-size` custom
/// properties.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = SWITCH)]
#[props(builder)]
pub struct Switch<'a> {
    pub checked: bool,

    pub disabled: bool,

    pub required: bool,

    /// Render `children` as the body markup as is, instead of treating them as
    /// the label text. Compose the body from a [`Toggle`] (which carries
    /// the control state props in this mode) and any custom hint markup.
    pub bare: bool,

    /// The name of the switch, submitted as a name/value pair with form data.
    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    /// The value of the switch, submitted as a name/value pair with form data.
    /// Defaults to the native checkbox value `on` when not set.
    #[prop(into)]
    pub value: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The switch's label, or the whole body markup when `bare` is set.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Switch<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.required { REQUIRED } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        let toggle = (!self.bare).then(|| Toggle {
            checked: self.checked,
            disabled: self.disabled,
            required: self.required,
            name: self.name.clone(),
            value: self.value.clone(),
            attributes: CommonAttrs::default(),
            children: self.children,
        });

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if self.bare {
                    (self.children)
                } @else {
                    (toggle)
                }
                @if let Some(hint) = &self.hint {
                    <small class=HINT>(hint)</small>
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// The label block of a [`Switch`]: the native checkbox control, the track
/// with the thumb, and the label content. Used standalone inside a bare
/// [`Switch`] to compose the body manually.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = SWITCH_TOGGLE)]
#[props(builder)]
pub struct Toggle<'a> {
    pub checked: bool,

    pub disabled: bool,

    pub required: bool,

    #[prop(into)]
    pub name: Option<Cow<'a, str>>,

    #[prop(into)]
    pub value: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Toggle<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let checked = self.checked.then_some(true);
        let disabled = self.disabled.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <label id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <input
                    class=CONTROL
                    type="checkbox"
                    role="switch"
                    name=[&self.name]
                    value=[&self.value]
                    checked=[checked]
                    disabled=[disabled]
                    required=[required]
                />
                <span class=TRACK>
                    <span class=THUMB></span>
                </span>
                <span class=LABEL>(self.children)</span>
            </label>
        }
        .render_to(buffer);
    }
}
