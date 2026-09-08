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

/// The stylesheet [`Page`] ships alongside its markup, standing in for the one
/// `wa-page` renders into its shadow root.
///
/// It is a container query on the page rather than a media query on the viewport,
/// so it answers the question the layout actually asks — how wide is *this page* —
/// and stays right for a page laid out inside a narrower column. That also makes it
/// the single source of truth: it needs no JS, so it is correct from the first paint
/// rather than from hydration, and nothing has to agree with it afterwards.
///
/// It has to be rendered rather than kept in `page.css` because a container query
/// condition cannot read a custom property, and the breakpoint is per page.
fn mobile_stylesheet(breakpoint: &str, navigation: bool) -> String {
    // Utilities from Web Awesome that mean "the page is narrow", plus the escape
    // hatch for an app-supplied navigation toggle. Upstream keys these off the
    // `view` attribute in `layers.css`; here they belong to the same query as
    // everything else, so there is one breakpoint and no cascade to fight.
    let common = ".page .wa-desktop-only{display:none!important}\
                  .page .wa-mobile-only{display:revert!important}\
                  .page [data-toggle-nav]{display:revert}";

    let navigation_rules = if navigation {
        ".page .page-menu{display:none}.page .page-navigation-toggle{display:inline-flex}"
    } else {
        ""
    };
    let menu_width = if navigation { "0" } else { "auto" };

    format!(
        "@container page (width < {breakpoint}){{\
         .page{{--menu-width:{menu_width};--aside-width:auto}}{common}{navigation_rules}}}"
    )
}

/// The page shell: a header, a body splitting into menu, main and aside, and a
/// footer.
///
/// The responsive layout comes entirely from the container query in
/// [`mobile_stylesheet`], which the page renders alongside its markup. The `view`
/// attribute is published for application code and read by `init_page` in
/// `wingy-hypertext-web`, which needs to know when to move the navigation into the
/// drawer — but no styling depends on it, so the page is laid out correctly before
/// any script has run.
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

        // The client parses the attribute and the container query above uses the
        // same value, so a rejected breakpoint has to be rejected for both.
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

        // Reflects the layout the page is in. Styling doesn't consult it — the
        // container query does that — so shipping the wide value is safe even when
        // the page turns out to be narrow. `view` is not a standard HTML attribute,
        // so it goes through the named-attribute escape hatch.
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
/// Place it in the page header: it stays hidden until the page collapses, which
/// the rendered stylesheet decides. Opening is the ordinary declarative drawer
/// trigger, so no page-specific click handling is involved. An app-supplied toggle
/// works the same way if it carries `data-toggle-nav`.
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
