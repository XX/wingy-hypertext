//! Port of `webassets/js/utils/action.js`.
//!
//! A registry of named, data-attribute driven actions dispatched from `click` events.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event};

use crate::dom::document;

/// Context passed to an action handler, mirroring `{ event, element }` in JS.
pub struct ActionCtx {
    pub event: Event,
    pub element: Element,
}

type ActionFn = Rc<dyn Fn(&JsValue, &ActionCtx)>;

thread_local! {
    static ACTIONS: RefCell<HashMap<String, ActionFn>> = RefCell::new(HashMap::new());
}

/// Registers an action handler under `name`. `args` is the parsed `data-args` JSON value.
pub fn register_action(name: &str, f: impl Fn(&JsValue, &ActionCtx) + 'static) {
    ACTIONS.with(|actions| {
        actions.borrow_mut().insert(name.to_string(), Rc::new(f));
    });
}

pub fn unregister_action(name: &str) {
    ACTIONS.with(|actions| {
        actions.borrow_mut().remove(name);
    });
}

pub fn run_action(name: &str, args: &JsValue, ctx: &ActionCtx) {
    let handler = ACTIONS.with(|actions| actions.borrow().get(name).cloned());
    if let Some(handler) = handler {
        handler(args, ctx);
    }
}

/// Resolves the nearest `[data-action]` ancestor of the event target and runs its action.
pub fn dispatch_action(event: &Event) {
    let Some(element) = event.target().and_then(|t| t.dyn_into::<Element>().ok()) else {
        return;
    };
    let Some(action_el) = element.closest("[data-action]").ok().flatten() else {
        return;
    };

    let name = action_el.get_attribute("data-action").unwrap_or_default();
    let args_raw = action_el.get_attribute("data-args").unwrap_or_else(|| "{}".to_string());
    let args = js_sys::JSON::parse(&args_raw).unwrap_or(JsValue::NULL);

    run_action(&name, &args, &ActionCtx {
        event: event.clone(),
        element: action_el,
    });
}

/// Installs a delegated `click` listener on `document.body`.
pub fn listen_click_actions() {
    let cb = Closure::<dyn FnMut(Event)>::new(move |event: Event| {
        dispatch_action(&event);
    });
    if let Some(body) = document().body() {
        let _ = body.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
    }
    cb.forget();
}
