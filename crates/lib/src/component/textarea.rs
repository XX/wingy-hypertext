use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CONTROL, HINT, LABEL, REQUIRED, RESIZE_AUTO, RESIZE_BOTH, RESIZE_HORIZONTAL, RESIZE_NONE, RESIZE_VERTICAL,
    TEXTAREA, TEXTAREA_COUNT, TEXTAREA_FIELD, TEXTAREA_FOOTER,
};
use crate::convert::utf16_len;

/// How a [`Textarea`] can be resized.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Resize {
    /// No resizing at all.
    None,
    #[default]
    Vertical,
    Horizontal,
    Both,
    /// The height follows the content. It is the one mode that needs the
    /// client-side behavior, since the height is measured in the browser.
    Auto,
}

impl Resize {
    pub const fn as_class(self) -> &'static str {
        match self {
            Self::None => RESIZE_NONE,
            Self::Vertical => RESIZE_VERTICAL,
            Self::Horizontal => RESIZE_HORIZONTAL,
            Self::Both => RESIZE_BOTH,
            Self::Auto => RESIZE_AUTO,
        }
    }
}

#[derive(Clone)]
pub struct TextareaParam<'a> {
    /// The number of rows the field shows before it scrolls or grows.
    pub rows: u32,

    pub autofocus: bool,

    pub disabled: bool,

    pub readonly: bool,

    pub required: bool,

    /// Shows the number of characters entered below the field, or how many are
    /// left when `maxlength` is set.
    pub count: bool,

    pub minlength: Option<u32>,

    pub maxlength: Option<u32>,

    pub name: Option<Cow<'a, str>>,

    /// The value of the field. Unlike an `<input>`, a `<textarea>` carries it as
    /// its content rather than as an attribute.
    pub value: Option<Cow<'a, str>>,

    pub placeholder: Option<Cow<'a, str>>,

    pub autocapitalize: Option<Cow<'a, str>>,

    pub autocomplete: Option<Cow<'a, str>>,

    pub inputmode: Option<Cow<'a, str>>,

    pub enterkeyhint: Option<Cow<'a, str>>,

    pub spellcheck: Option<Cow<'a, str>>,
}

impl<'a> Default for TextareaParam<'a> {
    fn default() -> Self {
        Self {
            rows: 4,
            autofocus: false,
            disabled: false,
            readonly: false,
            required: false,
            count: false,
            minlength: None,
            maxlength: None,
            name: None,
            value: None,
            placeholder: None,
            autocapitalize: None,
            autocomplete: None,
            inputmode: None,
            enterkeyhint: None,
            spellcheck: None,
        }
    }
}

pub trait TextareaParamBuilder<'a> {
    fn rows(mut self, rows: u32) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().rows = rows;
        self
    }

    fn autofocus(mut self, autofocus: bool) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().autofocus = autofocus;
        self
    }

    fn disabled(mut self, disabled: bool) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().disabled = disabled;
        self
    }

    fn readonly(mut self, readonly: bool) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().readonly = readonly;
        self
    }

    fn required(mut self, required: bool) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().required = required;
        self
    }

    fn count(mut self, count: bool) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().count = count;
        self
    }

    fn minlength(mut self, minlength: u32) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().minlength = Some(minlength);
        self
    }

    fn self_minlength(mut self, minlength: Option<u32>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().minlength = minlength;
        self
    }

    fn maxlength(mut self, maxlength: u32) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().maxlength = Some(maxlength);
        self
    }

    fn self_maxlength(mut self, maxlength: Option<u32>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().maxlength = maxlength;
        self
    }

    fn name(mut self, name: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().name = Some(name.into());
        self
    }

    fn self_name(mut self, name: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().name = name.map(Into::into);
        self
    }

    fn value(mut self, value: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().value = Some(value.into());
        self
    }

    fn self_value(mut self, value: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().value = value.map(Into::into);
        self
    }

    fn placeholder(mut self, placeholder: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().placeholder = Some(placeholder.into());
        self
    }

    fn self_placeholder(mut self, placeholder: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().placeholder = placeholder.map(Into::into);
        self
    }

    fn autocapitalize(mut self, autocapitalize: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().autocapitalize = Some(autocapitalize.into());
        self
    }

    fn self_autocapitalize(mut self, autocapitalize: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().autocapitalize = autocapitalize.map(Into::into);
        self
    }

    fn autocomplete(mut self, autocomplete: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().autocomplete = Some(autocomplete.into());
        self
    }

    fn self_autocomplete(mut self, autocomplete: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().autocomplete = autocomplete.map(Into::into);
        self
    }

    fn inputmode(mut self, inputmode: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().inputmode = Some(inputmode.into());
        self
    }

    fn self_inputmode(mut self, inputmode: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().inputmode = inputmode.map(Into::into);
        self
    }

    fn enterkeyhint(mut self, enterkeyhint: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().enterkeyhint = Some(enterkeyhint.into());
        self
    }

    fn self_enterkeyhint(mut self, enterkeyhint: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().enterkeyhint = enterkeyhint.map(Into::into);
        self
    }

    fn spellcheck(mut self, spellcheck: impl Into<Cow<'a, str>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().spellcheck = Some(spellcheck.into());
        self
    }

    fn self_spellcheck(mut self, spellcheck: Option<impl Into<Cow<'a, str>>>) -> Self
    where
        Self: Sized,
    {
        self.textarea_param_mut().spellcheck = spellcheck.map(Into::into);
        self
    }

    fn textarea_param_mut(&mut self) -> &mut TextareaParam<'a>;
}

impl<'a> TextareaParamBuilder<'a> for TextareaParam<'a> {
    fn textarea_param_mut(&mut self) -> &mut TextareaParam<'a> {
        self
    }
}

impl<'a, T: AsMut<TextareaParam<'a>>> TextareaParamBuilder<'a> for T {
    fn textarea_param_mut(&mut self) -> &mut TextareaParam<'a> {
        self.as_mut()
    }
}

/// A multiline text field, the [`Input`](super::input::Input) of long text: the
/// same label, hint, appearance and states around a native `<textarea>`:
///
/// ```ignore
/// rsx! {
///     <Textarea label="Comments" name="comment" rows=6 placeholder="Tell us what you think"/>
/// }
/// ```
///
/// `resize=Auto` and the character count are kept in step by
/// `wingy-hypertext-web` (`component::textarea`), wired up with
/// `init_textareas`/`listen_textareas`. Everything else is native.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = TEXTAREA)]
#[props(builder)]
pub struct Textarea<'a> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    #[prop(impl_from)]
    pub resize: Resize,

    #[as_ref]
    #[as_mut]
    pub param: TextareaParam<'a>,

    #[prop(into)]
    pub label: Option<Cow<'a, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'a, str>>,

    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl<'a> Default for Textarea<'a> {
    fn default() -> Self {
        Self {
            appearance: Appearance::Outlined,
            resize: Resize::default(),
            param: TextareaParam::default(),
            label: None,
            hint: None,
            bare: false,
            attributes: CommonAttrs::default(),
        }
    }
}

impl<'a> Textarea<'a> {
    /// The text of the character count, the one the client keeps updating.
    fn count_text(&self) -> String {
        let length = self.param.value.as_deref().map(utf16_len).unwrap_or(0);
        match self.param.maxlength {
            Some(maxlength) => format!("{} characters left", (maxlength as usize).saturating_sub(length)),
            None => format!("{length} characters"),
        }
    }
}

impl<'a> Renderable for Textarea<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.param.required { REQUIRED } else { "" },
            self.appearance.into_str(),
            self.resize.as_class(),
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if let Some(label) = &self.label {
                    <label class=LABEL>(label)</label>
                }
                @if !self.bare {
                    <TextareaField param = (self.param.clone()) />
                }
                @if self.hint.is_some() || self.param.count {
                    <div class=TEXTAREA_FOOTER>
                        @if let Some(hint) = &self.hint {
                            <small class=HINT>(hint)</small>
                        }
                        @if self.param.count {
                            <small class=TEXTAREA_COUNT>(self.count_text())</small>
                        }
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(AsRef, AsMut, Default, Props)]
#[const_str(CLASS = TEXTAREA_FIELD)]
#[props(builder)]
pub struct TextareaField<'a> {
    #[as_ref]
    #[as_mut]
    pub param: TextareaParam<'a>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,
}

impl<'a> Renderable for TextareaField<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        let autofocus = self.param.autofocus.then_some(true);
        let disabled = self.param.disabled.then_some(true);
        let readonly = self.param.readonly.then_some(true);
        let required = self.param.required.then_some(true);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                <textarea
                    class=CONTROL
                    name=[&self.param.name]
                    rows=(self.param.rows)
                    placeholder=[&self.param.placeholder]
                    minlength=[self.param.minlength]
                    maxlength=[self.param.maxlength]
                    autocapitalize=[&self.param.autocapitalize]
                    autocomplete=[&self.param.autocomplete]
                    inputmode=[&self.param.inputmode]
                    enterkeyhint=[&self.param.enterkeyhint]
                    spellcheck=[&self.param.spellcheck]
                    autofocus=[autofocus]
                    disabled=[disabled]
                    readonly=[readonly]
                    required=[required]
                >(&self.param.value)</textarea>
            </div>
        }
        .render_to(buffer);
    }
}
