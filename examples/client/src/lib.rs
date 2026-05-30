use hypertext::prelude::{GlobalAttributes, HtmxAttributes, hypertext_elements};
use hypertext::{Renderable, RenderableExt, rsx};
use wasm_bindgen::prelude::*;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::components::button::Button;
use wingy_hypertext::layouts::page::{Page, PageAside, PageBody, PageFooter, PageHeader, PageMain, PageMenu};
use wingy_hypertext::link::LinkSetters;
use wingy_hypertext::link::Target::*;
use wingy_hypertext::variant::Variant::*;

pub mod components;
pub mod fontawesome;

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
            "copy-button" => (components::copy_button::overview()),
            _ => {},
        }
    }
}

#[wasm_bindgen]
pub fn render_root(url_path: &str) -> String {
    rsx! {
        <Page>
            <PageHeader class="wa-split">
                <div class="wa-cluster">
                    <span class="icon" style="color: var(--wa-color-brand-fill-loud); font-size: 1.5em; --rotate-angle: 0deg;">
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class="wa-heading-m wa-desktop-only">"Wingy Hypertext"</span>
                    <a href="#">"Example Link"</a>
                </div>
                <div class="wa-cluster wa-gap-xs">
                    <Button variant=Brand appearance=Accent class="size-small" href="https://github.com/XX/wingy-hypertext" target=Blank>
                        <span class="start icon">
                            (fontawesome::icon("github"))
                        </span>
                        "GitHub"
                    </Button>
                </div>
            </PageHeader>
            <PageBody>
                <PageMenu>
                    <nav class="page-menu-nav border-end">
                        <div class="wa-flank"><span class="wa-heading-m">"Components"</span></div>
                    </nav>
                    <nav class="page-menu-nav border-end">
                        <a
                            class="wa-flank"
                            href="/badge"
                            hx-get="/badge"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Badge"</span>
                        </a>
                        <a
                            class="wa-flank"
                            href="/button"
                            hx-get="/button"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Button"</span>
                        </a>
                        <a
                            class="wa-flank"
                            href="/copy-button"
                            hx-get="/copy-button"
                            hx-target=".main-content"
                            hx-swap="innerHTML"
                            hx-push-url="true"
                        >
                            <span>"Copy Button"</span>
                        </a>
                    </nav>
                    <nav class="page-menu-nav border-end">
                        <div class="wa-flank"><span class="wa-heading-m">"Layouts"</span></div>
                    </nav>
                    <nav class="page-menu-nav">
                        <a class="wa-flank" href="#"><span>"Code Example"</span></a>
                        <a class="wa-flank" href="#"><span>"Page"</span></a>
                    </nav>
                </PageMenu>
                <PageMain class="main-content">
                    (main_section(url_path))
                </PageMain>
                <PageAside>
                </PageAside>
            </PageBody>
            <PageFooter class="wa-grid wa-gap-xl">
                <div class="wa-cluster" style="flex-wrap: nowrap">
                    <span class="icon">
                        (fontawesome::icon("puzzle-piece"))
                    </span>
                    <span id="brand-name" class="wa-heading-m">"Wingy Hypertext"</span>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Our Work</h3>
                    <a href="#">Habitat Restoration</a>
                    <a href="#">Migration Science</a>
                    <a href="#">Advocacy</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">About Us</h3>
                    <a href="#">Our History</a>
                    <a href="#">Leadership</a>
                    <a href="#">Fiscal Reports</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Discover</h3>
                    <a href="#">Field Guides</a>
                    <a href="#">Photo Search</a>
                    <a href="#">Gear and Resources</a>
                </div>
                <div class="wa-stack">
                    <h3 class="wa-heading-s">Get Involved</h3>
                    <a href="#">Adopt a Bird</a>
                    <a href="#">Your Local Audubon</a>
                    <a href="#">Youth Audubon Camps</a>
                </div>
            </PageFooter>
        </Page>
    }
    .render()
    .into_inner()
}
