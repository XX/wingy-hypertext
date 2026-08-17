use std::borrow::Cow;
use std::ops::{BitOr, BitOrAssign};

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{DISABLED, TOOLTIP, TOOLTIP_BODY};
use crate::helper::popup::{Popup, PopupBody, PopupPlacement};

/// The gap between the tooltip and its anchor, matching `wa-tooltip`.
const DEFAULT_DISTANCE: i32 = 8;

/// Keeps the tooltip inside the viewport when it is shifted along its anchor.
const SHIFT_PADDING: i32 = 8;

/// A single way of activating a [`Tooltip`]. Several of them combine into
/// [`TooltipTriggers`] with `|`, e.g. `Hover | Click`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TooltipTrigger {
    /// The pointer moves over the anchor.
    Hover,

    /// The anchor receives focus.
    Focus,

    /// The anchor is clicked; clicking again dismisses the tooltip.
    Click,

    /// No built-in activation: the tooltip is shown by rendering it `open` or
    /// by calling `set_tooltip_open` on the client.
    Manual,
}

impl TooltipTrigger {
    const fn bit(self) -> u8 {
        match self {
            Self::Hover => 1,
            Self::Focus => 1 << 1,
            Self::Click => 1 << 2,
            Self::Manual => 0,
        }
    }
}

/// The set of triggers that activate a [`Tooltip`], defaulting to
/// `Hover | Focus`. [`TooltipTrigger::Manual`] is the empty set, leaving the
/// tooltip to be shown programmatically only.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TooltipTriggers(u8);

impl TooltipTriggers {
    pub const fn contains(self, trigger: TooltipTrigger) -> bool {
        let bit = trigger.bit();
        self.0 & bit == bit
    }

    /// The triggers as the space-separated list `wa-tooltip` uses, which is
    /// also what the client-side behavior reads back.
    pub const fn into_str(self) -> &'static str {
        match self.0 {
            0 => "manual",
            1 => "hover",
            2 => "focus",
            3 => "hover focus",
            4 => "click",
            5 => "hover click",
            6 => "focus click",
            _ => "hover focus click",
        }
    }
}

impl Default for TooltipTriggers {
    fn default() -> Self {
        Self(TooltipTrigger::Hover.bit() | TooltipTrigger::Focus.bit())
    }
}

impl From<TooltipTrigger> for TooltipTriggers {
    fn from(trigger: TooltipTrigger) -> Self {
        Self(trigger.bit())
    }
}

impl BitOr for TooltipTrigger {
    type Output = TooltipTriggers;

    fn bitor(self, rhs: Self) -> TooltipTriggers {
        TooltipTriggers(self.bit() | rhs.bit())
    }
}

impl BitOr<TooltipTrigger> for TooltipTriggers {
    type Output = Self;

    fn bitor(self, rhs: TooltipTrigger) -> Self {
        Self(self.0 | rhs.bit())
    }
}

impl BitOrAssign<TooltipTrigger> for TooltipTriggers {
    fn bitor_assign(&mut self, rhs: TooltipTrigger) {
        self.0 |= rhs.bit();
    }
}

/// Brief contextual information displayed next to the element it describes,
/// shown when the user hovers, focuses or taps it.
///
/// The tooltip is rendered as a sibling of its anchor and points at it by id:
///
/// ```ignore
/// rsx! {
///     <Button id="save">"Save"</Button>
///     <Tooltip anchor_id="save">"Saves the current document"</Tooltip>
/// }
/// ```
///
/// Keep the content to text and presentational markup — a tooltip can't be
/// focused or operated with a keyboard, so interactive content belongs into a
/// [`Dropdown`](crate::component::dropdown::Dropdown) instead.
///
/// The tooltip is positioned with the [`Popup`] helper, and the behavior
/// (triggers with their delays, [Escape], repositioning, wiring the anchor's
/// `aria-labelledby`) is implemented in `wingy-hypertext-web`
/// (`component::tooltip`) and must be wired up on the client with
/// `init_tooltips`/`listen_tooltips`.
///
/// Showing and hiding dispatch the cancelable bubbling `wg-show`/`wg-hide`
/// events, followed by `wg-after-show`/`wg-after-hide` once the animation is
/// complete.
#[derive(AsRef, AsMut, Props)]
#[const_str(CLASS = TOOLTIP)]
#[props(builder)]
pub struct Tooltip<'a> {
    /// The preferred placement of the tooltip. The actual placement may vary
    /// to keep the tooltip inside of the viewport.
    #[prop(impl_from)]
    pub placement: PopupPlacement,

    /// The `id` of the element the tooltip describes (`for` in `wa-tooltip`).
    #[prop(into)]
    pub anchor_id: Option<Cow<'static, str>>,

    /// Renders the tooltip already open: `init_tooltips` shows it on the client.
    pub open: bool,

    /// Disables the tooltip so it won't show when triggered.
    pub disabled: bool,

    /// The distance of the tooltip from its anchor, in pixels. Defaults to 8.
    pub distance: Option<i32>,

    /// The offset of the tooltip along its anchor, in pixels.
    pub skidding: Option<i32>,

    /// How long to wait before showing the tooltip when the pointer moves in,
    /// in milliseconds. Defaults to 150.
    pub show_delay: Option<i32>,

    /// How long to wait before hiding the tooltip when the pointer moves out,
    /// in milliseconds. Defaults to 0.
    pub hide_delay: Option<i32>,

    /// Controls how the tooltip is activated.
    #[prop(into)]
    pub trigger: TooltipTriggers,

    pub arrow: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Default for Tooltip<'a> {
    fn default() -> Self {
        Self {
            placement: PopupPlacement::Top,
            anchor_id: None,
            open: false,
            disabled: false,
            distance: None,
            skidding: None,
            show_delay: None,
            hide_delay: None,
            trigger: TooltipTriggers::default(),
            arrow: true,
            attributes: CommonAttrs::default(),
            children: None,
        }
    }
}

impl<'a> Renderable for Tooltip<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS, if self.disabled { DISABLED } else { "" }]);
        let style_line = self.style_line_with(&[]);
        let open = self.open.then_some("");

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                role="tooltip"
                data-trigger=(self.trigger.into_str())
                data-show-delay=[self.show_delay]
                data-hide-delay=[self.hide_delay]
                data-open=[open]
                (self.get_attrs())
            >
                <Popup
                    placement=(self.placement)
                    self_anchor_id=(self.anchor_id.clone())
                    distance=(self.distance.unwrap_or(DEFAULT_DISTANCE))
                    self_skidding=(self.skidding)
                    flip=true
                    shift=true
                    shift_padding=SHIFT_PADDING
                >
                    // The anchor lives outside of the tooltip, so the popup only
                    // carries its body: the tooltip's content and the arrow.
                    <PopupBody arrow=(self.arrow) hover_bridge=true>
                        <div class=TOOLTIP_BODY>
                            (self.children)
                        </div>
                    </PopupBody>
                </Popup>
            </div>
        }
        .render_to(buffer);
    }
}
