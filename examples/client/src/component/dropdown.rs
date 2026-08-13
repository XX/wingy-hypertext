use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{DefaultBuilder, Renderable, renderable, rsx};
use iconic::{fontawesome, fontawesome_ext};
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Element, Event};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{END, ICON, SIZE_SMALL};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::dropdown::DropdownSize::*;
use wingy_hypertext::component::dropdown::{Dropdown, DropdownItem};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::helper::popup::PopupPlacement::RightStart;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext::variant::Variant::*;
use wingy_hypertext_web::util::event;

/// The dropdown trigger of the examples: a button with a caret, like Web
/// Awesome's `with-caret` buttons.
#[renderable(builder = DefaultBuilder)]
#[derive(Default)]
fn trigger_button<'a>(label: &'a str) -> impl Renderable {
    rsx! {
        <Button appearance=Filled>
            (label)
            <span class=(END, " ", ICON)>
                (fontawesome_ext::regular::ChevronDown)
            </span>
        </Button>
    }
}

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Dropdown"</Head>
        <p>"Dropdowns display a list of options next to a trigger element. They support keyboard navigation, "
            "submenus and checkable items, so they can serve as menus and context actions. The menu is positioned "
            "with the "<code>Popup</code>" helper, and the behavior (opening, selection, submenus, "
            "type-to-select) is implemented in Rust in "<code>"wingy-hypertext-web"</code>" and wired up with "
            <code>"listen_dropdowns"</code>" and "<code>"init_dropdowns"</code>"."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Options"/> })>
                    <DropdownItem value="edit">"Edit"</DropdownItem>
                    <DropdownItem value="duplicate">"Duplicate"</DropdownItem>
                    <DropdownItem value="delete">"Delete"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <Button appearance=Filled>"Options"</Button> })>
                        <DropdownItem value="edit">"Edit"</DropdownItem>
                        <DropdownItem value="duplicate">"Duplicate"</DropdownItem>
                        <DropdownItem value="delete">"Delete"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="showing-icons" anchor=true>
            "Showing Icons"
        </Head>
        <p>"Use the "<code>icon</code>" property to add an icon before an item's label."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Edit"/> })>
                    <DropdownItem value="copy" icon=(rsx! { (fontawesome::solid::Copy) })>
                        "Copy"
                    </DropdownItem>
                    <DropdownItem value="rename" icon=(rsx! { (fontawesome::solid::PenToSquare) })>
                        "Rename"
                    </DropdownItem>
                    <DropdownItem value="bookmark" icon=(rsx! { (fontawesome::solid::Bookmark) })>
                        "Bookmark"
                    </DropdownItem>
                    <DropdownItem value="delete" variant=Danger icon=(rsx! { (fontawesome::solid::XmarkCircle) })>
                        "Delete"
                    </DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="Edit"/> })>
                        <DropdownItem value="copy" icon=(rsx! { (fontawesome::solid::Copy) })>
                            "Copy"
                        </DropdownItem>
                        <DropdownItem value="rename" icon=(rsx! { (fontawesome::solid::PenToSquare) })>
                            "Rename"
                        </DropdownItem>
                        <DropdownItem value="bookmark" icon=(rsx! { (fontawesome::solid::Bookmark) })>
                            "Bookmark"
                        </DropdownItem>
                        <DropdownItem value="delete" variant=Danger icon=(rsx! { (fontawesome::solid::XmarkCircle) })>
                            "Delete"
                        </DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="labels-and-dividers" anchor=true>
            "Showing Labels & Dividers"
        </Head>
        <p>"Use any heading to label a group of items, and a "<code>Divider</code>" to separate them."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Device"/> })>
                    <h3>"Type"</h3>
                    <DropdownItem value="phone">"Phone"</DropdownItem>
                    <DropdownItem value="tablet">"Tablet"</DropdownItem>
                    <DropdownItem value="desktop">"Desktop"</DropdownItem>
                    <Divider/>
                    <DropdownItem value="more">"More options…"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="Device"/> })>
                        <h3>"Type"</h3>
                        <DropdownItem value="phone">"Phone"</DropdownItem>
                        <DropdownItem value="tablet">"Tablet"</DropdownItem>
                        <DropdownItem value="desktop">"Desktop"</DropdownItem>
                        <Divider/>
                        <DropdownItem value="more">"More options…"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="showing-details" anchor=true>
            "Showing Details"
        </Head>
        <p>"Use the "<code>details</code>" property to show secondary content after the label, "
            "such as a keyboard shortcut."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Message"/> })>
                    <DropdownItem value="reply" details=(rsx! { "⌘R" })>"Reply"</DropdownItem>
                    <DropdownItem value="forward" details=(rsx! { "⌘F" })>"Forward"</DropdownItem>
                    <DropdownItem value="move" details=(rsx! { "⌘M" })>"Move"</DropdownItem>
                    <Divider/>
                    <DropdownItem value="archive" details=(rsx! { "⌘A" })>"Archive"</DropdownItem>
                    <DropdownItem value="delete" variant=Danger details=(rsx! { "Del" })>"Delete"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="Message"/> })>
                        <DropdownItem value="reply" details=(rsx! { "⌘R" })>"Reply"</DropdownItem>
                        <DropdownItem value="forward" details=(rsx! { "⌘F" })>"Forward"</DropdownItem>
                        <DropdownItem value="move" details=(rsx! { "⌘M" })>"Move"</DropdownItem>
                        <Divider/>
                        <DropdownItem value="archive" details=(rsx! { "⌘A" })>"Archive"</DropdownItem>
                        <DropdownItem value="delete" variant=Danger details=(rsx! { "Del" })>"Delete"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="checkable-items" anchor=true>
            "Checkable Items"
        </Head>
        <p>"Set the "<code>checkbox</code>" property to turn an item into a toggle, and add "<code>checked</code>
            " to start it on. Selecting a checkable item flips its state and closes the dropdown; cancel the "
            <code>"wg-select"</code>" event to keep it open instead — which is what this demo does."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="dropdown-checkable-demo">
                    <Dropdown trigger=(rsx! { <TriggerButton label="View"/> })>
                        <DropdownItem checkbox=true checked=true value="canvas">"Show canvas"</DropdownItem>
                        <DropdownItem checkbox=true checked=true value="grid">"Show grid"</DropdownItem>
                        <DropdownItem checkbox=true value="source">"Show source"</DropdownItem>
                        <Divider/>
                        <DropdownItem value="preferences">"Preferences…"</DropdownItem>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="View"/> })>
                        <DropdownItem checkbox=true checked=true value="canvas">"Show canvas"</DropdownItem>
                        <DropdownItem checkbox=true checked=true value="grid">"Show grid"</DropdownItem>
                        <DropdownItem checkbox=true value="source">"Show source"</DropdownItem>
                        <Divider/>
                        <DropdownItem value="preferences">"Preferences…"</DropdownItem>
                    </Dropdown>

                    // Keep the menu open while toggling checkable items
                    document.add_steady_event_listener("wg-select", |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="destructive-items" anchor=true>
            "Destructive Items"
        </Head>
        <p>"Set "<code>"variant=Danger"</code>" on an item to flag a destructive action like deleting."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Project"/> })>
                    <DropdownItem value="share" icon=(rsx! { (fontawesome::solid::Copy) })>"Share"</DropdownItem>
                    <DropdownItem value="preferences" icon=(rsx! { (fontawesome::solid::Gear) })>
                        "Preferences"
                    </DropdownItem>
                    <Divider/>
                    <h3>"Danger zone"</h3>
                    <DropdownItem value="archive" icon=(rsx! { (fontawesome::solid::Bookmark) })>
                        "Archive"
                    </DropdownItem>
                    <DropdownItem value="delete" variant=Danger icon=(rsx! { (fontawesome::solid::XmarkCircle) })>
                        "Delete"
                    </DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="Project"/> })>
                        <DropdownItem value="share" icon=(rsx! { (fontawesome::solid::Copy) })>"Share"</DropdownItem>
                        <DropdownItem value="preferences" icon=(rsx! { (fontawesome::solid::Gear) })>
                            "Preferences"
                        </DropdownItem>
                        <Divider/>
                        <h3>"Danger zone"</h3>
                        <DropdownItem value="archive" icon=(rsx! { (fontawesome::solid::Bookmark) })>
                            "Archive"
                        </DropdownItem>
                        <DropdownItem value="delete" variant=Danger icon=(rsx! { (fontawesome::solid::XmarkCircle) })>
                            "Delete"
                        </DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="submenus" anchor=true>
            "Submenus"
        </Head>
        <p>"To nest a menu, pass the nested items to an item's "<code>submenu</code>" property. A submenu opens on "
            "hover, on "<code>"→"</code>" and on activation; "<code>"←"</code>" and "<code>Escape</code>
            " collapse it again. An item that opens a submenu doesn't emit "<code>"wg-select"</code>" itself."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="dropdown-select-demo">
                    <Dropdown trigger=(rsx! { <TriggerButton label="File"/> })>
                        <DropdownItem value="new">"New"</DropdownItem>
                        <DropdownItem value="open">"Open"</DropdownItem>
                        <Divider/>
                        <DropdownItem submenu=(rsx! {
                            <DropdownItem value="pdf">"PDF"</DropdownItem>
                            <DropdownItem value="docx">"Word document"</DropdownItem>
                            <DropdownItem value="xlsx">"Excel spreadsheet"</DropdownItem>
                            <DropdownItem value="csv">"CSV"</DropdownItem>
                        })>
                            "Export"
                        </DropdownItem>
                        <DropdownItem submenu=(rsx! {
                            <DropdownItem checkbox=true value="compress">"Compress files"</DropdownItem>
                            <DropdownItem checkbox=true checked=true value="metadata">"Include metadata"</DropdownItem>
                            <DropdownItem checkbox=true value="password">"Password protect"</DropdownItem>
                        })>
                            "Options"
                        </DropdownItem>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="File"/> })>
                        <DropdownItem value="new">"New"</DropdownItem>
                        <DropdownItem value="open">"Open"</DropdownItem>
                        <Divider/>
                        <DropdownItem submenu=(rsx! {
                            <DropdownItem value="pdf">"PDF"</DropdownItem>
                            <DropdownItem value="docx">"Word document"</DropdownItem>
                            <DropdownItem value="xlsx">"Excel spreadsheet"</DropdownItem>
                            <DropdownItem value="csv">"CSV"</DropdownItem>
                        })>
                            "Export"
                        </DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Add the "<code>disabled</code>" property to any item to make it unselectable."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown trigger=(rsx! { <TriggerButton label="Payment method"/> })>
                    <DropdownItem value="cash">"Cash"</DropdownItem>
                    <DropdownItem value="check" disabled=true>"Personal check"</DropdownItem>
                    <DropdownItem value="credit">"Credit card"</DropdownItem>
                    <DropdownItem value="gift-card">"Gift card"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="Payment method"/> })>
                        <DropdownItem value="cash">"Cash"</DropdownItem>
                        <DropdownItem value="check" disabled=true>"Personal check"</DropdownItem>
                        <DropdownItem value="credit">"Credit card"</DropdownItem>
                        <DropdownItem value="gift-card">"Gift card"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="placement" anchor=true>
            "Placement"
        </Head>
        <p>"Set the "<code>placement</code>" property to control where the menu opens relative to the trigger: "
            <code>BottomStart</code>" (default), "<code>Bottom</code>", "<code>BottomEnd</code>", "<code>Top</code>
            "…, "<code>RightStart</code>" or "<code>LeftStart</code>
            ". The menu moves to a more optimal spot when the preferred placement doesn't have room."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown placement=RightStart trigger=(rsx! { <TriggerButton label="File formats"/> })>
                    <DropdownItem value="pdf">"PDF document"</DropdownItem>
                    <DropdownItem value="docx">"Word document"</DropdownItem>
                    <DropdownItem value="xlsx">"Excel spreadsheet"</DropdownItem>
                    <DropdownItem value="txt">"Plain text"</DropdownItem>
                    <DropdownItem value="json">"JSON file"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown placement=RightStart trigger=(rsx! { <TriggerButton label="File formats"/> })>
                        <DropdownItem value="pdf">"PDF document"</DropdownItem>
                        <DropdownItem value="docx">"Word document"</DropdownItem>
                        <DropdownItem value="xlsx">"Excel spreadsheet"</DropdownItem>
                        <DropdownItem value="txt">"Plain text"</DropdownItem>
                        <DropdownItem value="json">"JSON file"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="distance" anchor=true>
            "Distance"
        </Head>
        <p>"Set the "<code>distance</code>" property to change the gap between the menu and the trigger, in pixels."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown distance=30 trigger=(rsx! { <TriggerButton label="Edit"/> })>
                    <DropdownItem value="cut">"Cut"</DropdownItem>
                    <DropdownItem value="copy">"Copy"</DropdownItem>
                    <DropdownItem value="paste">"Paste"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown distance=30 trigger=(rsx! { <TriggerButton label="Edit"/> })>
                        <DropdownItem value="cut">"Cut"</DropdownItem>
                        <DropdownItem value="copy">"Copy"</DropdownItem>
                        <DropdownItem value="paste">"Paste"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="offset" anchor=true>
            "Offset"
        </Head>
        <p>"Set the "<code>skidding</code>" property to slide the menu along the trigger, in pixels."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown skidding=30 trigger=(rsx! { <TriggerButton label="Edit"/> })>
                    <DropdownItem value="cut">"Cut"</DropdownItem>
                    <DropdownItem value="copy">"Copy"</DropdownItem>
                    <DropdownItem value="paste">"Paste"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown skidding=30 trigger=(rsx! { <TriggerButton label="Edit"/> })>
                        <DropdownItem value="cut">"Cut"</DropdownItem>
                        <DropdownItem value="copy">"Copy"</DropdownItem>
                        <DropdownItem value="paste">"Paste"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"Dropdowns are sized relative to the current font size. Use the "<code>size</code>" property, a "
            <code>"size-*"</code>" class or a "<code>"font-size"</code>" style on the dropdown "
            "(or an ancestor element) to change it."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown size=Small trigger=(rsx! {
                    <Button appearance=Filled class=SIZE_SMALL>
                        "Small"
                        <span class=(END, " ", ICON)>
                            (fontawesome_ext::regular::ChevronDown)
                        </span>
                    </Button>
                })>
                    <DropdownItem value="option-1">"Option 1"</DropdownItem>
                    <DropdownItem value="option-2">"Option 2"</DropdownItem>
                    <DropdownItem value="option-3">"Option 3"</DropdownItem>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown size=Small trigger=(rsx! { <Button appearance=Filled class=SIZE_SMALL>"Small"</Button> })>
                        <DropdownItem value="option-1">"Option 1"</DropdownItem>
                        <DropdownItem value="option-2">"Option 2"</DropdownItem>
                        <DropdownItem value="option-3">"Option 3"</DropdownItem>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="reacting-to-selections" anchor=true>
            "Reacting to Selections"
        </Head>
        <p>"When an item is selected, the dropdown dispatches a bubbling "<code>"wg-select"</code>" event. Inspect "
            <code>"event.detail.item"</code>" for the selected item and "<code>"event.detail.value"</code>
            " for its "<code>value</code>". The event is cancelable: cancel it to keep the menu open."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="dropdown-select-demo">
                    <Dropdown trigger=(rsx! { <TriggerButton label="View"/> })>
                        <DropdownItem value="zoom-in" icon=(rsx! { (fontawesome::solid::House) })>
                            "Zoom in"
                        </DropdownItem>
                        <DropdownItem value="zoom-out" icon=(rsx! { (fontawesome::solid::House) })>
                            "Zoom out"
                        </DropdownItem>
                        <Divider/>
                        <DropdownItem value="actual">"Actual size"</DropdownItem>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown trigger=(rsx! { <TriggerButton label="View"/> })>
                        <DropdownItem value="zoom-in">"Zoom in"</DropdownItem>
                        <DropdownItem value="zoom-out">"Zoom out"</DropdownItem>
                        <Divider/>
                        <DropdownItem value="actual">"Actual size"</DropdownItem>
                    </Dropdown>

                    // Report the selection
                    document.add_steady_event_listener("wg-select", |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring for the two demos that report selections: `.dropdown-select-demo`
/// prints the selected value, `.dropdown-checkable-demo` prints the new state of
/// a checkable item and keeps the menu open by canceling the event.
pub fn listen_dropdown_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener(event::SELECT, |event| {
        report_selection(&event);
    });
}

fn report_selection(event: &Event) -> Option<()> {
    let dropdown = event.target()?.maybe_into_element()?;
    let demo = dropdown
        .closest(".dropdown-select-demo, .dropdown-checkable-demo")
        .ok()??;
    let output = demo.query_selector(".dropdown-demo-output").ok()??;

    let custom: &CustomEvent = event.dyn_ref()?;
    let detail = custom.detail();
    let item = detail.get("item").dyn_into::<Element>().ok()?;
    let value = detail
        .get("value")
        .as_string()
        .unwrap_or_else(|| item.text_content().unwrap_or_default().trim().to_string());

    let checkable = item.get_attribute("role").as_deref() == Some("menuitemcheckbox");
    let text = if checkable {
        let checked = if item.class_list().contains("checked") {
            "checked"
        } else {
            "unchecked"
        };
        format!("{value}: {checked}")
    } else {
        format!("Selected: {value}")
    };
    output.set_text_content(Some(&text));

    // Toggling a checkable item keeps the menu open
    if checkable {
        event.prevent_default();
    }

    Some(())
}
