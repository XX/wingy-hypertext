//! Internal type erasure for component children.

use hypertext::Renderable;

/// Erases a concrete renderable type behind `&dyn Renderable`.
///
/// This keeps binary size (most notably WASM) independent of how many distinct
/// renderable types a component is instantiated with.
pub(crate) fn as_dyn<R: Renderable>(renderable: &Option<R>) -> Option<&dyn Renderable> {
    renderable.as_ref().map(|renderable| renderable as _)
}
