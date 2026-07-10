use hypertext::prelude::{GlobalAttributes, HtmxAttributes, hypertext_elements};
use hypertext::{Renderable, RenderableExt, rsx};
use wasm_bindgen::prelude::*;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{
    BORDER_END, CLUSTER, FLANK, GAP_XL, GAP_XS, GRID, HEADING_M, HEADING_S, ICON, PAGE_MENU_NAV, SIZE_SMALL, SPLIT,
    STACK, START,
};
use wingy_hypertext::components::button::Button;
use wingy_hypertext::layouts::page::{Page, PageBody, PageMenu};
use wingy_hypertext::link::LinkSetters;
use wingy_hypertext::link::Target::*;
use wingy_hypertext::variant::Variant::*;
use wingy_hypertext_web::components::callout::listen_close_callout;
use wingy_hypertext_web::{
    init_code_examples, init_page_element, init_scroll_to_anchor, listen_click_actions, listen_code_examples,
    register_copy_action,
};

pub mod components;
pub mod fontawesome;

/// One-time wiring: register actions and listeners.
#[wasm_bindgen]
pub fn init() {
    register_copy_action();
    init_code_examples();
    listen_code_examples();
    listen_click_actions();
    listen_close_callout();
}

/// Re-initialization run after every htmx settle.
#[wasm_bindgen]
pub fn reinit() {
    init_page_element();
    init_scroll_to_anchor();
}

#[wasm_bindgen]
pub fn request(route_path: &str) -> String {
    main_section(route_path).render().into_inner()
}

fn main_section(route_path: &str) -> impl Renderable {
    let parts: Vec<_> = route_path.split('#').collect();
    let path = parts.first().map(|path| path.trim_matches('/')).unwrap_or_default();

    rsx! {
        @match path {
            "badge" => (components::badge::overview()),
            "button" => (components::button::overview()),
            "callout" => (components::callout::overview()),
            "copy-button" => (components::copy_button::overview()),
            "input" => (components::input::overview()),
            _ => {},
        }
    }
}

#[wasm_bindgen]
pub fn render_root(url_path: &str) -> String {
    rsx! {
        <Page>
            <header class=SPLIT>
                <div class=CLUSTER>
                    <span class=ICON style="color: var(--wa-color-brand-fill-loud); font-size: 1.5em; --rotate-angle: 0deg;">
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class=(HEADING_M, " ", "wa-desktop-only")>"Wingy Hypertext"</span>
                    <a href="#">"Example Link"</a>
                </div>
                <div class=(CLUSTER, " ", GAP_XS)>
                    <Button variant=Brand appearance=Accent class=SIZE_SMALL href="https://github.com/XX/wingy-hypertext" target=Blank>
                        <span class=(START, " ", ICON)>
                            (fontawesome::icon("github"))
                        </span>
                        "GitHub"
                    </Button>
                </div>
            </header>
            <PageBody>
                <PageMenu>
                    <nav class=(PAGE_MENU_NAV, " ", BORDER_END)>
                        <div class=FLANK><span class=HEADING_M>"Components"</span></div>
                    </nav>
                    <nav class=(PAGE_MENU_NAV, " ", BORDER_END)>
                        <a
                            class=FLANK
                            href="/badge"
                            hx-get="/badge"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Badge"</span>
                        </a>
                        <a
                            class=FLANK
                            href="/button"
                            hx-get="/button"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Button"</span>
                        </a>
                        <a
                            class=FLANK
                            href="/callout"
                            hx-get="/callout"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Callout"</span>
                        </a>
                        <a
                            class=FLANK
                            href="/copy-button"
                            hx-get="/copy-button"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Copy Button"</span>
                        </a>
                        <a
                            class=FLANK
                            href="/input"
                            hx-get="/input"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Input"</span>
                        </a>
                    </nav>
                    <nav class=(PAGE_MENU_NAV, " ", BORDER_END)>
                        <div class=FLANK><span class=HEADING_M>"Layouts"</span></div>
                    </nav>
                    <nav class=PAGE_MENU_NAV>
                        <a class=FLANK href="#"><span>"Code Example"</span></a>
                        <a class=FLANK href="#"><span>"Page"</span></a>
                    </nav>
                </PageMenu>
                <main class="main-content">
                    (main_section(url_path))
                </main>
                <aside>
                </aside>
            </PageBody>
            <footer class=(GRID, " ", GAP_XL)>
                <div class=CLUSTER style="flex-wrap: nowrap">
                    <span class=ICON>
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class=HEADING_M>"Wingy Hypertext"</span>
                </div>
                <div class=STACK>
                    <h3 class=HEADING_S>Our Work</h3>
                    <a href="#">Habitat Restoration</a>
                    <a href="#">Migration Science</a>
                    <a href="#">Advocacy</a>
                </div>
                <div class=STACK>
                    <h3 class=HEADING_S>About Us</h3>
                    <a href="#">Our History</a>
                    <a href="#">Leadership</a>
                    <a href="#">Fiscal Reports</a>
                </div>
                <div class=STACK>
                    <h3 class=HEADING_S>Discover</h3>
                    <a href="#">Field Guides</a>
                    <a href="#">Photo Search</a>
                    <a href="#">Gear and Resources</a>
                </div>
                <div class=STACK>
                    <h3 class=HEADING_S>Get Involved</h3>
                    <a href="#">Adopt a Bird</a>
                    <a href="#">Your Local Audubon</a>
                    <a href="#">Youth Audubon Camps</a>
                </div>
            </footer>
        </Page>
    }
    .render()
    .into_inner()
}
