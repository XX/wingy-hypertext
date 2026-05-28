import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import init_htmx_request_interception from './vendor/htmx/client_patch.js';
import './js/components/copy_button.js';
import './js/components/head.js';
import './js/layouts/code_example.js';
import './js/layouts/page.js';
import './js/utils/animate.js';

await init();
init_htmx_request_interception(wasm);

let root_html = wasm.render_root(window.location.pathname);
let html_fragment = document.createRange().createContextualFragment(root_html);
let root = document.getElementById('root');
root.insertBefore(html_fragment, root.firstChild);

highlight.registerLanguage('html', html);
highlight.highlightAll();

htmx.process(root);
init_scroll_to_anchor();

document.body.addEventListener("htmx:afterSettle", function (_event) {
    highlight.highlightAll();
    init_scroll_to_anchor();
});
