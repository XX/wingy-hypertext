use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::{fontawesome, fontawesome_ext};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    CHECK, CHECKBOX_ADJACENT, CHECKED, DISABLED, DROPDOWN, DROPDOWN_ITEM, DROPDOWN_ITEM_DETAILS, DROPDOWN_ITEM_ICON,
    DROPDOWN_ITEM_LABEL, DROPDOWN_MENU, DROPDOWN_SUBMENU, SIZE_EXTRA_LARGE, SIZE_EXTRA_SMALL, SIZE_LARGE, SIZE_MEDIUM,
    SIZE_SMALL, SUBMENU_ADJACENT, SUBMENU_ICON,
};
use crate::convert;
use crate::helper::popup::AutoSize::Vertical;
use crate::helper::popup::{Popup, PopupBody, PopupPlacement};
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

/// A list of options displayed in a menu next to a trigger element. Everything
/// is composed through the children: a trigger — the first child, any element
/// that opens the menu (a [`Button`](crate::component::button::Button), for
/// example), followed by a [`DropdownMenu`] holding the [`DropdownItem`]s,
/// optionally interleaved with headings (group labels) and
/// [`Divider`](crate::layout::divider::Divider)s:
///
/// ```ignore
/// rsx! {
///     <Dropdown>
///         <Button>"Options"</Button>
///         <DropdownMenu>
///             <DropdownItem value="edit"><DropdownItemLabel>"Edit"</DropdownItemLabel></DropdownItem>
///         </DropdownMenu>
///     </Dropdown>
/// }
/// ```
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
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN)]
#[props(builder)]
pub struct Dropdown<'a> {
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

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Dropdown<'a> {
    fn default() -> Self {
        Self {
            placement: PopupPlacement::BottomStart,
            open: false,
            distance: None,
            skidding: None,
            size: None,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Dropdown<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line =
            self.class_line_with(&[Self::CLASS, self.size.map(DropdownSize::into_str).unwrap_or_default()]);
        let style_line = self.style_line_with(&[]);
        let open = self.open.then_some("");

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] data-open=[open] (self.get_attrs())>
                <Popup
                    placement=(self.placement)
                    flip=true
                    shift=true
                    shift_padding=10
                    auto_size=Vertical
                    auto_size_padding=10
                    bare=true
                    self_distance=(self.distance)
                    self_skidding=(self.skidding)
                >
                    // The trigger is the popup's anchor, the menu goes into the popup body.
                    (self.children)
                </Popup>
            </div>
        }
        .render_to(buffer);
    }
}

/// The element that triggers the dropdown, such as a `Button`. It is used
/// as the menu's anchor, so it must render exactly one element.
#[derive(Default, AsRef, AsMut, Props)]
#[props(builder)]
pub struct DropdownTrigger<'a> {
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownTrigger<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        rsx! {
            (self.children)
        }
        .render_to(buffer);
    }
}

/// The menu of a [`Dropdown`], rendered into the body of the popup that
/// anchors it to the trigger. Its children are the [`DropdownItem`]s.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_MENU)]
#[props(builder)]
pub struct DropdownMenu<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownMenu<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        rsx! {
            <PopupBody attributes=(self.attributes.clone())>
                <div class=DROPDOWN_MENU role="menu" tabindex="-1" aria-orientation="vertical" hidden>
                    (self.children)
                </div>
            </PopupBody>
        }
        .render_to(buffer);
    }
}

/// A single entry of a [`DropdownMenu`]: a plain action, a checkable item
/// (`checkbox`), or a submenu trigger.
///
/// The content is composed from the children: an optional
/// [`DropdownItemIcon`], a [`DropdownItemLabel`], an optional
/// [`DropdownItemDetails`] and, for a submenu trigger, a [`DropdownSubmenu`]
/// last. The checkmark of a checkable item is rendered by the item itself,
/// before the children.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_ITEM)]
#[props(builder)]
pub struct DropdownItem<'a> {
    /// `Danger` flags a destructive action; every other variant renders as a
    /// regular item.
    #[prop(impl_from)]
    pub variant: Variant,

    /// Makes the item a checkbox, rendering a checkmark when `checked`.
    pub checkbox: bool,

    /// Announces the item as a submenu trigger (`aria-haspopup`/`aria-expanded`).
    /// Set it on every item holding a [`DropdownSubmenu`]: the submenu is a
    /// child, so the item cannot detect it while rendering. The styling doesn't
    /// depend on this flag — the CSS matches the submenu itself.
    pub submenu: bool,

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

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for DropdownItem<'a> {
    fn default() -> Self {
        Self {
            variant: Variant::Neutral,
            checkbox: false,
            submenu: false,
            checked: false,
            disabled: false,
            checkbox_adjacent: false,
            submenu_adjacent: false,
            value: None,
            label: None,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for DropdownItem<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[
            Self::CLASS,
            if self.checked { CHECKED } else { "" },
            if self.disabled { DISABLED } else { "" },
            if self.checkbox_adjacent { CHECKBOX_ADJACENT } else { "" },
            if self.submenu_adjacent { SUBMENU_ADJACENT } else { "" },
            self.variant.into_str(),
        ]);
        let style_line = self.style_line_with(&[]);

        let role = if self.checkbox { "menuitemcheckbox" } else { "menuitem" };
        let checked = self.checkbox.then(|| convert::bool_to_str(self.checked));
        let has_popup = self.submenu.then_some("menu");
        let expanded = self.submenu.then_some("false");

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
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

/// An icon displayed before the label of a [`DropdownItem`].
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_ITEM_ICON)]
#[props(builder)]
pub struct DropdownItemIcon<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownItemIcon<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </span>
        }
        .render_to(buffer);
    }
}

/// The label of a [`DropdownItem`], taking the space the icon, the details and
/// the submenu indicator leave. It is also what type-to-select matches against,
/// unless the item carries an explicit `label`.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_ITEM_LABEL)]
#[props(builder)]
pub struct DropdownItemLabel<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownItemLabel<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </span>
        }
        .render_to(buffer);
    }
}

/// Secondary content displayed after the label of a [`DropdownItem`], such as a
/// keyboard shortcut.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_ITEM_DETAILS)]
#[props(builder)]
pub struct DropdownItemDetails<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownItemDetails<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </span>
        }
        .render_to(buffer);
    }
}

/// A menu nested into a [`DropdownItem`], turning it into a submenu trigger.
/// It renders the submenu indicator next to the item's content and the nested
/// menu itself, so it must be the last child of the item; the item needs
/// `submenu=true` for the matching ARIA attributes.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DROPDOWN_SUBMENU)]
#[props(builder)]
pub struct DropdownSubmenu<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DropdownSubmenu<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <span class=SUBMENU_ICON aria-hidden="true">
                (fontawesome_ext::regular::ChevronRight)
            </span>
            <div id=[id]
                class=[&class_line]
                style=[&style_line]
                role="menu"
                tabindex="-1"
                aria-orientation="vertical"
                hidden
                (self.get_attrs())
            >
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
