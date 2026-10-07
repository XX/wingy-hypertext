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
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::input::Input;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::dialog::{
    Dialog, DialogBody, DialogFooter, DialogHeader, DialogHeaderActions, DialogTitle,
};
use wingy_hypertext::layout::drawer::{Drawer, DrawerBody, DrawerHeader};
use wingy_hypertext::variant::Variant::*;

#[renderable(builder = DefaultBuilder)]
#[derive(Default)]
fn open_button<'a>(id: &'a str) -> impl Renderable {
    let data_dialog = format!("open {id}");
    rsx! {
        <Button appearance=Filled attrs=(attrs!["data-dialog" = &data_dialog])>
            "Open Dialog"
        </Button>
    }
}

#[renderable]
fn close_button() -> impl Renderable {
    rsx! {
        <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
            "Close"
        </Button>
    }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Dialog"</Head>
        <p>"Dialogs, sometimes called “modals”, appear above the page and require the user's immediate attention. "
            "Use them for confirmations, forms, or focused tasks that interrupt the main flow. The dialog is "
            "rendered as a native "<code>"<dialog>"</code>" element and composed of "<code>"DialogHeader"</code>", "
            <code>"DialogBody"</code>" and "<code>"DialogFooter"</code>"; its open/close behavior is implemented in "
            "Rust in "<code>"wingy-hypertext-web"</code>" and wired up with "<code>"listen_dialogs"</code>" and "
            <code>"init_dialogs"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-overview">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This is a standard dialog. You can put any content you want in here!"
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-overview"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-overview">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This is a standard dialog. You can put any content you want in here!"
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>

                    // The open button carries `data-dialog="open dialog-overview"`
                    <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-overview"])>
                        "Open Dialog"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>
        <Head level=H3 id="without-a-header" anchor=true>
            "Without a Header"
        </Head>
        <p>"To render a dialog without a header, leave out "<code>"DialogHeader"</code>". Without a visible "
            "title, label the dialog with an "<code>"aria-label"</code>" attribute."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-without-header" attrs=(attrs!["aria-label" = &"Dialog"])>
                    <DialogBody>
                        "Look ma, no header! Sometimes you just need a clean, simple dialog."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-without-header"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-without-header" attrs=(attrs!["aria-label" = &"Dialog"])>
                        <DialogBody>
                            "Look ma, no header! Sometimes you just need a clean, simple dialog."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="footer" anchor=true>
            "Footer"
        </Head>
        <p>"Footers can be used to display titles and more. Put "<code>"DialogFooter"</code>
            " after the body to add a footer to the dialog."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-footer">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "Check out the footer below — it's a great place for actions and buttons."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-footer"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-footer">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "Check out the footer below — it's a great place for actions and buttons."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="opening-closing-declaratively" anchor=true>
            "Opening & Closing Declaratively"
        </Head>
        <p>"Add "<code>r#"data-dialog="open <id>""#</code>" to any button on the page, where "<code>"<id>"</code>
            " is the id of the dialog you want to open."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-opening">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This dialog was opened declaratively — no JavaScript required!"
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-opening"])>
                    "Open Dialog"
                </Button>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-opening">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This dialog was opened declaratively — no JavaScript required!"
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>

                    <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-opening"])>
                        "Open Dialog"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"Similarly, you can add "<code>r#"data-dialog="close""#</code>" to a button "<em>"inside"</em>
            " of a dialog to tell it to close."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-dismiss">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "Click the button in the footer to close this dialog declaratively."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-dismiss"])>
                    "Open Dialog"
                </Button>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-dismiss">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "Click the button in the footer to close this dialog declaratively."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>

                    <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-dismiss"])>
                        "Open Dialog"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="width" anchor=true>
            "Width"
        </Head>
        <p>"Use the "<code>"--width"</code>" custom property to set the dialog's width."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-width" style="--width: 50vw;">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This dialog is wider than the default — handy when you need more room for content."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-width"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-width" style="--width: 50vw;">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This dialog is wider than the default — handy when you need more room for content."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="scrolling" anchor=true>
            "Scrolling"
        </Head>
        <p>"By design, a dialog's height will never exceed that of the viewport. As such, dialogs will not scroll "
            "with the page ensuring the header and footer are always accessible to the user."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-scrolling">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        <div style="height: 150vh; border: dashed 2px var(--wa-color-surface-border); padding: 0 1rem;">
                            <p>"Scroll down and give it a try! 👇"</p>
                        </div>
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-scrolling"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-scrolling">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            <div style="height: 150vh; ...">
                                <p>"Scroll down and give it a try! 👇"</p>
                            </div>
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="header-actions" anchor=true>
            "Header Actions"
        </Head>
        <p>"The header shows a functional close button by default. To add more buttons next to it, make the "
            "header "<code>"bare"</code>" and compose it from "<code>"DialogTitle"</code>" and "
            <code>"DialogHeaderActions"</code>": the actions are followed by the close button."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-header-actions">
                    <DialogHeader bare=true>
                        <DialogTitle>"Dialog"</DialogTitle>
                        <DialogHeaderActions>
                            <Button class="new-window" appearance=Plain attrs=(attrs!["aria-label" = &"Open in new window"])>
                                <span class=ICON>
                                    (fontawesome::solid::Gear)
                                </span>
                            </Button>
                        </DialogHeaderActions>
                    </DialogHeader>
                    <DialogBody>
                        "You can add custom actions to the header, like the icon button up there!"
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-header-actions"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-header-actions">
                        <DialogHeader bare=true>
                            <DialogTitle>"Dialog"</DialogTitle>
                            <DialogHeaderActions>
                                <Button class="new-window" appearance=Plain attrs=(attrs!["aria-label" = &"Open in new window"])>
                                    <span class=ICON>
                                        (fontawesome::solid::Gear)
                                    </span>
                                </Button>
                            </DialogHeaderActions>
                        </DialogHeader>
                        <DialogBody>
                            "You can add custom actions to the header, like the icon button up there!"
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="light-dismissal" anchor=true>
            "Light Dismissal"
        </Head>
        <p>"If you want the dialog to close when the user clicks on the overlay, add the "
            <code>"light_dismiss"</code>" attribute."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-light-dismiss" light_dismiss=true>
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This dialog will close when you click on the overlay."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-light-dismiss"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-light-dismiss" light_dismiss=true>
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This dialog will close when you click on the overlay."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="preventing-closing" anchor=true>
            "Preventing the Dialog from Closing"
        </Head>
        <p>"By default, dialogs close when the user clicks the close button or presses "<code>"Escape"</code>
            ". To keep the dialog open in cases where closing would be destructive, cancel the "<code>"wg-hide"</code>
            " event. When canceled, the dialog stays open and pulses briefly. Inspect "
            <code>"event.detail.source"</code>" to determine what triggered the request to close — this demo only "
            "allows the footer button to dismiss the dialog."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-deny-close" class="dialog-deny-close">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This dialog will only close when you click the button below."
                    </DialogBody>
                    <DialogFooter>
                        <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                            "Only this button will close it"
                        </Button>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-deny-close"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-deny-close" class="dialog-deny-close">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This dialog will only close when you click the button below."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Only this button will close it"
                            </Button>
                        </DialogFooter>
                    </Dialog>

                    // Prevent closing unless the footer button is the source
                    document.add_steady_event_listener("wg-hide", |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="initial-focus" anchor=true>
            "Initial Focus"
        </Head>
        <p>"To give focus to a specific element when the dialog opens, use the "<code>"autofocus"</code>
            " attribute on that element."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dialog id="dialog-focus">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        <Input autofocus=true placeholder="I will have focus when the dialog is opened" />
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <OpenButton id="dialog-focus"/>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dialog id="dialog-focus">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            <Input autofocus=true placeholder="I will have focus when the dialog is opened" />
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="nested" anchor=true>
            "Dialog from a Drawer"
        </Head>
        <p>"Modals can be stacked: a dialog opened from a drawer appears above it. "<code>"Escape"</code>
            " closes the top-most one only, and the page stays scroll locked until the last one closes."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Drawer id="dialog-nested-drawer">
                    <DrawerHeader>"Drawer"</DrawerHeader>
                    <DrawerBody>
                        <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-nested"])>
                            "Open Dialog"
                        </Button>
                    </DrawerBody>
                </Drawer>
                <Dialog id="dialog-nested">
                    <DialogHeader>"Dialog"</DialogHeader>
                    <DialogBody>
                        "This dialog is opened above the drawer."
                    </DialogBody>
                    <DialogFooter>
                        <CloseButton/>
                    </DialogFooter>
                </Dialog>
                <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open dialog-nested-drawer"])>
                    "Open Drawer"
                </Button>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Drawer id="dialog-nested-drawer">
                        <DrawerHeader>"Drawer"</DrawerHeader>
                        <DrawerBody>
                            <Button appearance=Filled attrs=(attrs!["data-dialog" = &"open dialog-nested"])>
                                "Open Dialog"
                            </Button>
                        </DrawerBody>
                    </Drawer>
                    <Dialog id="dialog-nested">
                        <DialogHeader>"Dialog"</DialogHeader>
                        <DialogBody>
                            "This dialog is opened above the drawer."
                        </DialogBody>
                        <DialogFooter>
                            <Button variant=Brand attrs=(attrs!["data-dialog" = &"close"])>
                                "Close"
                            </Button>
                        </DialogFooter>
                    </Dialog>

                    <Button appearance=Filled attrs=(attrs!["data-drawer" = &"open dialog-nested-drawer"])>
                        "Open Drawer"
                    </Button>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring for the demos that need scripted behavior: preventing the
/// deny-close dialog from closing and opening a new window from the header
/// actions button.
pub fn listen_dialog_overview() {
    let document = dom::existing::document();

    // Prevent `.dialog-deny-close` from closing unless the footer button was
    // the source of the request.
    document.add_steady_event_listener("wg-hide", |event| {
        prevent_deny_close(&event);
    });
}

fn prevent_deny_close(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    if !target.class_list().contains("dialog-deny-close") {
        return None;
    }

    let custom: &CustomEvent = event.dyn_ref()?;
    let source = custom.detail().get("source");
    let is_footer_button = source
        .dyn_into::<Element>()
        .ok()
        .and_then(|element| element.closest(".dialog-footer").ok().flatten())
        .is_some();

    if !is_footer_button {
        event.prevent_default();
    }

    Some(())
}
