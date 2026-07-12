use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{
    AriaAttributes, GlobalAttributes, SvgGlobalAttributes, hypertext_elements, hypertext_svg_elements,
};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CHECK, CLEAR_BUTTON, COMBOBOX, DISABLED, DISPLAY_INPUT, EXPAND_ICON, HINT, LABEL, LISTBOX, MULTIPLE, OPTION,
    OPTION_LABEL, PILL, POPUP, POPUP_BODY, REQUIRED, SELECT, SELECT_POPUP, SELECTED, VALUE_INPUT,
};

/// A dropdown control mirroring Web Awesome's `wa-select`: a combobox with a
/// display input and a listbox of [`SelectOption`] children. The interactive
/// behavior (opening, selection, keyboard navigation) is implemented in
/// `wingy-hypertext-web` (`components::select`) and must be wired up on the
/// client with `init_selects`/`listen_selects`.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = SELECT)]
#[props(builder)]
pub struct Select<R: Renderable = ()> {
    #[prop(impl_from)]
    pub appearance: Appearance,

    pub pill: bool,

    pub disabled: bool,

    pub required: bool,

    pub multiple: bool,

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

impl<R: Renderable> Renderable for Select<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.pill { PILL } else { "" },
            if self.required { REQUIRED } else { "" },
            if self.multiple { MULTIPLE } else { "" },
            if self.disabled { DISABLED } else { "" },
            self.appearance.into_str(),
        ]);
        let style_line = self.style_line_with([]);

        let disabled = self.disabled.then_some(true);
        let required = self.required.then_some(true);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line]>
                @if let Some(label) = &self.label {
                    <label class=LABEL>(label)</label>
                }
                <div
                    class=(SELECT_POPUP, " ", POPUP)
                    data-placement="bottom"
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
                                // Circle-xmark icon
                                <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512">
                                    // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                    // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                    <path fill="currentColor" d="M256 512A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM175 175c9.4-9.4 24.6-9.4 33.9 0l47 47 47-47c9.4-9.4 24.6-9.4 33.9 0s9.4 24.6 0 33.9l-47 47 47 47c9.4 9.4 9.4 24.6 0 33.9s-24.6 9.4-33.9 0l-47-47-47 47c-9.4 9.4-24.6 9.4-33.9 0s-9.4-24.6 0-33.9l47-47-47-47c-9.4-9.4-9.4-24.6 0-33.9z"/>
                                </svg>
                            </button>
                        }
                        <span class=EXPAND_ICON aria-hidden="true">
                            // Chevron-down icon
                            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                                // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                                // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                                <path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"/>
                            </svg>
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
                            (self.children)
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
#[derive(Default, AsRef, AsMut, Props)]
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

impl<R: Renderable> Renderable for SelectOption<R> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with([
            Self::CLASS,
            if self.selected { SELECTED } else { "" },
            if self.disabled { DISABLED } else { "" },
        ]);
        let style_line = self.style_line_with([]);

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
                    // Check icon
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512">
                        // !Font Awesome Free v7.2.0 by @fontawesome - https://fontawesome.com
                        // License - https://fontawesome.com/license/free Copyright 2026 Fonticons, Inc.
                        <path fill="currentColor" d="M434.8 70.1c14.3 10.4 17.5 30.4 7.1 44.7l-256 352c-5.5 7.6-14 12.3-23.4 13.1s-18.5-2.7-25.1-9.3l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l101.5 101.5 234-321.7c10.4-14.3 30.4-17.5 44.7-7.1z"/>
                    </svg>
                </span>
                <span class=OPTION_LABEL>(self.children)</span>
            </div>
        }
        .render_to(buffer);
    }
}
