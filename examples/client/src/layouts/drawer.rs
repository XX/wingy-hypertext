use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{DefaultBuilder, Renderable, renderable, rsx};
use iconic::fontawesome;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Element, Event};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::ICON;
use wingy_hypertext::components::button::Button;
use wingy_hypertext::components::head::Head;
use wingy_hypertext::components::head::HeadLevel::*;
use wingy_hypertext::components::input::Input;
use wingy_hypertext::layouts::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layouts::drawer::DrawerPlacement::*;
use wingy_hypertext::layouts::drawer::{Drawer, DrawerBody, DrawerFooter, DrawerHeader};
use wingy_hypertext::variant::Variant::*;

#[renderable(builder = DefaultBuilder)]
#[derive(Default)]
fn open_button<'a>(id: &'a str) -> impl Renderable {
    let data_drawer = format!("open {id}");
    rsx! {
        <Button appearance=Filled attrs=(attrs!["data-drawer" = &data_drawer])>
            "Open Drawer"
        </Button>
    }
}

#[renderable]
fn close_button() -> impl Renderable {
    rsx! {
        <Button variant=Brand attrs=(attrs!["data-drawer" = &"close"])>
            "Close"
        </Button>
    }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Drawer"</Head>
        <p>"Drawers slide in from the edge of the screen to expose additional options and information without "
            "navigating away. They are useful for navigation menus, filters, and secondary content. The drawer is "
            "rendered as a native "<code>"<dialog>"</code>" element; its open/close behavior is implemented in Rust "
            "in "<code>"wingy-hypertext-web"</code>" and wired up with "<code>"listen_drawers"</code>" and "
            <code>"init_drawers"</code>". Open any drawer declaratively with a "<code>r#"data-drawer="open <id>""#</code>
            " trigger, and close it from within with "<code>r#"data-drawer="close""#</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-overview">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "Drawers are great for showing additional content without leaving the current page."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-overview"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-overview">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "Drawers are great for showing additional content without leaving the current page."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>

                    // The open button carries `data-drawer="open drawer-overview"`
                    <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open drawer-overview"])>
                        "Open Drawer"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="without-a-header" anchor=true>
            "Without a Header"
        </Head>
        <p>"Headers are enabled by default. To render a drawer without a header, add the "
            <code>"without_header"</code>" attribute."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-without-header">
                    <DrawerBody>
                        "Look ma, no header!"
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-without-header"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-without-header">
                        <DrawerBody>
                            "Look ma, no header!"
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="footer" anchor=true>
            "Footer"
        </Head>
        <p>"Footers can be used to display titles and more. Use the "<code>"footer"</code>
            " property to add a footer to the drawer."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-footer">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer has a footer where you can put actions and other controls."
                    </DrawerBody>
                    <DrawerFooter>
                        <p>"Footer"</p>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-footer"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-footer">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer has a footer where you can put actions and other controls."
                        </DrawerBody>
                        <DrawerFooter>
                            <p>"Footer"</p>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="opening-closing-declaratively" anchor=true>
            "Opening & Closing Declaratively"
        </Head>
        <p>"Add "<code>r#"data-drawer="open <id>""#</code>" to any button on the page, where "<code>"<id>"</code>
            " is the id of the drawer you want to open. Similarly, add "<code>r#"data-drawer="close""#</code>
            " to a button "<em>"inside"</em>" of a drawer to tell it to close — no JavaScript required."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-opening">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer was opened declaratively using a data attribute on the button."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-opening"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-opening">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer was opened declaratively using a data attribute on the button."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>

                    <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open drawer-overview"])>
                        "Open Drawer"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placement" anchor=true>
            "Placement"
        </Head>
        <p>"Drawers slide in from the start by default. Set the "<code>"placement"</code>
            " attribute to "<code>"Start"</code>", "<code>"End"</code>", "<code>"Top"</code>", or "
            <code>"Bottom"</code>" to slide in from a different edge."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-placement-end" placement=End>
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer slides in from the end."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open drawer-placement-end"])>
                    "Open from End"
                </Button>

                <Drawer id="drawer-placement-bottom" placement=Bottom>
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer slides in from the bottom."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open drawer-placement-bottom"])>
                    "Open from Bottom"
                </Button>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-placement-end" placement=End>
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer slides in from the end."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>

                    <Drawer id="drawer-placement-bottom" placement=Bottom>
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer slides in from the bottom."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Use the "<code>"--size"</code>" custom property to set the drawer's size. This will be applied to the "
            "drawer's width or height depending on its placement."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-custom-size" style="--size: 50vw;">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer is always 50% of the viewport."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-custom-size"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-custom-size" style="--size: 50vw;">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer is always 50% of the viewport."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="scrolling" anchor=true>
            "Scrolling"
        </Head>
        <p>"By design, a drawer's height will never exceed 100% of its container. As such, drawers will not scroll "
            "with the page to ensure the header and footer are always accessible to the user."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-scrolling">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        <div style="height: 150vh; border: dashed 2px var(--wa-color-surface-border); padding: 0 1rem;">
                            <p>"Scroll down and give it a try! 👇"</p>
                        </div>
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-scrolling"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-scrolling">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            <div style="height: 150vh; ...">
                                <p>"Scroll down and give it a try! 👇"</p>
                            </div>
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="header-actions" anchor=true>
            "Header Actions"
        </Head>
        <p>"The header shows a functional close button by default. Use the "<code>"header_actions"</code>
            " property to add additional buttons if needed."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-header-actions" class="drawer-header-actions-demo">
                    <DrawerHeader actions=(rsx! {
                        <Button class="new-window" appearance=Plain>
                            <span class=ICON>
                                (fontawesome::solid::Gear)
                            </span>
                        </Button>
                    })>
                        "Drawer"
                    </DrawerHeader>
                    <DrawerBody>
                        "You can add custom actions to the header, like the button up there to open in a new window."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-header-actions"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-header-actions" class="drawer-header-actions-demo">
                        <DrawerHeader actions=(rsx! {
                            <Button class="new-window" appearance=Plain>
                                <span class=ICON>
                                    (fontawesome::solid::Gear)
                                </span>
                            </Button>
                        })>
                            "Drawer"
                        </DrawerHeader>
                        <DrawerBody>
                            "You can add custom actions to the header, like the button up there to open in a new window."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="light-dismissal" anchor=true>
            "Light Dismissal"
        </Head>
        <p>"If you want the drawer to close when the user clicks on the overlay, add the "
            <code>"light_dismiss"</code>" attribute."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-light-dismiss" light_dismiss=true>
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer will close when you click on the overlay."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-light-dismiss"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-light-dismiss" light_dismiss=true>
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer will close when you click on the overlay."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="preventing-closing" anchor=true>
            "Preventing the Drawer from Closing"
        </Head>
        <p>"By default, drawers close when the user clicks the close button, clicks the overlay, or presses "
            <code>"Escape"</code>". To keep the drawer open in cases where closing would be destructive, cancel "
            "the "<code>"wg-hide"</code>" event. When canceled, the drawer stays open and pulses briefly. Inspect "
            <code>"event.detail.source"</code>" to determine what triggered the request to close — this demo only "
            "allows the footer close button to dismiss the drawer."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-deny-close" class="drawer-deny-close">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        "This drawer will only close when you click the button below."
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-deny-close"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-deny-close" class="drawer-deny-close">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            "This drawer will only close when you click the button below."
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>

                    // Prevent closing unless the close button is the source
                    document.add_steady_event_listener("wg-hide", |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="initial-focus" anchor=true>
            "Initial Focus"
        </Head>
        <p>"To give focus to a specific element when the drawer opens, use the "<code>"autofocus"</code>
            " attribute on that element."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="drawer-focus">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        <Input autofocus=true placeholder="I will have focus when the drawer is opened" />
                    </DrawerBody>
                    <DrawerFooter>
                        <CloseButton/>
                    </DrawerFooter>
                </Drawer>
                <OpenButton id="drawer-focus"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="drawer-focus">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            <Input autofocus=true placeholder="I will have focus when the drawer is opened" />
                        </DrawerBody>
                        <DrawerFooter>
                            <CloseButton/>
                        </DrawerFooter>
                    </Drawer>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring for the two demos that need scripted behavior: preventing
/// the deny-close drawer from closing and opening a new window from the header
/// actions button.
pub fn listen_drawer_overview() {
    let document = dom::existing::document();

    // Prevent `.drawer-deny-close` from closing unless the footer close button
    // was the source of the request.
    document.add_steady_event_listener("wg-hide", |event| {
        prevent_deny_close(&event);
    });

    // The header-actions "new window" button opens the current page in a new tab.
    document.add_steady_event_listener("click", |event| {
        handle_new_window(&event);
    });
}

fn prevent_deny_close(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    target.closest(".drawer-deny-close").ok()??;

    let custom: &CustomEvent = event.dyn_ref()?;
    let source = custom.detail().get("source");
    let is_close_button = source
        .dyn_into::<Element>()
        .ok()
        .and_then(|element| element.closest("[data-drawer='close']").ok().flatten())
        .is_some();

    if !is_close_button {
        event.prevent_default();
    }

    Some(())
}

fn handle_new_window(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    target.closest(".new-window").ok()??;

    let window = dom::existing::window();
    let href = window.location().href().ok()?;
    window.open_with_url(&href).ok();

    Some(())
}
