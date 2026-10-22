use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::component::color_scheme_selector::ColorSchemeSelector;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Color Scheme Selector"</Head>
        <p>"The color scheme selector lets the user switch between the light and dark color schemes, or follow "
            "the system preference. It is an icon button opening a "<code>Dropdown</code>" with the "
            "available schemes, like the one in the top bar of this page."
        </p>
        <p>"The chosen scheme is kept in "<code>"localStorage"</code>" and the dark one is applied by toggling the "
            <code>"wa-dark"</code>" class on the document element. The behavior is implemented in Rust in "
            <code>"wingy-hypertext-web"</code>" and wired up with "<code>"listen_color_scheme_selector"</code>
            " and "<code>"init_color_scheme_selectors"</code>". All selectors on the page stay in sync."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ColorSchemeSelector />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"<ColorSchemeSelector />"#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="custom-labels" anchor=true>
            "Custom Labels"
        </Head>
        <p>"Use the "<code>"label_light"</code>", "<code>"label_dark"</code>" and "<code>"label_auto"</code>
            " attributes to change the labels of the menu items, e.g. to localize them."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <ColorSchemeSelector label_light="Светлая" label_dark="Тёмная" label_auto="Системная" />
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <ColorSchemeSelector label_light="Светлая" label_dark="Тёмная" label_auto="Системная" />
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
