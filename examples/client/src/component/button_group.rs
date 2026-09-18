use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::{fontawesome, fontawesome_ext};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{CLUSTER, END, GAP_S, ICON};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::button_group::ButtonGroup;
use wingy_hypertext::component::dropdown::{Dropdown, DropdownItem, DropdownItemLabel, DropdownMenu};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::tooltip::Tooltip;
use wingy_hypertext::helper::popup::PopupPlacement::BottomEnd;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::orientation::Orientation::*;
use wingy_hypertext::variant::Variant::*;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Button Group"</Head>
        <p>"Button groups combine related buttons into a single visual unit. Use them for toolbars, "
            "segmented controls, or any set of actions that belong together. The buttons keep their own "
            "props — the group only lays them out, collapsing the borders between neighbours and rounding "
            "the outer edges of the first and the last button."
        </p>
        <p>"Give every group a "<code>label</code>". It isn't shown on screen, but assistive devices "
            "announce it so people know what the grouped buttons control."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Alignment">
                    <Button appearance=Filled>"Left"</Button>
                    <Button appearance=Filled>"Center"</Button>
                    <Button appearance=Filled>"Right"</Button>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Alignment">
                        <Button appearance=Filled>"Left"</Button>
                        <Button appearance=Filled>"Center"</Button>
                        <Button appearance=Filled>"Right"</Button>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="orientation" anchor=true>
            "Orientation"
        </Head>
        <p>"Set "<code>"orientation=Vertical"</code>" to stack the buttons instead of placing them side by side."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Options" orientation=Vertical>
                    <Button appearance=Filled>"Top"</Button>
                    <Button appearance=Filled>"Middle"</Button>
                    <Button appearance=Filled>"Bottom"</Button>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Options" orientation=Vertical>
                        <Button appearance=Filled>"Top"</Button>
                        <Button appearance=Filled>"Middle"</Button>
                        <Button appearance=Filled>"Bottom"</Button>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="pill" anchor=true>
            "Pill"
        </Head>
        <p>"Set "<code>pill</code>" on every button to round the group's outer edges."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Alignment">
                    <Button appearance=Filled pill=true>"Left"</Button>
                    <Button appearance=Filled pill=true>"Center"</Button>
                    <Button appearance=Filled pill=true>"Right"</Button>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Alignment">
                        <Button appearance=Filled pill=true>"Left"</Button>
                        <Button appearance=Filled pill=true>"Center"</Button>
                        <Button appearance=Filled pill=true>"Right"</Button>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="dropdowns" anchor=true>
            "Dropdowns"
        </Head>
        <p>"Place a "<code>Dropdown</code>" anywhere in the group to attach a menu of related actions."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Options">
                    <Button appearance=Filled>"Edit"</Button>
                    <Dropdown>
                        <Button appearance=Filled>
                            "More"
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
                            <DropdownItem value="paste">
                                <DropdownItemLabel>"Paste"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                    <Button appearance=Filled>"Delete"</Button>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Options">
                        <Button appearance=Filled>"Edit"</Button>
                        <Dropdown>
                            <Button appearance=Filled>
                                "More"
                                <span class=(END, " ", ICON)>
                                    (fontawesome_ext::regular::ChevronDown)
                                </span>
                            </Button>
                            <DropdownMenu>
                                <DropdownItem value="cut">
                                    <DropdownItemLabel>"Cut"</DropdownItemLabel>
                                </DropdownItem>
                                ...
                            </DropdownMenu>
                        </Dropdown>
                        <Button appearance=Filled>"Delete"</Button>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="split-buttons" anchor=true>
            "Split Buttons"
        </Head>
        <p>"Pair a primary button with a dropdown to make a split button. Give the dropdown trigger an "
            "accessible label so people using assistive devices know what it opens."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Save">
                    <Button appearance=Filled variant=Brand>"Save"</Button>
                    <Dropdown placement=BottomEnd>
                        <Button appearance=Filled variant=Brand attrs=(attrs!["aria-label" = &"More options"])>
                            <span class=ICON>
                                (fontawesome_ext::regular::ChevronDown)
                            </span>
                        </Button>
                        <DropdownMenu>
                            <DropdownItem value="save">
                                <DropdownItemLabel>"Save"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="save-as">
                                <DropdownItemLabel>"Save as…"</DropdownItemLabel>
                            </DropdownItem>
                            <DropdownItem value="save-all">
                                <DropdownItemLabel>"Save all"</DropdownItemLabel>
                            </DropdownItem>
                        </DropdownMenu>
                    </Dropdown>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Save">
                        <Button appearance=Filled variant=Brand>"Save"</Button>
                        <Dropdown placement=BottomEnd>
                            <Button appearance=Filled variant=Brand attrs=(attrs!["aria-label" = &"More options"])>
                                <span class=ICON>
                                    (fontawesome_ext::regular::ChevronDown)
                                </span>
                            </Button>
                            <DropdownMenu>
                                <DropdownItem value="save">
                                    <DropdownItemLabel>"Save"</DropdownItemLabel>
                                </DropdownItem>
                                ...
                            </DropdownMenu>
                        </Dropdown>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="tooltips" anchor=true>
            "Tooltips"
        </Head>
        <p>"Pair each button with a "<code>Tooltip</code>" to explain what it does on hover and focus."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Alignment">
                    <Button appearance=Filled id="button-group-left">"Left"</Button>
                    <Button appearance=Filled id="button-group-center">"Center"</Button>
                    <Button appearance=Filled id="button-group-right">"Right"</Button>
                </ButtonGroup>
                <Tooltip anchor_id="button-group-left">"Align left"</Tooltip>
                <Tooltip anchor_id="button-group-center">"Align center"</Tooltip>
                <Tooltip anchor_id="button-group-right">"Align right"</Tooltip>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Alignment">
                        <Button appearance=Filled id="button-left">"Left"</Button>
                        <Button appearance=Filled id="button-center">"Center"</Button>
                        <Button appearance=Filled id="button-right">"Right"</Button>
                    </ButtonGroup>
                    <Tooltip anchor_id="button-left">"Align left"</Tooltip>
                    <Tooltip anchor_id="button-center">"Align center"</Tooltip>
                    <Tooltip anchor_id="button-right">"Align right"</Tooltip>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="toolbars" anchor=true>
            "Toolbars"
        </Head>
        <p>"Combine several button groups into a toolbar of related action sets. Use icon-only buttons with "
            "an "<code>"aria-label"</code>" for compact controls, and tooltips to name each one."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=(CLUSTER, " ", GAP_S)>
                    <ButtonGroup label="History">
                        <Button appearance=Filled id="toolbar-undo" attrs=(attrs!["aria-label" = &"Undo"])>
                            <span class=ICON>(fontawesome::solid::Undo)</span>
                        </Button>
                        <Button appearance=Filled id="toolbar-redo" attrs=(attrs!["aria-label" = &"Redo"])>
                            <span class=ICON>(fontawesome::solid::Redo)</span>
                        </Button>
                    </ButtonGroup>
                    <ButtonGroup label="Formatting">
                        <Button appearance=Filled id="toolbar-bold" attrs=(attrs!["aria-label" = &"Bold"])>
                            <span class=ICON>(fontawesome::solid::Bold)</span>
                        </Button>
                        <Button appearance=Filled id="toolbar-italic" attrs=(attrs!["aria-label" = &"Italic"])>
                            <span class=ICON>(fontawesome::solid::Italic)</span>
                        </Button>
                        <Button appearance=Filled id="toolbar-underline" attrs=(attrs!["aria-label" = &"Underline"])>
                            <span class=ICON>(fontawesome::solid::Underline)</span>
                        </Button>
                    </ButtonGroup>
                    <ButtonGroup label="Alignment">
                        <Button appearance=Filled id="toolbar-align-left" attrs=(attrs!["aria-label" = &"Align left"])>
                            <span class=ICON>(fontawesome::solid::AlignLeft)</span>
                        </Button>
                        <Button appearance=Filled id="toolbar-align-center" attrs=(attrs!["aria-label" = &"Align center"])>
                            <span class=ICON>(fontawesome::solid::AlignCenter)</span>
                        </Button>
                        <Button appearance=Filled id="toolbar-align-right" attrs=(attrs!["aria-label" = &"Align right"])>
                            <span class=ICON>(fontawesome::solid::AlignRight)</span>
                        </Button>
                    </ButtonGroup>

                    <Tooltip anchor_id="toolbar-undo">"Undo"</Tooltip>
                    <Tooltip anchor_id="toolbar-redo">"Redo"</Tooltip>

                    <Tooltip anchor_id="toolbar-bold">"Bold"</Tooltip>
                    <Tooltip anchor_id="toolbar-italic">"Italic"</Tooltip>
                    <Tooltip anchor_id="toolbar-underline">"Underline"</Tooltip>

                    <Tooltip anchor_id="toolbar-align-left">"Align left"</Tooltip>
                    <Tooltip anchor_id="toolbar-align-center">"Align center"</Tooltip>
                    <Tooltip anchor_id="toolbar-align-right">"Align right"</Tooltip>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=(CLUSTER, " ", GAP_S)>
                        <ButtonGroup label="History">
                            <Button appearance=Filled id="toolbar-undo" attrs=(attrs!["aria-label" = &"Undo"])>
                                <span class=ICON>(fontawesome::solid::Undo)</span>
                            </Button>
                            <Button appearance=Filled id="toolbar-redo" attrs=(attrs!["aria-label" = &"Redo"])>
                                <span class=ICON>(fontawesome::solid::Redo)</span>
                            </Button>
                        </ButtonGroup>
                        ...
                        <Tooltip anchor_id="toolbar-undo">"Undo"</Tooltip>
                        <Tooltip anchor_id="toolbar-redo">"Redo"</Tooltip>
                        ...
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="native-buttons" anchor=true>
            "Native Buttons"
        </Head>
        <p>"Button groups also work with native "<code>"<button>"</code>" elements: the group passes its "
            "layout down through custom properties the native styles read as well."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ButtonGroup label="Alignment">
                    <button class="wa-filled">"Left"</button>
                    <button class="wa-filled">"Center"</button>
                    <button class="wa-filled">"Right"</button>
                </ButtonGroup>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ButtonGroup label="Alignment">
                        <button class="wa-filled">"Left"</button>
                        <button class="wa-filled">"Center"</button>
                        <button class="wa-filled">"Right"</button>
                    </ButtonGroup>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
