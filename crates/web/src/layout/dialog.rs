//! An implementation of the `Dialog` behavior: a native modal `<dialog>` driven by
//! the shared modal behavior in [`crate::util::modal`] — animations,
//! declarative `data-dialog="open <id>"` / `data-dialog="close"` triggers, [Escape]
//! and light-dismiss handling, body scroll locking, focus restoring, and the
//! cancelable `wg-show`/`wg-hide` (plus `wg-after-show`/`wg-after-hide`)
//! lifecycle events. The state lives in the DOM, matching the markup produced
//! by `wingy_hypertext::layout::dialog`.

use const_format::concatcp;
use web_sys::Element;
use wingy_hypertext::class::{DIALOG, DIALOG_HEADER, DIALOG_TITLE};

use crate::util::modal::{self, ModalKind};

pub const KIND: ModalKind = ModalKind {
    class: DIALOG,
    selector: concatcp!('.', DIALOG),
    trigger: "data-dialog",
    trigger_selector: "[data-dialog]",
    title_selector: concatcp!(":scope > .", DIALOG_HEADER, " .", DIALOG_TITLE),
};

/// Whether the dialog is shown, including while its hide animation runs.
pub fn is_open(dialog: &Element) -> bool {
    modal::is_shown(dialog)
}

pub fn open_dialog(dialog: Element) {
    modal::open_modal(dialog);
}

/// Requests to close the dialog; `source` is reported to `wg-hide` listeners.
pub fn close_dialog(dialog: Element, source: Element) {
    modal::close_modal(dialog, source);
}

/// Labels the dialogs by their titles and shows the ones rendered with the
/// `data-open` attribute. Run after every render.
pub fn init_dialogs() {
    modal::init_modals(&KIND);
}

/// Installs the document-level listeners driving declarative open/close,
/// light dismiss and [Escape] for every dialog on the page.
pub fn listen_dialogs() {
    modal::listen_modals(&KIND);
}
