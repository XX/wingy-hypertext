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
use wingy_hypertext::component::dropdown::{
    Dropdown, DropdownItem, DropdownItemDetails, DropdownItemIcon, DropdownItemLabel, DropdownMenu, DropdownSubmenu,
};
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
        <p>"A dropdown is composed of a trigger — the first child, any element that opens the menu — and a "
            <code>DropdownMenu</code>" holding the items. Every item composes its own content out of a "
            <code>DropdownItemIcon</code>", a "<code>DropdownItemLabel</code>", a "<code>DropdownItemDetails</code>
            " and a "<code>DropdownSubmenu</code>", all of them optional except the label."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown>
                    <TriggerButton label="Options"/>
                    <DropdownMenu>
                        <DropdownItem value="edit">
                            <DropdownItemLabel>"Edit"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="duplicate">
                            <DropdownItemLabel>"Duplicate"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="delete">
                            <DropdownItemLabel>"Delete"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Options"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="edit">
                                <DropdownItemLabel>"Edit"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="duplicate">
                                <DropdownItemLabel>"Duplicate"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="delete">
                                <DropdownItemLabel>"Delete"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
        <p>"Put a "<code>DropdownItemIcon</code>" before the label to add an icon to an item."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown>
                    <TriggerButton label="Edit"/>
                    <DropdownMenu>
                        <DropdownItem value="copy">
                            <DropdownItemIcon>(fontawesome::solid::Copy)</DropdownItemIcon>
                            <DropdownItemLabel>"Copy"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="rename">
                            <DropdownItemIcon>(fontawesome::solid::PenToSquare)</DropdownItemIcon>
                            <DropdownItemLabel>"Rename"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="bookmark">
                            <DropdownItemIcon>(fontawesome::solid::Bookmark)</DropdownItemIcon>
                            <DropdownItemLabel>"Bookmark"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="delete" variant=Danger>
                            <DropdownItemIcon>(fontawesome::solid::XmarkCircle)</DropdownItemIcon>
                            <DropdownItemLabel>"Delete"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Edit"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="copy">
                                <DropdownItemIcon>(fontawesome::solid::Copy)</DropdownItemIcon>
                                <DropdownItemLabel>"Copy"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="rename">
                                <DropdownItemIcon>(fontawesome::solid::PenToSquare)</DropdownItemIcon>
                                <DropdownItemLabel>"Rename"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="delete" variant=Danger>
                                <DropdownItemIcon>(fontawesome::solid::XmarkCircle)</DropdownItemIcon>
                                <DropdownItemLabel>"Delete"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown>
                    <TriggerButton label="Device"/>
                    <DropdownMenu>
                        <h3>"Type"</h3>
                        <DropdownItem value="phone">
                            <DropdownItemLabel>"Phone"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="tablet">
                            <DropdownItemLabel>"Tablet"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="desktop">
                            <DropdownItemLabel>"Desktop"</DropdownItemLabel>
                        </DropdownItem>
                        <Divider/>
                        <DropdownItem value="more">
                            <DropdownItemLabel>"More options…"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Device"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <h3>"Type"</h3>
                            <DropdownItem value="phone">
                                <DropdownItemLabel>"Phone"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="tablet">
                                <DropdownItemLabel>"Tablet"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem value="more">
                                <DropdownItemLabel>"More options…"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="showing-details" anchor=true>
            "Showing Details"
        </Head>
        <p>"Put a "<code>DropdownItemDetails</code>" after the label to show secondary content, "
            "such as a keyboard shortcut."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Dropdown>
                    <TriggerButton label="Message"/>
                    <DropdownMenu>
                        <DropdownItem value="reply">
                            <DropdownItemLabel>"Reply"</DropdownItemLabel>
                            <DropdownItemDetails>"⌘R"</DropdownItemDetails>
                        </DropdownItem>
                        <DropdownItem value="forward">
                            <DropdownItemLabel>"Forward"</DropdownItemLabel>
                            <DropdownItemDetails>"⌘F"</DropdownItemDetails>
                        </DropdownItem>
                        <DropdownItem value="move">
                            <DropdownItemLabel>"Move"</DropdownItemLabel>
                            <DropdownItemDetails>"⌘M"</DropdownItemDetails>
                        </DropdownItem>
                        <Divider/>
                        <DropdownItem value="archive">
                            <DropdownItemLabel>"Archive"</DropdownItemLabel>
                            <DropdownItemDetails>"⌘A"</DropdownItemDetails>
                        </DropdownItem>
                        <DropdownItem value="delete" variant=Danger>
                            <DropdownItemLabel>"Delete"</DropdownItemLabel>
                            <DropdownItemDetails>"Del"</DropdownItemDetails>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Message"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="reply">
                                <DropdownItemLabel>"Reply"</DropdownItemLabel>
                                <DropdownItemDetails>"⌘R"</DropdownItemDetails>
                            </DropdownItem>
                            <DropdownItem value="forward">
                                <DropdownItemLabel>"Forward"</DropdownItemLabel>
                                <DropdownItemDetails>"⌘F"</DropdownItemDetails>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem value="delete" variant=Danger>
                                <DropdownItemLabel>"Delete"</DropdownItemLabel>
                                <DropdownItemDetails>"Del"</DropdownItemDetails>
                            </DropdownItem>
                        </DropdownMenu>
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
                    <Dropdown>
                        <TriggerButton label="View"/>
                        <DropdownMenu>
                            <DropdownItem checkbox=true checked=true value="canvas">
                                <DropdownItemLabel>"Show canvas"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem checkbox=true checked=true value="grid">
                                <DropdownItemLabel>"Show grid"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem checkbox=true value="source">
                                <DropdownItemLabel>"Show source"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem value="preferences">
                                <DropdownItemLabel>"Preferences…"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "View"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem checkbox=true checked=true value="canvas">
                                <DropdownItemLabel>"Show canvas"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem checkbox=true value="source">
                                <DropdownItemLabel>"Show source"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown>
                    <TriggerButton label="Project"/>
                    <DropdownMenu>
                        <DropdownItem value="share">
                            <DropdownItemIcon>(fontawesome::solid::Copy)</DropdownItemIcon>
                            <DropdownItemLabel>"Share"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="preferences">
                            <DropdownItemIcon>(fontawesome::solid::Gear)</DropdownItemIcon>
                            <DropdownItemLabel>"Preferences"</DropdownItemLabel>
                        </DropdownItem>
                        <Divider/>
                        <h3>"Danger zone"</h3>
                        <DropdownItem value="archive">
                            <DropdownItemIcon>(fontawesome::solid::Bookmark)</DropdownItemIcon>
                            <DropdownItemLabel>"Archive"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="delete" variant=Danger>
                            <DropdownItemIcon>(fontawesome::solid::XmarkCircle)</DropdownItemIcon>
                            <DropdownItemLabel>"Delete"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Project"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="share">
                                <DropdownItemIcon>(fontawesome::solid::Copy)</DropdownItemIcon>
                                <DropdownItemLabel>"Share"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <h3>"Danger zone"</h3>
                            <DropdownItem value="delete" variant=Danger>
                                <DropdownItemIcon>(fontawesome::solid::XmarkCircle)</DropdownItemIcon>
                                <DropdownItemLabel>"Delete"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="submenus" anchor=true>
            "Submenus"
        </Head>
        <p>"To nest a menu, put a "<code>DropdownSubmenu</code>" with the nested items last into an item and set "
            <code>"submenu=true"</code>" on it — the flag is what announces the item as a submenu trigger to "
            "assistive technology. A submenu opens on hover, on "<code>"→"</code>" and on activation; "
            <code>"←"</code>" and "<code>Escape</code>" collapse it again. An item that opens a submenu doesn't "
            "emit "<code>"wg-select"</code>" itself."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class="dropdown-select-demo">
                    <Dropdown>
                        <TriggerButton label="File"/>
                        <DropdownMenu>
                            <DropdownItem value="new">
                                <DropdownItemLabel>"New"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="open">
                                <DropdownItemLabel>"Open"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem submenu=true>
                                <DropdownItemLabel>"Export"</DropdownItemLabel>
                                <DropdownSubmenu>
                                    <DropdownItem value="pdf">
                                        <DropdownItemLabel>"PDF"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem value="docx">
                                        <DropdownItemLabel>"Word document"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem value="xlsx">
                                        <DropdownItemLabel>"Excel spreadsheet"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem value="csv">
                                        <DropdownItemLabel>"CSV"</DropdownItemLabel>
                                    </DropdownItem>
                                </DropdownSubmenu>
                            </DropdownItem>
                            <DropdownItem submenu=true>
                                <DropdownItemLabel>"Options"</DropdownItemLabel>
                                <DropdownSubmenu>
                                    <DropdownItem checkbox=true value="compress">
                                        <DropdownItemLabel>"Compress files"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem checkbox=true checked=true value="metadata">
                                        <DropdownItemLabel>"Include metadata"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem checkbox=true value="password">
                                        <DropdownItemLabel>"Password protect"</DropdownItemLabel>
                                    </DropdownItem>
                                </DropdownSubmenu>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "File"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="new">
                                <DropdownItemLabel>"New"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem submenu=true>
                                <DropdownItemLabel>"Export"</DropdownItemLabel>
                                <DropdownSubmenu>
                                    <DropdownItem value="pdf">
                                        <DropdownItemLabel>"PDF"</DropdownItemLabel>
                                    </DropdownItem>
                                    <DropdownItem value="docx">
                                        <DropdownItemLabel>"Word document"</DropdownItemLabel>
                                    </DropdownItem>
                                </DropdownSubmenu>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown>
                    <TriggerButton label="Payment method"/>
                    <DropdownMenu>
                        <DropdownItem value="cash">
                            <DropdownItemLabel>"Cash"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="check" disabled=true>
                            <DropdownItemLabel>"Personal check"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="credit">
                            <DropdownItemLabel>"Credit card"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="gift-card">
                            <DropdownItemLabel>"Gift card"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "Payment method"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="cash">
                                <DropdownItemLabel>"Cash"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="check" disabled=true>
                                <DropdownItemLabel>"Personal check"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown placement=RightStart>
                    <TriggerButton label="File formats"/>
                    <DropdownMenu>
                        <DropdownItem value="pdf">
                            <DropdownItemLabel>"PDF document"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="docx">
                            <DropdownItemLabel>"Word document"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="xlsx">
                            <DropdownItemLabel>"Excel spreadsheet"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="txt">
                            <DropdownItemLabel>"Plain text"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="json">
                            <DropdownItemLabel>"JSON file"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown placement=RightStart>
                        <Button appearance=Filled>
                            "File formats"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="pdf">
                                <DropdownItemLabel>"PDF document"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="docx">
                                <DropdownItemLabel>"Word document"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown distance=30>
                    <TriggerButton label="Edit"/>
                    <DropdownMenu>
                        <DropdownItem value="cut">
                            <DropdownItemLabel>"Cut"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="copy">
                            <DropdownItemLabel>"Copy"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="paste">
                            <DropdownItemLabel>"Paste"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown distance=30>
                        <Button appearance=Filled>
                            "Edit"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="cut">
                                <DropdownItemLabel>"Cut"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="copy">
                                <DropdownItemLabel>"Copy"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown skidding=30>
                    <TriggerButton label="Edit"/>
                    <DropdownMenu>
                        <DropdownItem value="cut">
                            <DropdownItemLabel>"Cut"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="copy">
                            <DropdownItemLabel>"Copy"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="paste">
                            <DropdownItemLabel>"Paste"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown skidding=30>
                        <Button appearance=Filled>
                            "Edit"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="cut">
                                <DropdownItemLabel>"Cut"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="copy">
                                <DropdownItemLabel>"Copy"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                <Dropdown size=Small>
                    <Button appearance=Filled class=SIZE_SMALL>
                        "Small"
                        <span class=(END, " ", ICON)>
                            (fontawesome_ext::regular::ChevronDown)
                        </span>
                    </Button>
                    <DropdownMenu>
                        <DropdownItem value="option-1">
                            <DropdownItemLabel>"Option 1"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="option-2">
                            <DropdownItemLabel>"Option 2"</DropdownItemLabel>
                        </DropdownItem>
                        <DropdownItem value="option-3">
                            <DropdownItemLabel>"Option 3"</DropdownItemLabel>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown size=Small>
                        <Button appearance=Filled class=SIZE_SMALL>
                            "Small"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="option-1">
                                <DropdownItemLabel>"Option 1"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="option-2">
                                <DropdownItemLabel>"Option 2"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
                    <Dropdown>
                        <TriggerButton label="View"/>
                        <DropdownMenu>
                            <DropdownItem value="zoom-in">
                                <DropdownItemIcon>(fontawesome::solid::House)</DropdownItemIcon>
                                <DropdownItemLabel>"Zoom in"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="zoom-out">
                                <DropdownItemIcon>(fontawesome::solid::House)</DropdownItemIcon>
                                <DropdownItemLabel>"Zoom out"</DropdownItemLabel>
                            </DropdownItem>
                            <Divider/>
                            <DropdownItem value="actual">
                                <DropdownItemLabel>"Actual size"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                    <p class="dropdown-demo-output">"Nothing selected yet"</p>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Dropdown>
                        <Button appearance=Filled>
                            "View"
                            <span class=(END, " ", ICON)>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="zoom-in">
                                <DropdownItemLabel>"Zoom in"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="zoom-out">
                                <DropdownItemLabel>"Zoom out"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
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
