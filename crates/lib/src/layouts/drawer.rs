use std::borrow::Cow;

use derive_more::{AsMut, AsRef};
use hypertext::prelude::{AriaAttributes, GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use strum::{AsRefStr, IntoStaticStr};
use wingy_hypertext_macros::{DynRenderable, Props, const_str};

use crate::appearance::Appearance;
use crate::attributes::{CommonAttributeGetters, CommonAttrs};
use crate::class::{
    BUTTON, DRAWER, DRAWER_BODY, DRAWER_CLOSE, DRAWER_FOOTER, DRAWER_HEADER, DRAWER_HEADER_ACTIONS, DRAWER_TITLE, ICON,
};

/// The direction from which the drawer will open.
#[derive(Copy, Clone, Debug, Default, IntoStaticStr, AsRefStr, PartialEq, Eq)]
#[strum(const_into_str, serialize_all = "kebab-case")]
pub enum DrawerPlacement {
    Top,
    #[default]
    End,
    Bottom,
    Start,
}

/// A panel mirroring Web Awesome's `wa-drawer`: slides in from the edge of the
/// screen to expose additional options and information without navigating away.
/// Rendered as a `<dialog>` element; the open/close behavior (modal display,
/// animations, `data-drawer` click handling, light dismiss, [Escape], body
/// scroll locking, `wg-show`/`wg-hide` events) is implemented in
/// `wingy-hypertext-web` (`layouts::drawer`) and must be wired up on the client
/// with `init_drawers`/`listen_drawers`.
///
/// Any element with `data-drawer="open <id>"` opens the drawer with that id on
/// click, and elements with `data-drawer="close"` inside a drawer close it —
/// the header close button uses exactly this mechanism.
#[derive(Default, AsRef, AsMut, Props, DynRenderable)]
#[const_str(CLASS = DRAWER)]
#[props(builder)]
pub struct Drawer<'a, H: Renderable = (), F: Renderable = ()> {
    #[prop(impl_from)]
    pub placement: DrawerPlacement,

    /// The drawer's label as displayed in the header. You should always include
    /// a relevant label, as it is required for proper accessibility.
    #[prop(into)]
    pub label: Option<Cow<'static, str>>,

    /// Renders the drawer already open: `init_drawers` shows it as a modal on the client.
    pub open: bool,

    /// Disables the header. This will also remove the default close button.
    pub without_header: bool,

    /// When enabled, the drawer will be closed when the user clicks outside of it.
    pub light_dismiss: bool,

    #[as_ref]
    #[as_mut]
    pub attrs: CommonAttrs,

    /// Optional actions added to the header next to the close button.
    #[prop(convert)]
    pub header_actions: Option<H>,

    /// The drawer's footer, usually one or more buttons representing various options.
    #[prop(convert)]
    pub footer: Option<F>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a, H: Renderable, F: Renderable> Drawer<'a, H, F> {
    fn render_to(&self, buffer: &mut Buffer, header_actions: Option<&dyn Renderable>, footer: Option<&dyn Renderable>) {
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
            >
                @if !self.without_header {
                    <header class=DRAWER_HEADER>
                        <h2 class=DRAWER_TITLE>
                            @if let Some(label) = &self.label {
                                (label)
                            } @else {
                                // An invisible character to prevent the header from collapsing
                                "\u{200B}"
                            }
                        </h2>
                        <div class=DRAWER_HEADER_ACTIONS>
                            (header_actions)
                            <button
                                class=(BUTTON, " ", Appearance::PLAIN, " ", DRAWER_CLOSE)
                                type="button"
                                data-drawer="close"
                                aria-label="Close"
                            >
                                <span class=ICON>
                                    (fontawesome::solid::Xmark)
                                </span>
                            </button>
                        </div>
                    </header>
                }
                <div class=DRAWER_BODY>
                    (self.children)
                </div>
                @if let Some(footer) = footer {
                    <footer class=DRAWER_FOOTER>
                        (footer)
                    </footer>
                }
            </dialog>
        }
        .render_to(buffer);
    }
}
