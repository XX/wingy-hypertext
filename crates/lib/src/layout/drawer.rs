use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance::Plain;
use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::class::{
    DRAWER, DRAWER_BODY, DRAWER_CLOSE, DRAWER_FOOTER, DRAWER_HEADER, DRAWER_HEADER_ACTIONS, DRAWER_TITLE, ICON,
};
use crate::component::button::Button;
use crate::layout::INVISIBLE;
use crate::{attrs, div_component};

/// The direction from which the drawer will open.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum DrawerPlacement {
    #[default]
    Start,
    Top,
    End,
    Bottom,
}

/// A panel, slides in from the edge of the screen to expose additional
/// options and information without navigating away. Rendered as a `<dialog>`
/// element; the open/close behavior (modal display, animations, `data-drawer`
/// click handling, light dismiss, [Escape], body scroll locking,
/// `wg-show`/`wg-hide` events) is implemented in `wingy-hypertext-web` (`layout::drawer`)
/// and must be wired up on the client with `init_drawers`/`listen_drawers`.
///
/// Any element with `data-drawer="open <id>"` opens the drawer with that id on
/// click, and elements with `data-drawer="close"` inside a drawer close it —
/// the header close button uses exactly this mechanism.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER)]
#[props(builder)]
pub struct Drawer<'a> {
    #[prop(impl_from)]
    pub placement: DrawerPlacement,

    /// Renders the drawer already open: `init_drawers` shows it as a modal on the client.
    pub open: bool,

    /// When enabled, the drawer will be closed when the user clicks outside of it.
    pub light_dismiss: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Drawer<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS, self.placement.into_str()]);
        let style_line = self.style_line_with(&[]);

        let open = self.open.then_some("");
        let light_dismiss = self.light_dismiss.then_some("");

        rsx! {
            <dialog
                id=[id]
                class=[&class_line]
                style=[&style_line]
                data-open=[open]
                data-light-dismiss=[light_dismiss]
                (self.get_attrs())
            >
                (self.children)
            </dialog>
        }
        .render_to(buffer);
    }
}

/// The drawer's header. By default its children are the drawer's title,
/// rendered next to the close button. You should always include a relevant
/// title, as it is required for proper accessibility.
///
/// With `bare` the children are rendered as is: compose the header from
/// [`DrawerTitle`] and [`DrawerHeaderActions`] to add actions next to the
/// close button.
///
/// Rendered as a `<div>`, not a `<header>`: `<dialog>` doesn't scope the banner
/// landmark away, so a `<header>` here would expose a second `banner` next to
/// the page's own.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER_HEADER)]
#[props(builder)]
pub struct DrawerHeader<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub bare: bool,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DrawerHeader<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if self.bare {
                    (self.children)
                } @else {
                    @if let Some(title) = &self.children {
                        <DrawerTitle>(title)</DrawerTitle>
                    } @else {
                        <DrawerTitle/>
                    }
                    <DrawerHeaderActions/>
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// The drawer's title, labeling the drawer.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER_TITLE)]
#[props(builder)]
pub struct DrawerTitle<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DrawerTitle<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <h2 id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if let Some(title) = &self.children {
                    (title)
                } @else {
                    // An invisible character to prevent the header from collapsing
                    (INVISIBLE)
                }
            </h2>
        }
        .render_to(buffer);
    }
}

/// The actions in the drawer's header: the children followed by the close button.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER_HEADER_ACTIONS)]
#[props(builder)]
pub struct DrawerHeaderActions<'a> {
    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DrawerHeaderActions<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
                @if !self.bare {
                    <Button class=DRAWER_CLOSE appearance=Plain attrs=(attrs!["data-drawer" = &"close", "aria-label" = &"Close"])>
                        <span class=ICON>
                            (fontawesome::solid::Xmark)
                        </span>
                    </Button>
                }
            </div>
        }
        .render_to(buffer);
    }
}

div_component!(DrawerBody, DRAWER_BODY);

div_component!(
    /// The drawer's footer, usually one or more buttons representing various options.
    ///
    /// Rendered as a `<div>` for the same reason [`DrawerHeader`] is: a `<footer>`
    /// inside a `<dialog>` would duplicate the page's `contentinfo` landmark.
    DrawerFooter,
    DRAWER_FOOTER
);
