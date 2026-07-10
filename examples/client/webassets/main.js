import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import rust from './vendor/highlight/languages/rust.js';
import init_htmx_request_interception from './vendor/htmx/client_patch.js';

await init();
init_htmx_request_interception(wasm);

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
