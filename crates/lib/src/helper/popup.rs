use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{ACTIVE, ARROW, POPUP, POPUP_BODY, POPUP_HOVER_BRIDGE};

/// The preferred placement of the popup relative to its anchor. The actual
/// placement may vary to keep the popup inside of the viewport when `flip` is on.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum PopupPlacement {
    #[default]
    Top,
    TopStart,
    TopEnd,
    Bottom,
    BottomStart,
    BottomEnd,
    Right,
    RightStart,
    RightEnd,
    Left,
    LeftStart,
    LeftEnd,
}

/// Which axes the popup automatically resizes on to prevent it from overflowing.
#[derive(Copy, Clone, Debug, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum AutoSize {
    Horizontal,
    Vertical,
    Both,
}

/// Which anchor dimensions the popup syncs its own dimensions to.
#[derive(Copy, Clone, Debug, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum SyncSize {
    Width,
    Height,
    Both,
}

/// The placement of the arrow. The default is `Anchor`, which aligns the arrow
/// as close to the center of the anchor as possible.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum ArrowPlacement {
    #[default]
    Anchor,
    Start,
    End,
    Center,
}

/// A positioning primitive: anchors one element to another and keeps them
/// positioned together as the page scrolls or resizes. The positioning logic
/// (a floating-ui subset: offset, flip, shift, sync, auto-size, arrow) is
/// implemented in `wingy-hypertext-web` (`helper::popup`) and must be
/// wired up on the client with `init_popups`/`listen_popups`; the configuration
/// is carried by `data-*` attributes on the host element.
///
/// Everything is composed out of the children: the anchor comes first — any
/// single element — followed by the [`PopupBody`] holding the content that
/// gets positioned next to it:
///
/// ```ignore
/// rsx! {
///     <Popup placement=BottomStart active=true>
///         <button>"Anchor"</button>
///         <PopupBody arrow=true>"Content"</PopupBody>
///     </Popup>
/// }
/// ```
///
/// The anchor may also live outside of the popup, referenced by
/// [`anchor_id`](Self::anchor_id); the popup then only carries the body, which
/// is what [`Tooltip`](crate::component::tooltip::Tooltip) does:
///
/// ```ignore
/// rsx! {
///     <button id="the-anchor">"Anchor"</button>
///     <Popup anchor_id="the-anchor" active=true>
///         <PopupBody arrow=true>"Content"</PopupBody>
///     </Popup>
/// }
/// ```
///
/// Only the body is positioned, so everything else — the arrow and the hover
/// bridge — is a prop of [`PopupBody`] rather than of the popup itself.
/// Components that come with a body of their own render it themselves: the
/// [`DropdownMenu`](crate::component::dropdown::DropdownMenu) and the
/// [`Select`](crate::component::select::Select) listbox are `PopupBody`s, so
/// their popups take them as plain children.
///
/// This is a low-level building block for popovers, dropdowns, and tooltips —
/// it provides positioning only, no styles and no accessible experience by itself.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = POPUP)]
#[props(builder)]
pub struct Popup<'a> {
    #[prop(impl_from)]
    pub placement: PopupPlacement,

    /// Activates the positioning logic and shows the popup.
    pub active: bool,

    /// The distance in pixels from which to offset the panel away from its anchor.
    pub distance: Option<i32>,

    /// The distance in pixels from which to offset the panel along its anchor.
    pub skidding: Option<i32>,

    /// When set, placement of the popup will flip to the opposite site to keep it in view.
    pub flip: bool,

    pub flip_padding: Option<i32>,

    /// Moves the popup along the axis to keep it in view when clipped.
    pub shift: bool,

    pub shift_padding: Option<i32>,

    /// When set, the popup will automatically resize itself to prevent it from
    /// overflowing, exposing `--auto-size-available-width/height` to its content.
    pub auto_size: Option<AutoSize>,

    pub auto_size_padding: Option<i32>,

    /// Syncs the popup's width or height to that of the anchor element.
    pub sync: Option<SyncSize>,

    /// How to align the arrow of the [`PopupBody`], if it has one.
    pub arrow_placement: Option<ArrowPlacement>,

    pub arrow_padding: Option<i32>,

    /// The `id` of an anchor element living outside of the popup. Leave it
    /// unset when the anchor is rendered as the popup's first child.
    #[prop(into)]
    pub anchor_id: Option<Cow<'static, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Popup<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS, if self.active { ACTIVE } else { "" }]);
        let style_line = self.style_line_with(&[]);

        let flip = self.flip.then_some("");
        let shift = self.shift.then_some("");

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-placement=(self.placement.into_str())
                data-anchor=[&self.anchor_id]
                data-distance=[self.distance]
                data-skidding=[self.skidding]
                data-flip=[flip]
                data-flip-padding=[self.flip_padding]
                data-shift=[shift]
                data-shift-padding=[self.shift_padding]
                data-sync=[self.sync.map(|sync| sync.into_str())]
                data-auto-size=[self.auto_size.map(|auto_size| auto_size.into_str())]
                data-auto-size-padding=[self.auto_size_padding]
                data-arrow-placement=[self.arrow_placement.map(|placement| placement.into_str())]
                data-arrow-padding=[self.arrow_padding]
                (self.get_attrs())
            >
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

/// The content a [`Popup`] positions next to its anchor: this is the element
/// the positioning logic moves around (`position: fixed`), so a popup without
/// a body positions nothing. It goes after the anchor, and it is also what
/// brings the arrow and the hover bridge.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = POPUP_BODY)]
#[props(builder)]
pub struct PopupBody<'a> {
    /// Attaches an arrow to the popup, rendered as the body's last child and
    /// customizable with the `--arrow-size` and `--arrow-color` custom
    /// properties. It is aligned against the anchor by
    /// [`Popup::arrow_placement`] and [`Popup::arrow_padding`].
    pub arrow: bool,

    /// Fills the gap between the anchor and the popup with an invisible element
    /// so the pointer never technically leaves them (useful for hover-driven
    /// popups). It is rendered before the body, as its sibling inside the popup.
    pub hover_bridge: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for PopupBody<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            @if self.hover_bridge {
                <span class=POPUP_HOVER_BRIDGE></span>
            }
            <div id=[id] class=[&class_line] style=[&style_line]>
                (self.children)
                @if self.arrow {
                    <div class=ARROW role="presentation"></div>
                }
            </div>
        }
        .render_to(buffer);
    }
}
