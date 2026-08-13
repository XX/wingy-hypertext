//! A Rust port of the `wa-popup` positioning behavior: a floating-ui subset
//! (offset, flip with best-fit fallback, shift, size syncing, auto-size,
//! arrow) plus repositioning on scroll and resize. The configuration is read
//! from `data-*` attributes on the `.popup` host element rendered by
//! `wingy_hypertext::helpers::popup`, and the positioned container is the
//! `.popup-body` child (`position: fixed`, so all math is in viewport
//! coordinates).

use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::access::{CastToElement, CastToHtmlElement};
use web_sys::{AddEventListenerOptions, Element, HtmlElement};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

impl Side {
    fn opposite(self) -> Self {
        match self {
            Self::Top => Self::Bottom,
            Self::Bottom => Self::Top,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

    fn is_vertical(self) -> bool {
        matches!(self, Self::Top | Self::Bottom)
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Align {
    Start,
    Center,
    End,
}

struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    fn from_element(element: &Element) -> Self {
        let rect = element.get_bounding_client_rect();
        Self {
            x: rect.x(),
            y: rect.y(),
            width: rect.width(),
            height: rect.height(),
        }
    }

    fn right(&self) -> f64 {
        self.x + self.width
    }

    fn bottom(&self) -> f64 {
        self.y + self.height
    }

    fn center_x(&self) -> f64 {
        self.x + self.width / 2.0
    }

    fn center_y(&self) -> f64 {
        self.y + self.height / 2.0
    }
}

/// The positioning options of a popup, mirroring the `data-*` attributes of the
/// `.popup` host element. [`PopupConfig::new`] builds the options for a
/// placement with everything else turned off, so other components (like the
/// dropdown's submenus) can position elements without a `.popup` host.
pub struct PopupConfig {
    pub side: Side,
    pub align: Align,
    pub distance: f64,
    pub skidding: f64,
    pub flip: bool,
    pub flip_padding: f64,
    pub shift: bool,
    pub shift_padding: f64,
    pub sync: Option<String>,
    pub auto_size: Option<String>,
    pub auto_size_padding: f64,
    pub arrow_placement: String,
    pub arrow_padding: f64,
}

impl PopupConfig {
    pub fn new(placement: &str) -> Self {
        let (side, align) = parse_placement(placement);
        Self {
            side,
            align,
            distance: 0.0,
            skidding: 0.0,
            flip: false,
            flip_padding: 0.0,
            shift: false,
            shift_padding: 0.0,
            sync: None,
            auto_size: None,
            auto_size_padding: 0.0,
            arrow_placement: String::new(),
            arrow_padding: 0.0,
        }
    }
}

fn parse_placement(placement: &str) -> (Side, Align) {
    let (side, align) = placement.split_once('-').unwrap_or((placement, ""));
    let side = match side {
        "bottom" => Side::Bottom,
        "left" => Side::Left,
        "right" => Side::Right,
        _ => Side::Top,
    };
    let align = match align {
        "start" => Align::Start,
        "end" => Align::End,
        _ => Align::Center,
    };
    (side, align)
}

pub fn placement_str(side: Side, align: Align) -> String {
    match align {
        Align::Start => format!("{}-start", side.as_str()),
        Align::End => format!("{}-end", side.as_str()),
        Align::Center => side.as_str().to_string(),
    }
}

fn number_attr(host: &Element, name: &str, default: f64) -> f64 {
    host.get_attribute(name)
        .and_then(|value| value.parse().ok())
        .unwrap_or(default)
}

fn read_config(host: &Element) -> PopupConfig {
    let placement = host.get_attribute("data-placement").unwrap_or_default();
    let (side, align) = parse_placement(&placement);

    PopupConfig {
        side,
        align,
        distance: number_attr(host, "data-distance", 0.0),
        skidding: number_attr(host, "data-skidding", 0.0),
        flip: host.has_attribute("data-flip"),
        flip_padding: number_attr(host, "data-flip-padding", 0.0),
        shift: host.has_attribute("data-shift"),
        shift_padding: number_attr(host, "data-shift-padding", 0.0),
        sync: host.get_attribute("data-sync"),
        auto_size: host.get_attribute("data-auto-size"),
        auto_size_padding: number_attr(host, "data-auto-size-padding", 0.0),
        arrow_placement: host
            .get_attribute("data-arrow-placement")
            .unwrap_or_else(|| "anchor".to_string()),
        arrow_padding: number_attr(host, "data-arrow-padding", 10.0),
    }
}

fn popup_body(host: &Element) -> Option<HtmlElement> {
    host.query_selector(":scope > .popup-body").ok()??.maybe_into_html()
}

/// The anchor is either an external element referenced by id (`data-anchor`)
/// or the first child of the host that isn't a part of the popup itself.
fn anchor_element(host: &Element) -> Option<Element> {
    if let Some(id) = host.get_attribute("data-anchor") {
        return dom::existing::document().get_element_by_id(&id);
    }
    host.query_selector(":scope > :not(.popup-body):not(.popup-hover-bridge)")
        .ok()?
}

fn viewport_size() -> (f64, f64) {
    let window = dom::existing::window();
    let width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
    let height = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
    (width, height)
}

/// The free space between the anchor's `side` edge and the viewport.
fn side_space(side: Side, anchor: &Rect, viewport: (f64, f64)) -> f64 {
    match side {
        Side::Top => anchor.y,
        Side::Bottom => viewport.1 - anchor.bottom(),
        Side::Left => anchor.x,
        Side::Right => viewport.0 - anchor.right(),
    }
}

fn host_style(host: &Element) -> Option<web_sys::CssStyleDeclaration> {
    host.maybe_as_html().map(|html| html.style())
}

/// Forces the popup to recalculate and reposition itself.
pub fn reposition(host: &Element) -> Option<()> {
    let config = read_config(host);
    let anchor = anchor_element(host)?;
    let popup = popup_body(host)?;

    let (side, align) = place(&anchor, &popup, &config, host)?;

    host.set_attribute("data-current-placement", &placement_str(side, align))
        .ok();

    // The arrow is aligned against the popup's final position
    let anchor_rect = Rect::from_element(&anchor);
    let popup_rect = Rect::from_element(&popup);
    position_arrow(&config, &popup, &anchor_rect, side, popup_rect.x, popup_rect.y);
    update_hover_bridge(host, &anchor, &popup, side);

    Some(())
}

/// Positions the `position: fixed` element `popup` next to `anchor` following
/// `config`, and returns the resolved placement. The
/// `--auto-size-available-width/height` custom properties are exposed on
/// `size_host` (the `.popup` host for a popup, the floating element itself
/// otherwise).
pub fn place(
    anchor: &Element,
    popup: &HtmlElement,
    config: &PopupConfig,
    size_host: &Element,
) -> Option<(Side, Align)> {
    let anchor_rect = Rect::from_element(anchor);
    let viewport = viewport_size();

    // Sync the popup's dimensions to the anchor first, so all further
    // measurements see the final size.
    let sync = config.sync.as_deref().unwrap_or("");
    if sync == "width" || sync == "both" {
        popup
            .style()
            .set_property("width", &format!("{}px", anchor_rect.width))
            .ok();
    } else {
        popup.style().remove_property("width").ok();
    }
    if sync == "height" || sync == "both" {
        popup
            .style()
            .set_property("height", &format!("{}px", anchor_rect.height))
            .ok();
    } else {
        popup.style().remove_property("height").ok();
    }

    // Flip to the opposite side when the preferred one can't fit the popup;
    // when neither fits, use the side with the most space (best-fit).
    let mut side = config.side;
    if config.flip {
        let main_size = |side: Side| {
            if side.is_vertical() {
                popup.offset_height() as f64
            } else {
                popup.offset_width() as f64
            }
        };
        let fits = |side: Side| {
            side_space(side, &anchor_rect, viewport) >= main_size(side) + config.distance + config.flip_padding
        };

        if !fits(side) {
            let opposite = side.opposite();
            if fits(opposite) || side_space(opposite, &anchor_rect, viewport) > side_space(side, &anchor_rect, viewport)
            {
                side = opposite;
            }
        }
    }

    // Expose the available space so auto-sized popups can adhere to it,
    // then re-measure: the `--auto-size-available-*` variables cap the
    // popup's max width/height in CSS.
    let style = host_style(size_host)?;
    let auto_size = config.auto_size.as_deref().unwrap_or("");
    if auto_size == "vertical" || auto_size == "both" {
        let available = match side {
            Side::Top | Side::Bottom => side_space(side, &anchor_rect, viewport) - config.distance,
            _ => viewport.1 - config.auto_size_padding,
        } - config.auto_size_padding;
        style
            .set_property("--auto-size-available-height", &format!("{}px", available.max(0.0)))
            .ok();
    } else {
        style.remove_property("--auto-size-available-height").ok();
    }
    if auto_size == "horizontal" || auto_size == "both" {
        let available = match side {
            Side::Left | Side::Right => side_space(side, &anchor_rect, viewport) - config.distance,
            _ => viewport.0 - config.auto_size_padding,
        } - config.auto_size_padding;
        style
            .set_property("--auto-size-available-width", &format!("{}px", available.max(0.0)))
            .ok();
    } else {
        style.remove_property("--auto-size-available-width").ok();
    }

    let popup_width = popup.offset_width() as f64;
    let popup_height = popup.offset_height() as f64;

    // The main-axis coordinate comes from the side, the cross-axis one from
    // the alignment plus skidding.
    let main = match side {
        Side::Top => anchor_rect.y - config.distance - popup_height,
        Side::Bottom => anchor_rect.bottom() + config.distance,
        Side::Left => anchor_rect.x - config.distance - popup_width,
        Side::Right => anchor_rect.right() + config.distance,
    };
    let mut cross = if side.is_vertical() {
        match config.align {
            Align::Start => anchor_rect.x,
            Align::Center => anchor_rect.center_x() - popup_width / 2.0,
            Align::End => anchor_rect.right() - popup_width,
        }
    } else {
        match config.align {
            Align::Start => anchor_rect.y,
            Align::Center => anchor_rect.center_y() - popup_height / 2.0,
            Align::End => anchor_rect.bottom() - popup_height,
        }
    } + config.skidding;

    // Shift the popup along the cross axis to keep it in view
    if config.shift {
        let (viewport_size, popup_size) = if side.is_vertical() {
            (viewport.0, popup_width)
        } else {
            (viewport.1, popup_height)
        };
        let max = viewport_size - popup_size - config.shift_padding;
        cross = cross.min(max).max(config.shift_padding);
    }

    let (x, y) = if side.is_vertical() {
        (cross, main)
    } else {
        (main, cross)
    };
    popup.style().set_property("left", &format!("{x}px")).ok();
    popup.style().set_property("top", &format!("{y}px")).ok();

    Some((side, config.align))
}

fn position_arrow(config: &PopupConfig, popup: &HtmlElement, anchor_rect: &Rect, side: Side, x: f64, y: f64) {
    let Some(arrow) = popup
        .query_selector(":scope > .arrow")
        .ok()
        .flatten()
        .and_then(|arrow| arrow.maybe_into_html())
    else {
        return;
    };

    let style = arrow.style();
    for property in ["top", "right", "bottom", "left"] {
        style.remove_property(property).ok();
    }

    // The properties to align the arrow along the popup's cross axis
    let (start, end) = if side.is_vertical() {
        ("left", "right")
    } else {
        ("top", "bottom")
    };

    match config.arrow_placement.as_str() {
        "start" => {
            let value = format!("calc({}px - var(--arrow-padding-offset))", config.arrow_padding);
            style.set_property(start, &value).ok();
        },
        "end" => {
            let value = format!("calc({}px - var(--arrow-padding-offset))", config.arrow_padding);
            style.set_property(end, &value).ok();
        },
        "center" => {
            style.set_property(start, "calc(50% - var(--arrow-size-diagonal))").ok();
        },
        // Anchor (default): align the arrow as close to the center of the
        // anchor as possible, considering available space and arrow padding
        _ => {
            let (anchor_center, popup_start, popup_size, arrow_size) = if side.is_vertical() {
                (
                    anchor_rect.center_x(),
                    x,
                    popup.offset_width() as f64,
                    arrow.offset_width() as f64,
                )
            } else {
                (
                    anchor_rect.center_y(),
                    y,
                    popup.offset_height() as f64,
                    arrow.offset_height() as f64,
                )
            };
            let offset = (anchor_center - popup_start - arrow_size / 2.0)
                .min(popup_size - arrow_size - config.arrow_padding)
                .max(config.arrow_padding);
            style.set_property(start, &format!("{offset}px")).ok();
        },
    }

    // Overlap the arrow with the inside edge of the popup border
    style
        .set_property(
            side.opposite().as_str(),
            "calc(var(--arrow-base-offset) - var(--arrow-size-diagonal))",
        )
        .ok();
}

/// Fills the gap between the anchor and the popup with the hover bridge
/// polygon so the pointer never technically leaves them.
fn update_hover_bridge(host: &Element, anchor: &Element, popup: &HtmlElement, side: Side) {
    if host
        .query_selector(":scope > .popup-hover-bridge")
        .ok()
        .flatten()
        .is_none()
    {
        return;
    }
    let Some(style) = host_style(host) else {
        return;
    };

    let anchor_rect = Rect::from_element(anchor);
    let popup_rect = Rect::from_element(popup);

    let (corners, values): ([&str; 8], [f64; 8]) = if side.is_vertical() {
        if anchor_rect.y < popup_rect.y {
            // Anchor is above
            (
                [
                    "top-left-x",
                    "top-left-y",
                    "top-right-x",
                    "top-right-y",
                    "bottom-left-x",
                    "bottom-left-y",
                    "bottom-right-x",
                    "bottom-right-y",
                ],
                [
                    anchor_rect.x,
                    anchor_rect.bottom(),
                    anchor_rect.right(),
                    anchor_rect.bottom(),
                    popup_rect.x,
                    popup_rect.y,
                    popup_rect.right(),
                    popup_rect.y,
                ],
            )
        } else {
            // Anchor is below
            (
                [
                    "top-left-x",
                    "top-left-y",
                    "top-right-x",
                    "top-right-y",
                    "bottom-left-x",
                    "bottom-left-y",
                    "bottom-right-x",
                    "bottom-right-y",
                ],
                [
                    popup_rect.x,
                    popup_rect.bottom(),
                    popup_rect.right(),
                    popup_rect.bottom(),
                    anchor_rect.x,
                    anchor_rect.y,
                    anchor_rect.right(),
                    anchor_rect.y,
                ],
            )
        }
    } else if anchor_rect.x < popup_rect.x {
        // Anchor is on the left
        (
            [
                "top-left-x",
                "top-left-y",
                "top-right-x",
                "top-right-y",
                "bottom-left-x",
                "bottom-left-y",
                "bottom-right-x",
                "bottom-right-y",
            ],
            [
                anchor_rect.right(),
                anchor_rect.y,
                popup_rect.x,
                popup_rect.y,
                anchor_rect.right(),
                anchor_rect.bottom(),
                popup_rect.x,
                popup_rect.bottom(),
            ],
        )
    } else {
        // Anchor is on the right
        (
            [
                "top-left-x",
                "top-left-y",
                "top-right-x",
                "top-right-y",
                "bottom-left-x",
                "bottom-left-y",
                "bottom-right-x",
                "bottom-right-y",
            ],
            [
                popup_rect.right(),
                popup_rect.y,
                anchor_rect.x,
                anchor_rect.y,
                popup_rect.right(),
                popup_rect.bottom(),
                anchor_rect.x,
                anchor_rect.bottom(),
            ],
        )
    };

    for (corner, value) in corners.iter().zip(values) {
        style
            .set_property(&format!("--hover-bridge-{corner}"), &format!("{value}px"))
            .ok();
    }
}

/// Activates or deactivates the positioning logic: `.popup-body` is only
/// displayed while the host has the `active` class.
pub fn set_popup_active(host: &Element, active: bool) {
    host.class_list().toggle_with_force("active", active).ok();

    if active {
        reposition(host);
    } else {
        host.remove_attribute("data-current-placement").ok();
        if let Some(style) = host_style(host) {
            style.remove_property("--auto-size-available-width").ok();
            style.remove_property("--auto-size-available-height").ok();
        }
    }
}

fn reposition_active_popups() {
    let Ok(popups) = dom::existing::document().query_selector_all(".popup.active") else {
        return;
    };
    for i in 0..popups.length() {
        if let Some(host) = popups.get(i).and_then(|node| node.maybe_into_element()) {
            reposition(&host);
        }
    }
}

/// Positions every active `.popup` on the page. Run it after every render.
pub fn init_popups() {
    reposition_active_popups();
}

/// Installs the window-level listeners that keep active popups anchored while
/// the page scrolls or resizes (the `autoUpdate` part of floating-ui).
pub fn listen_popups() {
    let window = dom::existing::window();

    // Scroll events don't bubble, but they do capture, so a capturing
    // listener on the window sees scrolling of any nested container too.
    let options = AddEventListenerOptions::new();
    options.set_capture(true);
    options.set_passive(true);
    window.add_steady_event_listener_with_options(
        "scroll",
        |_| {
            reposition_active_popups();
        },
        &options,
    );

    window.add_steady_event_listener("resize", |_| {
        reposition_active_popups();
    });
}
