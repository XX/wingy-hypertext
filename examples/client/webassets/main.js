import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import rust from './vendor/highlight/languages/rust.js';
import define_client_router_ext from './vendor/htmx/client-router.ext.js';

await init();

function router(path, event) {
    return wasm.request(path);
}
define_client_router_ext(htmx, "/api", router);

let root_html = wasm.render_root(window.location.pathname);
let html_fragment = document.createRange().createContextualFragment(root_html);
let root = document.getElementById('root');
root.insertBefore(html_fragment, root.firstChild);

highlight.registerLanguage('html', html);
highlight.registerLanguage('rust', rust);

htmx.process(root);

reinit(root);
wasm.init();

document.body.addEventListener("htmx:afterSettle", function (event) {
    reinit(event.target);
});

function reinit(_root) {
    highlight.highlightAll();
    wasm.reinit();
}
