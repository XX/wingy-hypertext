//! A registry of named, data-attribute driven actions dispatched from `click` events.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::JSON;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event};

use crate::dom;

/// Context passed to an action handler, mirroring `{ event, element }` in JS.
pub struct ActionCtx {
    pub event: Event,
    pub element: Element,
}

type ActionHandler = Rc<dyn Fn(&JsValue, &ActionCtx)>;

thread_local! {
    static ACTIONS: RefCell<HashMap<String, ActionHandler>> = RefCell::new(HashMap::new());
}

pub fn register_action(name: impl Into<String>, handler: impl Fn(&JsValue, &ActionCtx) + 'static) {
    ACTIONS.with(|actions| {
        actions.borrow_mut().insert(name.into(), Rc::new(handler));
    });
}

pub fn unregister_action(name: impl AsRef<str>) {
    ACTIONS.with(|actions| {
        actions.borrow_mut().remove(name.as_ref());
    });
}

pub fn run_action(name: &str, args: &JsValue, ctx: &ActionCtx) -> Option<()> {
    let handler = ACTIONS.with(|actions| actions.borrow().get(name).cloned())?;
    handler(args, ctx);
    Some(())
}

/// Resolves the nearest `[data-action]` ancestor of the event target and runs its action.
pub fn dispatch_action(event: Event) -> Option<()> {
    let element = event
        .target()?
        .dyn_into::<Element>()
        .ok()?
        .closest("[data-action]")
        .ok()
        .flatten()?;

    let name = element.get_attribute("data-action").unwrap_or_default();
    let args_raw = element.get_attribute("data-args");
    let args = JSON::parse(args_raw.as_deref().unwrap_or("{}")).expect_throw("cannot parse `data-args` as JSON");

    run_action(&name, &args, &ActionCtx { event, element })
}

/// Installs a delegated `click` listener on `document.body`.
pub fn listen_click_actions() {
    let callback = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        dispatch_action(event);
    });

    dom::body()
        .add_event_listener_with_callback("click", callback.as_ref().unchecked_ref())
        .unwrap_throw();

    callback.forget();
}
