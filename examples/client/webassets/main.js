import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import init_htmx_request_interception from './vendor/htmx/client_patch.js';

await init();
init_htmx_request_interception(wasm);

let root_html = wasm.render_root(window.location.pathname);
let html_fragment = document.createRange().createContextualFragment(root_html);
let root = document.getElementById('root');
root.insertBefore(html_fragment, root.firstChild);

highlight.registerLanguage('html', html);

htmx.process(root);

reinit(root);
wasm.register_copy_action();
wasm.init_code_examples();
wasm.listen_code_examples();
wasm.listen_click_actions();

document.body.addEventListener("htmx:afterSettle", function (event) {
    reinit(event.target);
});

function reinit(root) {
    highlight.highlightAll();

    // Page metrics and anchor scrolling are handled by the wingy-hypertext-web WASM module.
    wasm.init_page_element();
    wasm.init_scroll_to_anchor();
}
