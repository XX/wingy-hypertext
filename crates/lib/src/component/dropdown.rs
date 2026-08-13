use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::{fontawesome, fontawesome_ext};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CHECK, CHECKBOX_ADJACENT, CHECKED, DISABLED, DROPDOWN, DROPDOWN_ITEM, DROPDOWN_ITEM_DETAILS, DROPDOWN_ITEM_ICON,
    DROPDOWN_ITEM_LABEL, DROPDOWN_MENU, DROPDOWN_SUBMENU, HAS_SUBMENU, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE,
    SIZE_MEDIUM, SIZE_SMALL, SUBMENU_ADJACENT, SUBMENU_ICON,
};
use crate::convert;
use crate::helper::popup::{self, Popup, PopupBody, PopupPlacement};
use crate::variant::Variant;

/// The dropdown's size, applied as the corresponding `size-*` class. Dropdowns
/// are sized relative to the current font size, so the size can also be set
/// with a `font-size` style or a `size-*` class on any ancestor element.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DropdownSize {
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl DropdownSize {
    pub const fn into_str(self) -> &'static str {
        match self {
            Self::ExtraSmall => SIZE_EXTRA_SMALL,
            Self::Small => SIZE_SMALL,
            Self::Medium => SIZE_MEDIUM,
            Self::Large => SIZE_LARGE,
            Self::ExtraLarge => SIZE_EXTRA_LARGE,
        }
    }
}

/// A list of options displayed in a menu next to a trigger element, mirroring
/// Web Awesome's `wa-dropdown`: the `trigger` prop holds the element that opens
/// the menu (a [`Button`](crate::component::button::Button), for example) and
/// the children are the [`DropdownItem`]s of the menu, optionally interleaved
/// with headings (group labels) and [`Divider`](crate::layout::divider::Divider)s.
///
/// The menu is positioned with the [`Popup`] helper, and the interactive
/// behavior (opening, selection, submenus, keyboard navigation with
/// type-to-select) is implemented in `wingy-hypertext-web`
/// (`component::dropdown`) and must be wired up on the client with
/// `init_dropdowns`/`listen_dropdowns`.
///
/// Selecting an item dispatches a cancelable bubbling `wg-select` event on the
/// dropdown; canceling it keeps the menu open. Opening and closing dispatch
/// `wg-show`/`wg-hide` and `wg-after-show`/`wg-after-hide`.
#[derive(AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = DROPDOWN)]
#[props(builder)]
pub struct Dropdown<'a, T: Renderable = ()> {
    /// The placement of the menu in reference to the trigger. The menu flips
    /// to a more optimal location when the preferred placement doesn't have
    /// enough room.
    #[prop(impl_from)]
    pub placement: PopupPlacement,

    /// Renders the dropdown already open: `init_dropdowns` opens it on the client.
    pub open: bool,

    /// The distance of the menu from its trigger, in pixels.
    pub distance: Option<i32>,

    /// The offset of the menu along its trigger, in pixels.
    pub skidding: Option<i32>,

    pub size: Option<DropdownSize>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// The element that triggers the dropdown, such as a `Button`. It is used
    /// as the menu's anchor, so it must render exactly one element.
    #[prop(convert)]
    pub trigger: Option<T>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a, T: Renderable> Default for Dropdown<'a, T> {
    fn default() -> Self {
        Self {
            placement: PopupPlacement::BottomStart,
            open: false,
            distance: None,
            skidding: None,
            size: None,
            attributes: CommonAttrs::default(),
            trigger: None,
            children: None,
        }
    }
}

impl<'a, T: Renderable> Dropdown<'a, T> {
    fn render_to(&self, buffer: &mut Buffer, trigger: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line =
            self.class_line_with(&[Self::CLASS, self.size.map(DropdownSize::into_str).unwrap_or_default()]);
        let style_line = self.style_line_with(&[]);

        let open = self.open.then_some("");

        // The trigger is the popup's anchor, the menu goes into the popup body.
        let content = rsx! {
            (trigger)
            <PopupBody>
                <div class=DROPDOWN_MENU role="menu" tabindex="-1" aria-orientation="vertical" hidden>
                    (self.children)
                </div>
            </PopupBody>
        };

        // The popup needs no class of its own: it is the only child of the
        // dropdown host, so `.dropdown > .popup` addresses it unambiguously.
        // The optional `distance`/`skidding` don't survive a builder setter
        // (which would wrap them into `Some` again), so they are assigned directly.
        let mut menu_popup = Popup::builder()
            .placement(self.placement)
            .flip(true)
            .shift(true)
            .shift_padding(10)
            .auto_size(popup::AutoSize::Vertical)
            .auto_size_padding(10)
            .bare(true)
            .children(&content);
        menu_popup.distance = self.distance;
        menu_popup.skidding = self.skidding;

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] data-open=[open] (self.get_attrs())>
                (menu_popup)
            </div>
        }
        .render_to(buffer);
    }
}

/// A single entry of a [`Dropdown`] menu, mirroring Web Awesome's
/// `wa-dropdown-item`: a plain action, a checkable item (`checkbox`), or a
/// submenu trigger (the `submenu` prop).
#[derive(AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = DROPDOWN_ITEM)]
#[props(builder)]
pub struct DropdownItem<'a, I: Renderable = (), D: Renderable = (), S: Renderable = ()> {
    /// `Danger` flags a destructive action; every other variant renders as a
    /// regular item.
    #[prop(impl_from)]
    pub variant: Variant,

    /// Makes the item a checkbox, rendering a checkmark when `checked`.
    pub checkbox: bool,

    pub checked: bool,

    pub disabled: bool,

    /// Draws the space a checkmark takes even without being a checkbox, so
    /// labels stay aligned with checkable siblings. `init_dropdowns` keeps this
    /// in sync with the items of the same menu on the client.
    pub checkbox_adjacent: bool,

    /// Draws the space a submenu indicator takes, so items stay aligned with
    /// siblings that have submenus. Kept in sync by `init_dropdowns` as well.
    pub submenu_adjacent: bool,

    /// An optional value, useful for determining which item was selected when
    /// listening to the dropdown's `wg-select` event.
    #[prop(into)]
    pub value: Option<Cow<'static, str>>,

    /// The item's plain text label. Usually derived from the item's content,
    /// but can be provided manually for cases involving complex content; it is
    /// what type-to-select matches against.
    #[prop(into)]
    pub label: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// An optional icon displayed before the label.
    #[prop(convert)]
    pub icon: Option<I>,

    /// Additional content displayed after the label, such as a keyboard shortcut.
    #[prop(convert)]
    pub details: Option<D>,

    /// Nested [`DropdownItem`]s, turning this item into a submenu trigger.
    #[prop(convert)]
    pub submenu: Option<S>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a, I: Renderable, D: Renderable, S: Renderable> Default for DropdownItem<'a, I, D, S> {
    fn default() -> Self {
        Self {
            variant: Variant::Neutral,
            checkbox: false,
            checked: false,
            disabled: false,
            checkbox_adjacent: false,
            submenu_adjacent: false,
            value: None,
            label: None,
            attributes: CommonAttrs::default(),
            icon: None,
            details: None,
            submenu: None,
            children: None,
        }
    }
}

impl<'a, I: Renderable, D: Renderable, S: Renderable> DropdownItem<'a, I, D, S> {
    fn render_to(
        &self,
        buffer: &mut Buffer,
        icon: Option<&dyn Renderable>,
        details: Option<&dyn Renderable>,
        submenu: Option<&dyn Renderable>,
    ) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.checked { CHECKED } else { "" },
            if self.disabled { DISABLED } else { "" },
            if submenu.is_some() { HAS_SUBMENU } else { "" },
            if self.checkbox_adjacent { CHECKBOX_ADJACENT } else { "" },
            if self.submenu_adjacent { SUBMENU_ADJACENT } else { "" },
            self.variant.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        let role = if self.checkbox { "menuitemcheckbox" } else { "menuitem" };
        let checked = self.checkbox.then(|| convert::bool_to_str(self.checked));
        let has_popup = submenu.is_some().then_some("menu");
        let expanded = submenu.is_some().then_some("false");

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role=(role)
                tabindex="-1"
                aria-checked=[checked]
                aria-disabled=[self.disabled.then_some("true")]
                aria-haspopup=[has_popup]
                aria-expanded=[expanded]
                data-value=[&self.value]
                data-label=[&self.label]
                (self.get_attrs())
            >
                @if self.checkbox {
                    <span class=CHECK aria-hidden="true">
                        (fontawesome::solid::Check)
                    </span>
                }
                @if let Some(icon) = icon {
                    <span class=DROPDOWN_ITEM_ICON>(icon)</span>
                }
                <span class=DROPDOWN_ITEM_LABEL>(self.children)</span>
                @if let Some(details) = details {
                    <span class=DROPDOWN_ITEM_DETAILS>(details)</span>
                }
                @if let Some(submenu) = submenu {
                    <span class=SUBMENU_ICON aria-hidden="true">
                        (fontawesome_ext::regular::ChevronRight)
                    </span>
                    <div class=DROPDOWN_SUBMENU role="menu" tabindex="-1" aria-orientation="vertical" hidden>
                        (submenu)
                    </div>
                }
            </div>
        }
        .render_to(buffer);
    }
}
