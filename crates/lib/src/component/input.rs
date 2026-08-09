use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{CONTROL, HINT, INPUT, LABEL, PILL, REQUIRED, TEXT_FIELD};

/// The type of data the input collects. Mirrors a subset of the native `<input>` `type` attribute.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum InputType {
    #[default]
    Text,
    Email,
    Number,
    Password,
    Search,
    Tel,
    Url,
    Date,
    DatetimeLocal,
    Time,
}

impl InputType {
    pub const TEXT: &str = Self::Text.into_str();
    pub const EMAIL: &str = Self::Email.into_str();
    pub const NUMBER: &str = Self::Number.into_str();
    pub const PASSWORD: &str = Self::Password.into_str();
    pub const SEARCH: &str = Self::Search.into_str();
    pub const TEL: &str = Self::Tel.into_str();
    pub const URL: &str = Self::Url.into_str();
    pub const DATE: &str = Self::Date.into_str();
    pub const DATETIME_LOCAL: &str = Self::DatetimeLocal.into_str();
    pub const TIME: &str = Self::Time.into_str();
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = INPUT)]
#[props(builder)]
pub struct Input<'a> {
    #[prop(impl_from)]
    pub input_type: InputType,

    #[prop(impl_from)]
    pub appearance: Appearance,

    pub pill: bool,

    pub autofocus: bool,

    pub disabled: bool,

    pub readonly: bool,

    pub required: bool,

    #[prop(into)]
    pub name: Option<Cow<'static, str>>,

    #[prop(into)]
    pub value: Option<Cow<'static, str>>,

    #[prop(into)]
    pub placeholder: Option<Cow<'static, str>>,

    #[prop(into)]
    pub label: Option<Cow<'static, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Input<'a> {
    fn has_text_field_props(&self) -> bool {
        self.input_type != InputType::default()
            || self.disabled
            || self.readonly
            || self.required
            || self.name.is_some()
            || self.value.is_some()
            || self.placeholder.is_some()
    }
}

impl<'a> Renderable for Input<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.pill { PILL } else { "" },
            if self.required { REQUIRED } else { "" },
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        let text_field = (self.children.is_none() || self.has_text_field_props()).then(|| TextField {
            input_type: self.input_type,
            autofocus: self.autofocus,
            disabled: self.disabled,
            readonly: self.readonly,
            required: self.required,
            name: self.name.clone(),
            value: self.value.clone(),
            placeholder: self.placeholder.clone(),
            ..TextField::default()
        });

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if let Some(label) = &self.label {
                    <label class=LABEL>(label)</label>
                }
                (text_field)
                (self.children)
                @if let Some(hint) = &self.hint {
                    <small class=HINT>(hint)</small>
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// The field box wrapping the native `<input>` control.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = TEXT_FIELD)]
#[props(builder)]
pub struct TextField<'a> {
    #[prop(impl_from)]
    pub input_type: InputType,

    pub autofocus: bool,

    pub disabled: bool,

    pub readonly: bool,

    pub required: bool,

    #[prop(into)]
    pub name: Option<Cow<'static, str>>,

    #[prop(into)]
    pub value: Option<Cow<'static, str>>,

    #[prop(into)]
    pub placeholder: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl Renderable for TextField<'_> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let autofocus = self.autofocus.then_some(true);
        let disabled = self.disabled.then_some(true);
        let readonly = self.readonly.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                <input
                    class=CONTROL
                    type=(self.input_type.into_str())
                    name=[&self.name]
                    value=[&self.value]
                    placeholder=[&self.placeholder]
                    autofocus=[autofocus]
                    disabled=[disabled]
                    readonly=[readonly]
                    required=[required]
                    (self.get_attrs())
                />
            </div>
        }
        .render_to(buffer);
    }
}
