use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Element, Event};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::class::{CLUSTER, SPLIT, STACK};
use wingy_hypertext::component::badge::Badge;
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::icon_placement::ExpandIconPlacement::*;
use wingy_hypertext::layout::accordion::AccordionMode::*;
use wingy_hypertext::layout::accordion::{
    Accordion, AccordionItem, AccordionItemIcon, AccordionItemLabel, AccordionItemPanel, AccordionItemTrigger,
};
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext::variant::Variant::*;
use wingy_hypertext_web::layout::accordion;
use wingy_hypertext_web::util::event;

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Accordion"</Head>
        <p>"Accordions are a vertically stacked set of interactive headings that each contain a title, representing "
            "a section of content. Each "<code>"AccordionItem"</code>" wraps its trigger button in a heading, per the "
            "W3C accordion pattern; the client adds the animation, the modes and the keyboard navigation."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="What is Web Awesome?">
                        "Web Awesome is a comprehensive library of web components you can use to build beautiful, accessible web "
                        "applications. It's built on open web standards and works with any framework."
                    </AccordionItem>
                    <AccordionItem label="Is it free to use?">
                        "The core Web Awesome library is completely free and open source. A Pro tier is also available with "
                        "additional components and features."
                    </AccordionItem>
                    <AccordionItem label="Does it work with my framework?">
                        "Yes! This port renders plain HTML from Rust components, so it works with any server or client "
                        "that renders hypertext."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="What is Web Awesome?">
                            "Web Awesome is a comprehensive library of web components you can use to build beautiful, accessible web "
                            "applications. It's built on open web standards and works with any framework."
                        </AccordionItem>
                        <AccordionItem label="Is it free to use?">
                            "The core Web Awesome library is completely free and open source. A Pro tier is also available with "
                            "additional components and features."
                        </AccordionItem>
                        <AccordionItem label="Does it work with my framework?">
                            "Yes! This port renders plain HTML from Rust components, so it works with any server or client "
                            "that renders hypertext."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="expanded-initially" anchor=true>
            "Expanded Initially"
        </Head>
        <p>"Use the "<code>"expanded"</code>" property on an accordion item to expand it by default."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="Already open" expanded=true>
                        "This item is expanded by default. Click the header to collapse it."
                    </AccordionItem>
                    <AccordionItem label="Click to open">
                        "This item starts collapsed. Click the header to expand it."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="Already open" expanded=true>
                            "This item is expanded by default. Click the header to collapse it."
                        </AccordionItem>
                        <AccordionItem label="Click to open">
                            "This item starts collapsed. Click the header to expand it."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Use the "<code>"disabled"</code>" property on an accordion item to prevent it from being toggled."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="Active item" expanded=true>
                        "This item can be expanded and collapsed normally."
                    </AccordionItem>
                    <AccordionItem label="Disabled item" disabled=true>
                        "This item is disabled and cannot be toggled."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="Active item" expanded=true>
                            "This item can be expanded and collapsed normally."
                        </AccordionItem>
                        <AccordionItem label="Disabled item" disabled=true>
                            "This item is disabled and cannot be toggled."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="heading-level" anchor=true>
            "Heading Level"
        </Head>
        <p>"Each accordion item wraps its trigger in a heading so screen reader users can navigate to it. The "
            "default is "<code>"<h3>"</code>". Set the "<code>"heading_level"</code>" property on the items to match "
            "your page's hierarchy. The level is semantic only; the accordion inherits the surrounding font, so the "
            "appearance is identical at every level."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="Section one" heading_level=H2>
                        "This trigger is wrapped in an "<code>"<h2>"</code>"."
                    </AccordionItem>
                    <AccordionItem label="Section two" heading_level=H2>
                        "Match the level to where this accordion sits in your document outline."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="Section one" heading_level=H2>
                            "This trigger is wrapped in an "<code>"<h2>"</code>"."
                        </AccordionItem>
                        <AccordionItem label="Section two" heading_level=H2>
                            "Match the level to where this accordion sits in your document outline."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"If an accordion lives outside the document outline (inside a nav or another component with its own "
            "structure), set "<code>"heading_level"</code>" to "<code>"HeadingLevel::NoHeading"</code>" to omit the heading "
            "wrapper and render the button directly."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="Settings" heading_level=NoHeading>
                        "Adjust your preferences here."
                    </AccordionItem>
                    <AccordionItem label="Notifications" heading_level=NoHeading>
                        "Manage how and when you receive notifications."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="Settings" heading_level=NoHeading>
                            "Adjust your preferences here."
                        </AccordionItem>
                        <AccordionItem label="Notifications" heading_level=NoHeading>
                            "Manage how and when you receive notifications."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="size" anchor=true>
            "Size"
        </Head>
        <p>"The accordion's text and expand/collapse icon scale with "<code>"font-size"</code>". Setting "
            <code>"font-size"</code>" on an accordion proportionally resizes the type and icon together."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Accordion style="font-size: 0.875rem;">
                        <AccordionItem label="Small accordion">
                            "Text and icon scale down together."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "Content here."
                        </AccordionItem>
                    </Accordion>
                    <Accordion>
                        <AccordionItem label="Default accordion">
                            "The default size."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "Content here."
                        </AccordionItem>
                    </Accordion>
                    <Accordion style="font-size: 1.25rem;">
                        <AccordionItem label="Large accordion">
                            "Everything scales up together."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "Content here."
                        </AccordionItem>
                    </Accordion>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Accordion style="font-size: 0.875rem;">
                            <AccordionItem label="Small accordion">
                                "Text and icon scale down together."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "Content here."
                            </AccordionItem>
                        </Accordion>
                        <Accordion>
                            <AccordionItem label="Default accordion">
                                "The default size."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "Content here."
                            </AccordionItem>
                        </Accordion>
                        <Accordion style="font-size: 1.25rem;">
                            <AccordionItem label="Large accordion">
                                "Everything scales up together."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "Content here."
                            </AccordionItem>
                        </Accordion>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>"appearance"</code>" property to change the accordion's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Accordion>
                        <AccordionItem label="Outlined (default)">
                            "This is the default outlined appearance. It has a subtle border that helps it stand out "
                            "without being too flashy."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "More content here."
                        </AccordionItem>
                    </Accordion>
                    <Accordion appearance=FilledOutlined>
                        <AccordionItem label="Filled-outlined">
                            "The filled-outlined appearance combines a filled background with an outline. It gives the "
                            "accordion a bit more visual weight."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "More content here."
                        </AccordionItem>
                    </Accordion>
                    <Accordion appearance=Filled>
                        <AccordionItem label="Filled">
                            "The filled appearance adds a background color to each item. Use this when you want the "
                            "accordion to really pop on the page."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "More content here."
                        </AccordionItem>
                    </Accordion>
                    <Accordion appearance=Plain>
                        <AccordionItem label="Plain">
                            "No bells and whistles on this one. The plain appearance strips away borders and backgrounds "
                            "for a minimalist look."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "More content here."
                        </AccordionItem>
                    </Accordion>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Accordion>
                            <AccordionItem label="Outlined (default)">
                                "This is the default outlined appearance. It has a subtle border that helps it stand out "
                                "without being too flashy."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "More content here."
                            </AccordionItem>
                        </Accordion>
                        <Accordion appearance=FilledOutlined>
                            <AccordionItem label="Filled-outlined">
                                "The filled-outlined appearance combines a filled background with an outline. It gives the "
                                "accordion a bit more visual weight."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "More content here."
                            </AccordionItem>
                        </Accordion>
                        <Accordion appearance=Filled>
                            <AccordionItem label="Filled">
                                "The filled appearance adds a background color to each item. Use this when you want the "
                                "accordion to really pop on the page."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "More content here."
                            </AccordionItem>
                        </Accordion>
                        <Accordion appearance=Plain>
                            <AccordionItem label="Plain">
                                "No bells and whistles on this one. The plain appearance strips away borders and backgrounds "
                                "for a minimalist look."
                            </AccordionItem>
                            <AccordionItem label="Another item">
                                "More content here."
                            </AccordionItem>
                        </Accordion>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="mode" anchor=true>
            "Mode"
        </Head>
        <p>"Use the "<code>"mode"</code>" property to control how items can be expanded: "<code>"Multiple"</code>
            " (the default) lets any number of items be open at once; "<code>"Single"</code>" keeps one item open at "
            "a time — opening another collapses it, and clicking the open item keeps it open."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion mode=Single>
                    <AccordionItem label="Section one" expanded=true>
                        "Opening another section will automatically collapse this one. Only one section can be open at a time."
                    </AccordionItem>
                    <AccordionItem label="Section two">
                        "Try opening this section to see section one collapse automatically."
                    </AccordionItem>
                    <AccordionItem label="Section three">
                        "Clicking an already-open section won't close it — open another instead."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion mode=Single>
                        <AccordionItem label="Section one" expanded=true>
                            "Opening another section will automatically collapse this one. Only one section can be open at a time."
                        </AccordionItem>
                        <AccordionItem label="Section two">
                            "Try opening this section to see section one collapse automatically."
                        </AccordionItem>
                        <AccordionItem label="Section three">
                            "Clicking an already-open section won't close it — open another instead."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
        <p>"Use "<code>"SingleCollapsible"</code>" when you want the same one-at-a-time constraint but still want "
            "users to be able to close every section."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion mode=SingleCollapsible>
                    <AccordionItem label="Filters">
                        "Opening another section will collapse this one, and clicking the open section closes it."
                    </AccordionItem>
                    <AccordionItem label="Sort">
                        "Try opening and closing each section in turn."
                    </AccordionItem>
                    <AccordionItem label="Display">
                        "Zero open sections is a valid state in this mode."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion mode=SingleCollapsible>
                        <AccordionItem label="Filters">
                            "Opening another section will collapse this one, and clicking the open section closes it."
                        </AccordionItem>
                        <AccordionItem label="Sort">
                            "Try opening and closing each section in turn."
                        </AccordionItem>
                        <AccordionItem label="Display">
                            "Zero open sections is a valid state in this mode."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="icon-placement" anchor=true>
            "Icon Placement"
        </Head>
        <p>"The expand/collapse icon appears at the end of each header by default. Set "<code>"icon_placement"</code>
            " to "<code>"Start"</code>" to move it to the beginning, a common pattern for sidebars and tree style "
            "navigation."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion icon_placement=Start>
                    <AccordionItem label="Start">
                        "Icon is at the start of the header."
                    </AccordionItem>
                    <AccordionItem label="Another item">
                        "More content here."
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion icon_placement=Start>
                        <AccordionItem label="Start">
                            "Icon is at the start of the header."
                        </AccordionItem>
                        <AccordionItem label="Another item">
                            "More content here."
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="custom-icon" anchor=true>
            "Custom Icon"
        </Head>
        <p>"To replace the default expand/collapse icon, compose the item from "<code>"AccordionItemTrigger"</code>
            " with "<code>"bare"</code>" and "<code>"AccordionItemPanel"</code>", and put any icon in "
            <code>"AccordionItemIcon"</code>". By default the icon rotates as the item expands: target "
            <code>".accordion-item-icon"</code>" to customize the rotation, or set "<code>"rotate: none"</code>
            " and swap the icon instead."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem class="circle-plus">
                        <AccordionItemTrigger bare=true>
                            <AccordionItemLabel>"Rotate a custom icon"</AccordionItemLabel>
                            <AccordionItemIcon>(fontawesome::solid::Plus)</AccordionItemIcon>
                        </AccordionItemTrigger>
                        <AccordionItemPanel>"Replace the default chevron and customize how it rotates."</AccordionItemPanel>
                    </AccordionItem>
                    <AccordionItem class="plus-minus">
                        <AccordionItemTrigger bare=true>
                            <AccordionItemLabel>"Swap the icon instead"</AccordionItemLabel>
                            <AccordionItemIcon>
                                <span data-when="collapsed">(fontawesome::solid::Plus)</span>
                                <span data-when="expanded">(fontawesome::solid::Minus)</span>
                            </AccordionItemIcon>
                        </AccordionItemTrigger>
                        <AccordionItemPanel>"Prevent the rotation and swap + for − when the item expands."</AccordionItemPanel>
                    </AccordionItem>
                </Accordion>
                <style>"
                    /* Customize the rotation when expanded */
                    .circle-plus.expanded .accordion-item-icon {
                        rotate: 225deg;
                    }

                    /* Prevent the default rotation animation… */
                    .plus-minus .accordion-item-icon {
                        rotate: none;
                    }

                    /* …and swap the icon based on the expanded state */
                    .plus-minus [data-when] {
                        display: inline-flex;
                    }

                    .plus-minus:not(.expanded) [data-when='expanded'],
                    .plus-minus.expanded [data-when='collapsed'] {
                        display: none;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem class="circle-plus">
                            <AccordionItemTrigger bare=true>
                                <AccordionItemLabel>"Rotate a custom icon"</AccordionItemLabel>
                                <AccordionItemIcon>(fontawesome::solid::Plus)</AccordionItemIcon>
                            </AccordionItemTrigger>
                            <AccordionItemPanel>"Replace the default chevron and customize how it rotates."</AccordionItemPanel>
                        </AccordionItem>
                        <AccordionItem class="plus-minus">
                            <AccordionItemTrigger bare=true>
                                <AccordionItemLabel>"Swap the icon instead"</AccordionItemLabel>
                                <AccordionItemIcon>
                                    <span data-when="collapsed">(fontawesome::solid::Plus)</span>
                                    <span data-when="expanded">(fontawesome::solid::Minus)</span>
                                </AccordionItemIcon>
                            </AccordionItemTrigger>
                            <AccordionItemPanel>"Prevent the rotation and swap + for − when the item expands."</AccordionItemPanel>
                        </AccordionItem>
                    </Accordion>
                    <style>"
                        /* Customize the rotation when expanded */
                        .circle-plus.expanded .accordion-item-icon {
                            rotate: 225deg;
                        }

                        /* Prevent the default rotation animation… */
                        .plus-minus .accordion-item-icon {
                            rotate: none;
                        }

                        /* …and swap the icon based on the expanded state */
                        .plus-minus [data-when] {
                            display: inline-flex;
                        }

                        .plus-minus:not(.expanded) [data-when='expanded'],
                        .plus-minus.expanded [data-when='collapsed'] {
                            display: none;
                        }
                    "</style>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="html-in-the-label" anchor=true>
            "HTML in the Label"
        </Head>
        <p>"To place HTML in an accordion item's header, compose the item from "<code>"AccordionItemTrigger"</code>
            " and "<code>"AccordionItemPanel"</code>" instead of using the "<code>"label"</code>" property. This lets you "
            "add icons, badges, or other elements alongside the label text."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem>
                        <AccordionItemTrigger>
                            <div class=SPLIT>
                                <span>"Tasks"</span>
                                <Badge appearance=Filled variant=Success style="font-size: var(--wa-font-size-xs);">"3 ready"</Badge>
                            </div>
                        </AccordionItemTrigger>
                        <AccordionItemPanel>"All three tasks are ready to be reviewed."</AccordionItemPanel>
                    </AccordionItem>
                    <AccordionItem>
                        <AccordionItemTrigger>
                            <div class=SPLIT>
                                <span>"Issues"</span>
                                <Badge appearance=Filled variant=Danger style="font-size: var(--wa-font-size-xs);">"2 open"</Badge>
                            </div>
                        </AccordionItemTrigger>
                        <AccordionItemPanel>"There are two open issues that need your attention."</AccordionItemPanel>
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem>
                            <AccordionItemTrigger>
                                <div class=SPLIT>
                                    <span>"Tasks"</span>
                                    <Badge appearance=Filled variant=Success style="font-size: var(--wa-font-size-xs);">"3 ready"</Badge>
                                </div>
                            </AccordionItemTrigger>
                            <AccordionItemPanel>"All three tasks are ready to be reviewed."</AccordionItemPanel>
                        </AccordionItem>
                        <AccordionItem>
                            <AccordionItemTrigger>
                                <div class=SPLIT>
                                    <span>"Issues"</span>
                                    <Badge appearance=Filled variant=Danger style="font-size: var(--wa-font-size-xs);">"2 open"</Badge>
                                </div>
                            </AccordionItemTrigger>
                            <AccordionItemPanel>"There are two open issues that need your attention."</AccordionItemPanel>
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="expand-collapse-all" anchor=true>
            "Expand & Collapse All"
        </Head>
        <p>"Use the "<code>"expand_all"</code>" and "<code>"collapse_all"</code>" functions of "
            <code>"wingy_hypertext_web::layout::accordion"</code>" to control all items at once. Note that "
            <code>"expand_all"</code>" does nothing when the mode is "<code>"Single"</code>" or "
            <code>"SingleCollapsible"</code>"."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div>
                    <Accordion id="accordion-methods">
                        <AccordionItem label="Section one">"Content for the first section."</AccordionItem>
                        <AccordionItem label="Section two">"Content for the second section."</AccordionItem>
                        <AccordionItem label="Section three">"Content for the third section."</AccordionItem>
                    </Accordion>
                    <Divider/>
                    <div class=CLUSTER>
                        <Button appearance=Filled id="expand-all">"Expand All"</Button>
                        <Button appearance=Filled id="collapse-all">"Collapse All"</Button>
                    </div>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div>
                        <Accordion id="accordion-methods">
                            <AccordionItem label="Section one">"Content for the first section."</AccordionItem>
                            <AccordionItem label="Section two">"Content for the second section."</AccordionItem>
                            <AccordionItem label="Section three">"Content for the third section."</AccordionItem>
                        </Accordion>
                        <Divider/>
                        <div class=CLUSTER>
                            <Button appearance=Filled id="expand-all">"Expand All"</Button>
                            <Button appearance=Filled id="collapse-all">"Collapse All"</Button>
                        </div>
                    </div>

                    // Client side
                    let accordion = dom::existing::document().get_element_by_id("accordion-methods");
                    accordion::expand_all(&accordion);
                    accordion::collapse_all(&accordion);
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="nested-accordions" anchor=true>
            "Nested Accordions"
        </Head>
        <p>"Place an accordion inside an accordion item to nest one accordion inside another. Each accordion "
            "manages its own items independently, so toggling an inner item won't affect outer items, and properties "
            "like "<code>"mode"</code>" apply only to direct children."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion>
                    <AccordionItem label="Fruits" expanded=true>
                        <Accordion mode=Single>
                            <AccordionItem label="Apples">"Crisp, sweet, and great for pies."</AccordionItem>
                            <AccordionItem label="Oranges">"Juicy and packed with vitamin C."</AccordionItem>
                            <AccordionItem label="Bananas">"Soft, sweet, and easy to peel."</AccordionItem>
                        </Accordion>
                    </AccordionItem>
                    <AccordionItem label="Vegetables">
                        <Accordion mode=Single>
                            <AccordionItem label="Carrots">"Crunchy and rich in beta carotene."</AccordionItem>
                            <AccordionItem label="Broccoli">"A nutrient-dense cruciferous vegetable."</AccordionItem>
                        </Accordion>
                    </AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion>
                        <AccordionItem label="Fruits" expanded=true>
                            <Accordion mode=Single>
                                <AccordionItem label="Apples">"Crisp, sweet, and great for pies."</AccordionItem>
                                <AccordionItem label="Oranges">"Juicy and packed with vitamin C."</AccordionItem>
                                <AccordionItem label="Bananas">"Soft, sweet, and easy to peel."</AccordionItem>
                            </Accordion>
                        </AccordionItem>
                        <AccordionItem label="Vegetables">
                            <Accordion mode=Single>
                                <AccordionItem label="Carrots">"Crunchy and rich in beta carotene."</AccordionItem>
                                <AccordionItem label="Broccoli">"A nutrient-dense cruciferous vegetable."</AccordionItem>
                            </Accordion>
                        </AccordionItem>
                    </Accordion>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="preventing-expand-or-collapse" anchor=true>
            "Preventing Expand or Collapse"
        </Head>
        <p>"Listen for the "<code>"wg-expand"</code>" or "<code>"wg-collapse"</code>" events and call "
            <code>"event.preventDefault()"</code>" to stop the action from completing. The "<code>"event.detail.item"</code>
            " property tells you which accordion item triggered the event."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Accordion id="accordion-prevent">
                    <AccordionItem label="Locked open" expanded=true class="locked">
                        "This item is locked open — the "<code>"wg-collapse"</code>" event is being intercepted and prevented."
                    </AccordionItem>
                    <AccordionItem label="Works normally">"This item can be toggled normally."</AccordionItem>
                </Accordion>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Accordion id="accordion-prevent">
                        <AccordionItem label="Locked open" expanded=true class="locked">
                            "This item is locked open — the "<code>"wg-collapse"</code>" event is being intercepted and prevented."
                        </AccordionItem>
                        <AccordionItem label="Works normally">"This item can be toggled normally."</AccordionItem>
                    </Accordion>

                    // Cancel collapsing the locked item
                    document.add_steady_event_listener("wg-collapse", |event| { ... });
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}

//
// Interactive overview demo wiring
//

/// One-time wiring for the demos that need scripted behavior: the expand/collapse
/// all buttons and the locked item that refuses to collapse.
pub fn listen_accordion_overview() {
    let document = dom::existing::document();

    document.add_steady_event_listener("click", |event| {
        handle_expand_collapse_all(&event);
    });
    document.add_steady_event_listener(event::COLLAPSE, |event| {
        prevent_locked_collapse(&event);
    });
}

fn handle_expand_collapse_all(event: &Event) -> Option<()> {
    let target = event.target()?.maybe_into_element()?;
    let button = target.closest("#expand-all, #collapse-all").ok()??;
    let accordion = dom::existing::document().get_element_by_id("accordion-methods")?;

    if button.id() == "expand-all" {
        accordion::expand_all(&accordion);
    } else {
        accordion::collapse_all(&accordion);
    }

    Some(())
}

fn prevent_locked_collapse(event: &Event) -> Option<()> {
    let custom: &CustomEvent = event.dyn_ref()?;
    let item = custom.detail().get("item").dyn_into::<Element>().ok()?;

    if item.class_list().contains("locked") {
        event.prevent_default();
    }

    Some(())
}
