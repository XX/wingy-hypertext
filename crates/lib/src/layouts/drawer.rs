use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::appearance::Appearance::Plain;
use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::attrs;
use crate::class::{
    DRAWER, DRAWER_BODY, DRAWER_CLOSE, DRAWER_FOOTER, DRAWER_HEADER, DRAWER_HEADER_ACTIONS, DRAWER_TITLE, ICON,
};
use crate::components::button::Button;
use crate::layouts::INVISIBLE;

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
/// `wg-show`/`wg-hide` events) is implemented in `wingy-hypertext-web` (`layouts::drawer`)
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
        let id = self.id();
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

#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = DRAWER_HEADER)]
#[props(builder)]
pub struct DrawerHeader<'a, A: Renderable = ()> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    /// Actions added to the header next to the close button.
    #[prop(convert)]
    pub actions: Option<A>,

    pub bare: bool,

    /// The drawer's title as displayed in the header. You should always include
    /// a relevant title, as it is required for proper accessibility.
    pub children: Option<&'a dyn Renderable>,
}

impl<'a, A: Renderable> DrawerHeader<'a, A> {
    fn render_to(&self, buffer: &mut Buffer, actions: Option<&dyn Renderable>) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <header id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                @if self.bare {
                    (self.children)
                } @else {
                    <h2 class=DRAWER_TITLE>
                        @if let Some(title) = &self.children {
                            (title)
                        } @else {
                            // An invisible character to prevent the header from collapsing
                            (INVISIBLE)
                        }
                    </h2>
                    <div class=DRAWER_HEADER_ACTIONS>
                        (actions)
                        <Button class=DRAWER_CLOSE appearance=Plain attrs=(attrs!["data-drawer" = &"close", "aria-label" = &"Close"])>
                            <span class=ICON>
                                (fontawesome::solid::Xmark)
                            </span>
                        </Button>
                    </div>
                }
            </header>
        }
        .render_to(buffer);
    }
}

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER_BODY)]
#[props(builder)]
pub struct DrawerBody<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DrawerBody<'a> {
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

/// The drawer's footer, usually one or more buttons representing various options.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DRAWER_FOOTER)]
#[props(builder)]
pub struct DrawerFooter<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DrawerFooter<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <footer id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
            </footer>
        }
        .render_to(buffer);
    }
}
