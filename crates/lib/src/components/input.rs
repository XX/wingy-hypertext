use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};

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
#[const_str(CLASS = "input")]
#[props(builder)]
pub struct Input {
    #[prop(from)]
    pub input_type: InputType,

    #[prop(from)]
    pub appearance: Appearance,

    pub pill: bool,

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
    pub attrs: CommonAttrs,
}

impl Renderable for Input {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.pill { "pill" } else { "" },
            if self.required { "required" } else { "" },
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with([]);

        let disabled = self.disabled.then_some(true);
        let readonly = self.readonly.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                @if let Some(label) = &self.label {
                    <label class="label">(label)</label>
                }
                <div class="text-field">
                    <input
                        class="control"
                        type=(self.input_type.into_str())
                        name=[&self.name]
                        value=[&self.value]
                        placeholder=[&self.placeholder]
                        disabled=[disabled]
                        readonly=[readonly]
                        required=[required]
                    />
                </div>
                @if let Some(hint) = &self.hint {
                    <small class="hint">(hint)</small>
                }
            </div>
        }
        .render_to(buffer);
    }
}
