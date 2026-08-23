export function define_client_router_ext(htmx, server_api, router, preproc) {
    htmx.defineExtension('client-router', {
        onEvent: function (name, event) {
            if (router && name === "htmx:configRequest") {
                let path = event.detail.path;

                if (!path.startsWith(server_api)) {
                    event.preventDefault(); 

                    let client_html = router(path, event);

                    let target = event.detail.target;
                    let swap_spec = event.detail.elt.getAttribute('hx-swap') || 'innerHTML';
                    
                    htmx.swap(target, client_html, { swapStyle: swap_spec });
                }
            } else if (preproc && name === "htmx:beforeSwap") {
                event.detail.serverResponse = preproc(event.detail.serverResponse, event);
            }
        }
    });
}

export { define_client_router_ext as default };
