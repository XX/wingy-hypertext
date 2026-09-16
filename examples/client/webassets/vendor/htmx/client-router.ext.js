/**
 * Answers htmx requests from a client-side router instead of the network.
 *
 * The interception happens at the transport: htmx builds its request, fires its
 * events and installs its response handler as usual, and only the trip to the
 * server is replaced. Everything downstream — `hx-push-url`, `hx-select`,
 * out-of-band swaps, `HX-*` response headers, indicators, the settle pipeline,
 * history snapshots, error handling — then runs exactly as it would for a real
 * response, because from htmx's point of view that is what it got.
 *
 * Cancelling `htmx:configRequest` and swapping by hand looks simpler, but htmx
 * returns from `issueAjaxRequest` the moment that event is cancelled, so every one
 * of those features is skipped along with the request.
 *
 * Restoring a history entry htmx no longer has cached is deliberately left alone:
 * that path swaps its response into the history element — the whole body — so the
 * fragments a router returns are the wrong shape for it. Set
 * `htmx.config.refreshOnHistoryMiss` if the app can serve its shell for the path.
 */

/** Makes a read-only XHR property report a value we chose. */
function override(xhr, name, value) {
    Object.defineProperty(xhr, name, { get: () => value, configurable: true });
}

/**
 * Drives an XHR to a completed 200 without touching the network, then hands it to
 * whichever handler htmx installed.
 */
function answer(xhr, path, body, deliver) {
    override(xhr, "readyState", 4);
    override(xhr, "status", 200);
    override(xhr, "statusText", "OK");
    override(xhr, "responseText", body);
    override(xhr, "response", body);
    // htmx reads this back to work out the path a redirect landed on, which is what
    // `hx-push-url` pushes when it is left to follow the response.
    override(xhr, "responseURL", new URL(path, location.href).href);
    xhr.getAllResponseHeaders = () => "";
    xhr.getResponseHeader = () => null;

    // A real response never arrives during send(), and htmx has work left to do
    // after it: deliver on a later task, as the network would.
    setTimeout(deliver, 0);
}

export function define_client_router_ext(htmx, server_api, router, preproc) {
    htmx.defineExtension("client-router", {
        onEvent: function (name, event) {
            if (router && name === "htmx:beforeSend") {
                const detail = event.detail;
                const path = detail.pathInfo.finalRequestPath;

                if (path.startsWith(server_api)) return;

                const body = router(path, event);
                if (body === undefined || body === null) return;

                const xhr = detail.xhr;
                // htmx calls send() as soon as this event returns; neutralise it for
                // this request only, leaving the prototype alone.
                xhr.send = () => {};
                xhr.abort = () => {};

                answer(xhr, path, body, () => xhr.onload());
            } else if (preproc && name === "htmx:beforeSwap") {
                event.detail.serverResponse = preproc(event.detail.serverResponse, event);
            }
        },
    });
}

export { define_client_router_ext as default };
