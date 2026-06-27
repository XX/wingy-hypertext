const actions = new Map();

export function register_action(name, fn) {
    actions.set(name, fn);
}

export function unregister_action(name) {
    actions.delete(name);
}

export function run_action(name, args, ctx) {
    actions.get(name)?.(args, ctx);
}

export function dispatch_action(event) {
    const element = event.target.closest("[data-action]");
    if (!element) return;

    run_action(
        element.dataset.action,
        JSON.parse(element.dataset.args) || {},
        {
            event,
            element
        }
    );
}

export function listen_click_actions() {
    document.body.addEventListener("click", (event) => {
        dispatch_action(event);
    });
}

export const Action = {
    register: register_action,
    unregister: unregister_action,
    run: run_action,
    dispatch: dispatch_action
};

window.WingyAction = Action;
