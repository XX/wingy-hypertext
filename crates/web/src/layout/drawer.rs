//! An implementation of the `Drawer` behavior: a native modal `<dialog>` driven by
//! the shared modal behavior in [`crate::util::modal`] — animations,
//! declarative `data-drawer="open <id>"` / `data-drawer="close"` triggers, [Escape]
//! and light-dismiss handling, body scroll locking, focus restoring, and the
//! cancelable `wg-show`/`wg-hide` (plus `wg-after-show`/`wg-after-hide`)
//! lifecycle events. The state lives in the DOM, matching the markup produced
//! by `wingy_hypertext::layout::drawer`.

use const_format::concatcp;
use web_sys::Element;
use wingy_hypertext::class::{DRAWER, DRAWER_HEADER, DRAWER_TITLE};

use crate::util::modal::{self, ModalKind};

pub const KIND: ModalKind = ModalKind {
    class: DRAWER,
    selector: concatcp!('.', DRAWER),
    trigger: "data-drawer",
    trigger_selector: "[data-drawer]",
    title_selector: concatcp!(":scope > .", DRAWER_HEADER, " .", DRAWER_TITLE),
};

/// Whether the drawer is shown, including while its hide animation runs.
pub fn is_open(drawer: &Element) -> bool {
    modal::is_shown(drawer)
}

pub fn open_drawer(drawer: Element) {
    modal::open_modal(drawer);
}

/// Requests to close the drawer; `source` is reported to `wg-hide` listeners.
pub fn close_drawer(drawer: Element, source: Element) {
    modal::close_modal(drawer, source);
}

/// Labels the drawers by their titles and shows the ones rendered with the
/// `data-open` attribute. Run after every render.
pub fn init_drawers() {
    modal::init_modals(&KIND);
}

/// Installs the document-level listeners driving declarative open/close,
/// light dismiss and [Escape] for every drawer on the page.
pub fn listen_drawers() {
    modal::listen_modals(&KIND);
}
