use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Raw, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{Props, const_str};

use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::attrs;
use crate::class::{ICON, PAGE, PAGE_BODY, PAGE_MENU, PAGE_NAVIGATION_DRAWER, PAGE_NAVIGATION_TOGGLE};
use crate::layout::drawer::{Drawer, DrawerBody, DrawerHeader, DrawerPlacement};

/// The id of the drawer [`Page`] renders for the mobile navigation, and the one
/// [`PageNavigationToggle`] opens.
pub const NAVIGATION_ID: &str = "page-navigation";

/// The width below which [`Page`] switches to its mobile layout, unless
/// `mobile_breakpoint` says otherwise. Kept in step with `init_page`.
pub const DEFAULT_MOBILE_BREAKPOINT: &str = "920px";

/// Narrows a caller-supplied breakpoint to something that cannot escape the
/// `@media` prelude it is interpolated into: digits, a decimal point and a unit.
/// Anything else falls back to the default.
fn safe_breakpoint(breakpoint: Option<&str>) -> &str {
    let Some(breakpoint) = breakpoint.map(str::trim) else {
        return DEFAULT_MOBILE_BREAKPOINT;
    };

    let valid = breakpoint.starts_with(|char: char| char.is_ascii_digit() || char == '.')
        && breakpoint
            .chars()
            .all(|char| char.is_ascii_digit() || char == '.' || char.is_ascii_alphabetic() || char == '%');

    if valid { breakpoint } else { DEFAULT_MOBILE_BREAKPOINT }
}

/// The stylesheet [`Page`] ships alongside its markup, mirroring the one
/// `wa-page` renders into its shadow root.
///
/// Until the client has measured the page, `view` reads `desktop` — which is a
/// lie on a narrow screen, and a server-rendered page would land there showing
/// the full desktop grid until the WASM module boots. A media query needs no
/// JS, so it covers exactly that window. It has to be rendered rather than kept
/// in `page.css` because the breakpoint is per-page.
fn mobile_stylesheet(breakpoint: &str, navigation: bool) -> String {
    let navigation_rules = if navigation {
        ".page .page-menu{display:none}.page .page-navigation-toggle{display:inline-flex}"
    } else {
        ""
    };
    let menu_width = if navigation { "0" } else { "auto" };

    format!(
        "@media screen and (width < {breakpoint}){{\
         .page{{--menu-width:{menu_width};--aside-width:auto}}{navigation_rules}}}"
    )
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = PAGE)]
#[props(builder)]
pub struct Page<'a> {
    /// Renders the drawer holding the navigation on narrow pages. Pair it with a
    /// [`PageNavigationToggle`] in the header; on mobile the client moves the
    /// [`PageMenu`] content into the drawer, and moves it back on desktop.
    pub navigation: bool,

    /// The width below which the page switches to the mobile layout, as a CSS
    /// length (`px`, `rem` and `em` are resolved). Defaults to `920px`.
    #[prop(into)]
    pub mobile_breakpoint: Option<Cow<'a, str>>,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Page<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);
        let breakpoint = safe_breakpoint(self.mobile_breakpoint.as_deref());
        let stylesheet = mobile_stylesheet(breakpoint, self.navigation);

        // The client parses the attribute and the media query above uses the same
        // value, so a rejected breakpoint has to be rejected for both.
        let breakpoint_attr = self.mobile_breakpoint.is_some().then_some(breakpoint);

        let navigation = self.navigation.then(|| {
            rsx! {
                <Drawer
                    id=NAVIGATION_ID
                    class=PAGE_NAVIGATION_DRAWER
                    placement=(DrawerPlacement::Start)
                    light_dismiss=true
                >
                    <DrawerHeader/>
                    <DrawerBody/>
                </Drawer>
            }
        });

        // `view` is not a standard HTML attribute, so it goes through the
        // named-attribute escape hatch rather than rsx's typed attributes.
        let view = attrs!["view" = &"desktop"];

        rsx! {
            <div
                id=[id]
                class=[&class_line]
                style=[&style_line]
                (&view[..])
                data-mobile-breakpoint=[breakpoint_attr]
                (self.get_attrs())
            >
                // XSS SAFETY: the only interpolated value is the breakpoint, which
                // `safe_breakpoint` restricts to a CSS length.
                <style>(Raw::dangerously_create(&stylesheet))</style>
                (self.children)
                (navigation)
            </div>
        }
        .render_to(buffer);
    }
}

/// The hamburger opening the mobile navigation drawer of a [`Page`].
///
/// Place it in the page header: `layers.css` hides anything carrying
/// `data-toggle-nav` while the page is in the desktop view, so it appears only
/// when the navigation has collapsed. Opening is the ordinary declarative drawer
/// trigger, so no page-specific click handling is involved.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = PAGE_NAVIGATION_TOGGLE)]
#[props(builder)]
pub struct PageNavigationToggle<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// Replaces the default hamburger icon.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for PageNavigationToggle<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <button
                id=[id]
                class=[&class_line]
                style=[&style_line]
                type="button"
                data-toggle-nav=""
                data-drawer="open page-navigation"
                aria-label="Open navigation"
                aria-controls=NAVIGATION_ID
                (self.get_attrs())
            >
                @if let Some(children) = self.children {
                    (children)
                } @else {
                    <span class=ICON>
                        (fontawesome::solid::Bars)
                    </span>
                }
            </button>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = PAGE_BODY)]
#[props(builder)]
pub struct PageBody<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for PageBody<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = PAGE_MENU)]
#[props(builder)]
pub struct PageMenu<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for PageMenu<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </div>
        }
        .render_to(buffer);
    }
}
