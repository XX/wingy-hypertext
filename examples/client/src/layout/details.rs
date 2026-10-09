use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{DETAILS_COLLAPSE_ICON, DETAILS_EXPAND_ICON, DETAILS_ICON, DETAILS_SUMMARY, STACK};
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::icon_placement::ExpandIconPlacement::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::layout::details::{Details, DetailsBody, DetailsHeader};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Details"</Head>
        <p>"Details display a brief summary and expand to reveal additional content. Use them to progressively "
            "disclose information, group related FAQs, or hide advanced options. It is a native "
            <code>"<details>"</code>", so it opens, closes and groups by name even before the client code runs; "
            "the client adds the animation and the arrow keys."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Details summary="Toggle Me">
                    "Click the summary to expand and collapse the details component. You can put any content in "
                    "here that you want to reveal on demand!"
                </Details>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Details summary="Toggle Me">
                        "Click the summary to expand and collapse the details component. You can put any content in "
                        "here that you want to reveal on demand!"
                    </Details>
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
        <p>"Use the "<code>open</code>" property to expand the details initially."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Details summary="Toggle Me" open=true>
                    "This details component is expanded by default. Users can click the summary to collapse it if "
                    "they want to hide the content."
                </Details>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Details summary="Toggle Me" open=true>
                        "This details component is expanded by default. Users can click the summary to collapse it if "
                        "they want to hide the content."
                    </Details>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="disabled" anchor=true>
            "Disabled"
        </Head>
        <p>"Use the "<code>disabled</code>" property to prevent the details from expanding."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Details summary="Disabled" disabled=true>
                    "This content can't be seen because the details component is disabled. Try removing the "
                    "disabled property to reveal what's inside!"
                </Details>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Details summary="Disabled" disabled=true>
                        "This content can't be seen because the details component is disabled. Try removing the "
                        "disabled property to reveal what's inside!"
                    </Details>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="expand-collapse-icons" anchor=true>
            "Expand & Collapse Icons"
        </Head>
        <p>"Use the "<code>expand_icon</code>" and "<code>collapse_icon</code>
            " properties to change the expand and collapse icons, respectively. To disable the animation, "
            "override the "<code>rotate</code>" property of the "<code>".details-icon"</code>" element as shown below."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Details class="custom-icons">
                    <DetailsHeader bare=true>
                        <span class=DETAILS_SUMMARY>"Toggle Me"</span>
                        <span class=DETAILS_ICON>
                            <span class=DETAILS_EXPAND_ICON>
                                (fontawesome::solid::Plus)
                            </span>
                            <span class=DETAILS_COLLAPSE_ICON>
                                (fontawesome::solid::Minus)
                            </span>
                        </span>
                    </DetailsHeader>
                    <DetailsBody>
                        "This example uses custom plus and minus icons for expanding and collapsing. You can use any "
                        "icon you want to match the look and feel of your app."
                    </DetailsBody>
                </Details>
                <style>"
                    /* Disable the expand/collapse animation */
                    .custom-icons .details-icon {
                        rotate: none;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Details class="custom-icons">
                        <DetailsHeader bare=true>
                            <span class=DETAILS_SUMMARY>"Toggle Me"</span>
                            <span class=DETAILS_ICON>
                                <span class=DETAILS_EXPAND_ICON>
                                    (fontawesome::solid::Plus)
                                </span>
                                <span class=DETAILS_COLLAPSE_ICON>
                                    (fontawesome::solid::Minus)
                                </span>
                            </span>
                        </DetailsHeader>
                        <DetailsBody>
                            "This example uses custom plus and minus icons for expanding and collapsing. You can use any "
                            "icon you want to match the look and feel of your app."
                        </DetailsBody>
                    </Details>
                    <style>"
                        /* Disable the expand/collapse animation */
                        .custom-icons .details-icon {
                            rotate: none;
                        }
                    "</style>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="icon-placement" anchor=true>
            "Icon Placement"
        </Head>
        <p>"The default position for the expand and collapse icons is at the end of the summary. Set the "
            <code>icon_placement</code>" property to "<code>Start</code>
            " to place the icon at the start of the summary."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Details summary="Start" icon_placement=Start>
                        "The expand/collapse icon is at the start of the summary. This is a common pattern that "
                        "feels familiar to users who are used to tree views and file explorers."
                    </Details>
                    <Details summary="End" icon_placement=End>
                        "The expand/collapse icon is at the end of the summary. This is the default placement and "
                        "works great for most use cases."
                    </Details>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Details summary="Start" icon_placement=Start>
                            "The expand/collapse icon is at the start of the summary. This is a common pattern that "
                            "feels familiar to users who are used to tree views and file explorers."
                        </Details>
                        <Details summary="End" icon_placement=End>
                            "The expand/collapse icon is at the end of the summary. This is the default placement and "
                            "works great for most use cases."
                        </Details>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="html-in-summary" anchor=true>
            "HTML in Summary"
        </Head>
        <p>"The "<code>summary</code>" property takes markup as well as a string. Links and other interactive "
            "elements will still retain their behavior:"
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Details>
                    <DetailsHeader>
                        <span>
                            "Some text "
                            <a href="https://webawesome.com" target="_blank">"a link"</a>
                            " more text"
                        </span>
                    </DetailsHeader>
                    <DetailsBody>
                        "You can put HTML in the summary, including links and other interactive elements. Pretty neat, "
                        "right?"
                    </DetailsBody>
                </Details>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Details>
                        <DetailsHeader>
                            <span>
                                "Some text "
                                <a href="https://webawesome.com" target="_blank">"a link"</a>
                                " more text"
                            </span>
                        </DetailsHeader>
                        <DetailsBody>
                            "You can put HTML in the summary, including links and other interactive elements. Pretty neat, "
                            "right?"
                        </DetailsBody>
                    </Details>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="rtl" anchor=true>
            "Right-to-Left Languages"
        </Head>
        <p>"The details component, including its "<code>icon_placement</code>
            ", automatically adapts to right-to-left languages:"
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Details summary="تبديلني" attrs=(attrs!["lang" = &"ar", "dir" = &"rtl"])>
                        "استخدام طريقة لوريم إيبسوم لأنها تعطي توزيعاَ طبيعياَ -إلى حد ما- للأحرف عوضاً عن"
                    </Details>
                    <Details summary="تبديلني" icon_placement=Start attrs=(attrs!["lang" = &"ar", "dir" = &"rtl"])>
                        "استخدام طريقة لوريم إيبسوم لأنها تعطي توزيعاَ طبيعياَ -إلى حد ما- للأحرف عوضاً عن"
                    </Details>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Details summary="تبديلني" attrs=(attrs!["lang" = &"ar", "dir" = &"rtl"])>
                            "استخدام طريقة لوريم إيبسوم لأنها تعطي توزيعاَ طبيعياَ -إلى حد ما- للأحرف عوضاً عن"
                        </Details>
                        <Details summary="تبديلني" icon_placement=Start attrs=(attrs!["lang" = &"ar", "dir" = &"rtl"])>
                            "استخدام طريقة لوريم إيبسوم لأنها تعطي توزيعاَ طبيعياَ -إلى حد ما- للأحرف عوضاً عن"
                        </Details>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the element’s visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Details summary="Outlined (default)">
                        "This is the default outlined appearance. It has a subtle border that helps it stand out "
                        "without being too flashy."
                    </Details>
                    <Details summary="Filled-outlined" appearance=FilledOutlined>
                        "The filled-outlined appearance combines a filled header with an outlined body. It gives "
                        "the summary a bit more visual weight while keeping the content area clean."
                    </Details>
                    <Details summary="Filled" appearance=Filled>
                        "The filled appearance adds a background color to the entire component. Use this when you "
                        "want the details to really pop on the page."
                    </Details>
                    <Details summary="Plain" appearance=Plain>
                        "No bells and whistles on this one. The plain appearance strips away borders and backgrounds "
                        "for a minimalist look."
                    </Details>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Details summary="Outlined (default)">
                            "This is the default outlined appearance. It has a subtle border that helps it stand out "
                            "without being too flashy."
                        </Details>
                        <Details summary="Filled-outlined" appearance=FilledOutlined>
                            "The filled-outlined appearance combines a filled header with an outlined body. It gives "
                            "the summary a bit more visual weight while keeping the content area clean."
                        </Details>
                        <Details summary="Filled" appearance=Filled>
                            "The filled appearance adds a background color to the entire component. Use this when you "
                            "want the details to really pop on the page."
                        </Details>
                        <Details summary="Plain" appearance=Plain>
                            "No bells and whistles on this one. The plain appearance strips away borders and backgrounds "
                            "for a minimalist look."
                        </Details>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="grouping" anchor=true>
            "Grouping Details"
        </Head>
        <p>"Use the "<code>name</code>" property to create accordion-like behavior where only one details "
            "element with the same name can be open at a time. This matches the behavior of native "
            <code>"<details>"</code>" elements."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=STACK>
                    <Details name="group-1" summary="Section 1" open=true>
                        "This is the first section of the accordion. When you open another section, this one will "
                        "close automatically. Give it a try!"
                    </Details>
                    <Details name="group-1" summary="Section 2">
                        "This is the second section. Notice how the first section closed when you opened this one? "
                        "That's the accordion behavior in action, powered by the shared name property."
                    </Details>
                    <Details name="group-1" summary="Section 3">
                        "And here's the third section. You can have as many sections as you need — just make sure "
                        "they all share the same name and only one will be open at a time."
                    </Details>
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <div class=STACK>
                        <Details name="group-1" summary="Section 1" open=true>
                            "This is the first section of the accordion. When you open another section, this one will "
                            "close automatically. Give it a try!"
                        </Details>
                        <Details name="group-1" summary="Section 2">
                            "This is the second section. Notice how the first section closed when you opened this one? "
                            "That's the accordion behavior in action, powered by the shared name property."
                        </Details>
                        <Details name="group-1" summary="Section 3">
                            "And here's the third section. You can have as many sections as you need — just make sure "
                            "they all share the same name and only one will be open at a time."
                        </Details>
                    </div>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
