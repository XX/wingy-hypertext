use derive_more::{AsMut, AsRef};
use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Buffer, Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext_macros::{Props, const_str};

use crate::appearance::Appearance::Plain;
use crate::attributes::{CommonAttributeGetters, CommonAttributeSetters, CommonAttrs};
use crate::attrs;
use crate::class::{
    DIALOG, DIALOG_BODY, DIALOG_CLOSE, DIALOG_FOOTER, DIALOG_HEADER, DIALOG_HEADER_ACTIONS, DIALOG_TITLE, ICON,
};
use crate::component::button::Button;
use crate::layout::INVISIBLE;

/// A dialog, appears above the page and requires the user's immediate
/// attention: confirmations, forms or focused tasks that interrupt the main
/// flow. Rendered as a `<dialog>` element and composed of [`DialogHeader`],
/// [`DialogBody`] and [`DialogFooter`]; the open/close behavior (modal display,
/// animations, `data-dialog` click handling, light dismiss, [Escape], body
/// scroll locking, `wg-show`/`wg-hide` events) is implemented in
/// `wingy-hypertext-web` (`layout::dialog`) and must be wired up on the client
/// with `init_dialogs`/`listen_dialogs`.
///
/// Any element with `data-dialog="open <id>"` opens the dialog with that id on
/// click, and elements with `data-dialog="close"` inside a dialog close it —
/// the header close button uses exactly this mechanism.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG)]
#[props(builder)]
pub struct Dialog<'a> {
    /// Renders the dialog already open: `init_dialogs` shows it as a modal on the client.
    pub open: bool,

    /// When enabled, the dialog will be closed when the user clicks outside of it.
    pub light_dismiss: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for Dialog<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
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

/// The dialog's header. By default its children are the dialog's title,
/// rendered next to the close button. You should always include a relevant
/// title, as it is required for proper accessibility.
///
/// With `bare` the children are rendered as is: compose the header from
/// [`DialogTitle`] and [`DialogHeaderActions`] to add actions next to the
/// close button.
///
/// Rendered as a `<div>`, not a `<header>`: `<dialog>` doesn't scope the banner
/// landmark away, so a `<header>` here would expose a second `banner` next to
/// the page's own.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG_HEADER)]
#[props(builder)]
pub struct DialogHeader<'a> {
    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DialogHeader<'a> {
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
                        <DialogTitle>(title)</DialogTitle>
                    } @else {
                        <DialogTitle/>
                    }
                    <DialogHeaderActions/>
                }
            </div>
        }
        .render_to(buffer);
    }
}

/// The dialog's title, labeling the dialog.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG_TITLE)]
#[props(builder)]
pub struct DialogTitle<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DialogTitle<'a> {
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

/// The actions in the dialog's header: the children followed by the close button.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG_HEADER_ACTIONS)]
#[props(builder)]
pub struct DialogHeaderActions<'a> {
    pub bare: bool,

    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DialogHeaderActions<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
        let class_line = self.class_line_with(&[Self::CLASS]);
        let style_line = self.style_line_with(&[]);

        rsx! {
            <div id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                (self.children)
                @if !self.bare {
                    <Button class=DIALOG_CLOSE appearance=Plain attrs=(attrs!["data-dialog" = &"close", "aria-label" = &"Close"])>
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

#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG_BODY)]
#[props(builder)]
pub struct DialogBody<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DialogBody<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
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

/// The dialog's footer, usually one or more buttons representing various options.
///
/// Rendered as a `<div>` for the same reason [`DialogHeader`] is: a `<footer>`
/// inside a `<dialog>` would duplicate the page's `contentinfo` landmark.
#[derive(Default, AsRef, AsMut, Props)]
#[const_str(CLASS = DIALOG_FOOTER)]
#[props(builder)]
pub struct DialogFooter<'a> {
    #[as_ref]
    #[as_mut]
    pub attributes: CommonAttrs<'a>,

    pub children: Option<&'a dyn Renderable>,
}

impl<'a> Renderable for DialogFooter<'a> {
    fn render_to(&self, buffer: &mut Buffer) {
        let id = self.not_empty_id();
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
