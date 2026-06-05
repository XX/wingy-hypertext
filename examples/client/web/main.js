import init, * as wasm from "./dist/client.js";
import highlight from "./vendor/highlight/highlight.js";
import html from './vendor/highlight/languages/xml.js';
import init_htmx_request_interception from './vendor/htmx/client_patch.js';
import { register_copy_action } from './js/components/copy_button.js';
import { init_scroll_to_anchor } from './js/components/head.js';
import { init_code_examples, listen_code_examples } from './js/layouts/code_example.js';
import { init_page_element } from './js/layouts/page.js';
import { listen_click_actions } from './js/utils/action.js';
import './js/utils/animate.js';

await init();
init_htmx_request_interception(wasm);

let root_html = wasm.render_root(window.location.pathname);
let html_fragment = document.createRange().createContextualFragment(root_html);
let root = document.getElementById('root');
root.insertBefore(html_fragment, root.firstChild);

highlight.registerLanguage('html', html);

htmx.process(root);

reinit(root);
register_copy_action();
init_code_examples();
listen_code_examples();
listen_click_actions();

document.body.addEventListener("htmx:afterSettle", function (event) {
    reinit(event.target);
});

function reinit(root) {
    highlight.highlightAll();

    const page = root.querySelector('.page');
    init_page_element(page);
    init_scroll_to_anchor();
}
