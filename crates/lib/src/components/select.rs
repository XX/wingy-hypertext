use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::{fontawesome, fontawesome_ext};
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CHECK, CLEAR_BUTTON, COMBOBOX, DISABLED, DISPLAY_INPUT, EXPAND_ICON, HINT, LABEL, LISTBOX, MULTIPLE, OPTION,
    OPTION_LABEL, PILL, POPUP, POPUP_BODY, REQUIRED, SELECT, SELECT_POPUP, SELECTED, TAGS, VALUE_INPUT,
};

/// The preferred placement of the select's menu. The actual placement may
/// flip to keep the listbox in the viewport.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum SelectPlacement {
    #[default]
    Bottom,
    Top,
}

/// A dropdown control mirroring Web Awesome's `wa-select`: a combobox with a
/// display input and a listbox of [`SelectOption`] children. The interactive
/// behavior (opening, selection, keyboard navigation) is implemented in
/// `wingy-hypertext-web` (`components::select`) and must be wired up on the
/// client with `init_selects`/`listen_selects`.
#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = SELECT)]
#[props(builder)]
pub struct Select<R: Renderable = ()> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    pub placement: SelectPlacement,

    pub pill: bool,

    pub disabled: bool,

    pub required: bool,

    pub multiple: bool,

    /// The maximum number of selected options to show as tags with `multiple`.
    /// After the maximum, "+n" is shown to indicate the number of additional
    /// items that are selected. Set to 0 to remove the limit. Defaults to 3.
    pub max_options_visible: Option<i32>,

    /// Adds a clear button that resets the selection when the select is not empty.
    pub with_clear: bool,

    #[prop(into)]
    pub name: Option<Cow<'static, str>>,

    #[prop(into)]
    pub placeholder: Option<Cow<'static, str>>,

    #[prop(into)]
    pub label: Option<Cow<'static, str>>,

    #[prop(into)]
    pub hint: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> Select<R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.pill { PILL } else { "" },
            if self.required { REQUIRED } else { "" },
            if self.multiple { MULTIPLE } else { "" },
            if self.disabled { DISABLED } else { "" },
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        let disabled = self.disabled.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-max-options-visible=[self.max_options_visible]
            >
                @if let Some(label) = &self.label {
                    <label class=LABEL>(label)</label>
                }
                <div
                    class=(SELECT_POPUP, " ", POPUP)
                    data-placement=(self.placement.into_str())
                    data-flip
                    data-shift
                    data-sync="width"
                    data-auto-size="vertical"
                    data-auto-size-padding="10"
                >
                    <div class=COMBOBOX>
                        <input
                            class=DISPLAY_INPUT
                            type="text"
                            placeholder=[&self.placeholder]
                            disabled=[disabled]
                            autocomplete="off"
                            spellcheck="false"
                            autocapitalize="off"
                            readonly
                            role="combobox"
                            aria-haspopup="listbox"
                            aria-expanded="false"
                        />
                        @if self.multiple {
                            // Selected options are rendered here as removable
                            // tags by the client-side logic
                            <div class=TAGS></div>
                        }
                        <input
                            class=VALUE_INPUT
                            type="text"
                            name=[&self.name]
                            disabled=[disabled]
                            required=[required]
                            tabindex="-1"
                            aria-hidden="true"
                        />
                        @if self.with_clear {
                            <button class=CLEAR_BUTTON type="button" tabindex="-1" aria-label="Clear entry" hidden>
                                (fontawesome::regular::CircleXmark)
                            </button>
                        }
                        <span class=EXPAND_ICON aria-hidden="true">
                            (fontawesome_ext::regular::ChevronDown)
                        </span>
                    </div>
                    <div class=POPUP_BODY>
                        <div
                            class=LISTBOX
                            role="listbox"
                            tabindex="-1"
                            aria-multiselectable=(if self.multiple { "true" } else { "false" })
                            hidden
                        >
                            (children)
                        </div>
                    </div>
                </div>
                @if let Some(hint) = &self.hint {
                    <small class=HINT>(hint)</small>
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// A single choice within a [`Select`], mirroring Web Awesome's `wa-option`.
/// Named `SelectOption` to avoid clashing with `std::option::Option`.
#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = OPTION)]
#[props(builder)]
pub struct SelectOption<R: Renderable = ()> {
    #[prop(into)]
    pub value: Option<Cow<'static, str>>,

    /// The option's plain text label. Usually derived from the option's content,
    /// but can be provided manually for cases involving complex content.
    #[prop(into)]
    pub label: Option<Cow<'static, str>>,

    pub disabled: bool,

    pub selected: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    #[prop(convert)]
    pub children: Option<R>,
}

impl<R: Renderable> SelectOption<R> {
    fn render_to(&self, buffer: &mut Buffer, children: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.selected { SELECTED } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="option"
                tabindex="-1"
                aria-selected=(if self.selected { "true" } else { "false" })
                aria-disabled=[self.disabled.then_some("true")]
                data-value=[&self.value]
                data-label=[&self.label]
            >
                <span class=CHECK aria-hidden="true">
                    (fontawesome::solid::Check)
                </span>
                <span class=OPTION_LABEL>(children)</span>
            </div>
        }
        .render_to(buffer);
    }
}
