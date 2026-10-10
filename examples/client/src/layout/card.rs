use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome;
use indoc::formatdoc;
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{CAPTION_S, FRAME_LANDSCAPE, GRID, ICON};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::head::Head;
use wingy_hypertext::component::head::HeadLevel::*;
use wingy_hypertext::component::rating::Rating;
use wingy_hypertext::layout::card::{
    Card, CardActions, CardBody, CardFooter, CardFooterActions, CardHeader, CardHeaderActions, CardMedia,
};
use wingy_hypertext::layout::code_example::{CodeExample, CodeExampleButton, CodeExamplePreview, CodeExampleSource};
use wingy_hypertext::orientation::Orientation::*;
use wingy_hypertext::variant::Variant::*;

const KITTEN: &str = "https://images.unsplash.com/photo-1559209172-0ff8f6d49ff7?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=500&q=80";
const KITTEN_ALT: &str = "A kitten sits patiently between a terracotta pot and decorative grasses.";
const KITTEN_WALKING: &str = "https://images.unsplash.com/photo-1547191783-94d5f8f6d8b1?ixlib=rb-1.2.1&ixid=eyJhcHBfaWQiOjEyMDd9&auto=format&fit=crop&w=400&q=80";
const DOG_VIDEO: &str = "https://uploads.webawesome.com/dog-with-glasses.mp4";

pub fn overview() -> impl Renderable {
    rsx! {
        <Head level=H1>"Card"</Head>
        <p>"Cards group related content and actions inside a bordered container. Use them to present products, "
            "articles, user profiles, or any self-contained unit of information."
        </p>
        <p>"The card's sections are subcomponents: "<code>"CardMedia"</code>", "<code>"CardHeader"</code>", "
            <code>"CardBody"</code>", "<code>"CardFooter"</code>" and, for a horizontal card, "
            <code>"CardActions"</code>". Unused sections are simply left out, the styles follow the ones present."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Card class="card-overview">
                    <CardMedia><img src=KITTEN alt=KITTEN_ALT></CardMedia>
                    <CardBody>
                        <strong>"Mittens"</strong><br>
                        "This kitten is as cute as he is playful. Bring him home today!"<br>
                        <small class=CAPTION_S>"6 weeks old"</small>
                    </CardBody>
                    <CardFooter>
                        <Button variant=Brand pill=true>"More Info"</Button>
                        <CardFooterActions><Rating label="Rating"/></CardFooterActions>
                    </CardFooter>
                </Card>
                <style>"
                    .card-overview {
                        width: 300px;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">(formatdoc!(r#"
                    <Card class="card-overview">
                        <CardMedia><img src="{KITTEN}" alt="{KITTEN_ALT}"></CardMedia>
                        <CardBody>
                            <strong>"Mittens"</strong><br>
                            "This kitten is as cute as he is playful. Bring him home today!"<br>
                            <small class=CAPTION_S>"6 weeks old"</small>
                        </CardBody>
                        <CardFooter>
                            <Button variant=Brand pill=true>"More Info"</Button>
                            <CardFooterActions><Rating label="Rating"/></CardFooterActions>
                        </CardFooter>
                    </Card>
                    <style>"
                        .card-overview {{
                            width: 300px;
                        }}
                    "</style>
                "#))</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H2 id="examples" anchor=true>
            "Examples"
        </Head>

        <Head level=H3 id="basic-card" anchor=true>
            "Basic Card"
        </Head>
        <p>"A card can hold any content. Media, a header, and a footer are all optional."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Card class="card-basic">
                    <CardBody>
                        "This is just a basic card. No media, no header, and no footer. Just your content."
                    </CardBody>
                </Card>
                <style>"
                    .card-basic {
                        max-width: 300px;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Card class="card-basic">
                        <CardBody>
                            "This is just a basic card. No media, no header, and no footer. Just your content."
                        </CardBody>
                    </Card>
                    <style>"
                        .card-basic {
                            max-width: 300px;
                        }
                    "</style>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="header" anchor=true>
            "Header"
        </Head>
        <p>"Headers can be used to display titles and more. Put a "<code>"CardHeaderActions"</code>
            " last to push the actions to the end of the header."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Card class="card-header-demo">
                    <CardHeader>
                        <h3>"Header Title"</h3>
                        <CardHeaderActions>
                            <Button appearance=Plain attrs=(attrs!["aria-label" = &"Settings"])>
                                <span class=ICON>
                                    (fontawesome::solid::Gear)
                                </span>
                            </Button>
                        </CardHeaderActions>
                    </CardHeader>
                    <CardBody>"This card has a header. You can put all sorts of things in it!"</CardBody>
                </Card>
                <style>"
                    .card-header-demo {
                        max-width: 300px;
                    }

                    .card-header-demo h3 {
                        margin: 0;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Card class="card-header-demo">
                        <CardHeader>
                            <h3>"Header Title"</h3>
                            <CardHeaderActions>
                                <Button appearance=Plain attrs=(attrs!["aria-label" = &"Settings"])>
                                    <span class=ICON>
                                        (fontawesome::solid::Gear)
                                    </span>
                                </Button>
                            </CardHeaderActions>
                        </CardHeader>
                        <CardBody>"This card has a header. You can put all sorts of things in it!"</CardBody>
                    </Card>
                    <style>"
                        .card-header-demo {
                            max-width: 300px;
                        }

                        .card-header-demo h3 {
                            margin: 0;
                        }
                    "</style>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="footer" anchor=true>
            "Footer"
        </Head>
        <p>"Footers can be used to display actions, summaries, or other relevant content. Put a "
            <code>"CardFooterActions"</code>" last to push the actions to the end of the footer."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <Card class="card-footer-demo">
                    <CardBody>"This card has a footer. You can put all sorts of things in it!"</CardBody>
                    <CardFooter>
                        <Rating label="Rating"/>
                        <CardFooterActions>
                            <Button variant=Brand>"Preview"</Button>
                        </CardFooterActions>
                    </CardFooter>
                </Card>
                <style>"
                    .card-footer-demo {
                        max-width: 300px;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">r#"
                    <Card class="card-footer-demo">
                        <CardBody>"This card has a footer. You can put all sorts of things in it!"</CardBody>
                        <CardFooter>
                            <Rating label="Rating"/>
                            <CardFooterActions>
                                <Button variant=Brand>"Preview"</Button>
                            </CardFooterActions>
                        </CardFooter>
                    </Card>
                    <style>"
                        .card-footer-demo {
                            max-width: 300px;
                        }
                    "</style>
                "#</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="media" anchor=true>
            "Media"
        </Head>
        <p>"Card media is displayed atop the card and will stretch to fit."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=GRID>
                    <Card class="card-media-demo">
                        <CardMedia class=FRAME_LANDSCAPE>
                            <img src=KITTEN_WALKING alt="A kitten walks towards camera on top of pallet.">
                        </CardMedia>
                        <CardBody>"This card has an image of a kitten walking along a pallet."</CardBody>
                    </Card>
                    <Card class="card-media-demo">
                        <CardMedia>
                            <video controls>
                                <source src=DOG_VIDEO>
                                <p>"Your browser doesn't support HTML video"</p>
                            </video>
                        </CardMedia>
                        <CardBody>"This card has a video of a dog wearing shades."</CardBody>
                    </Card>
                </div>
                <style>"
                    .card-media-demo {
                        max-width: 300px;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">(formatdoc!(r#"
                    <div class=GRID>
                        <Card class="card-media-demo">
                            <CardMedia class=FRAME_LANDSCAPE>
                                <img src="{KITTEN_WALKING}" alt="A kitten walks towards camera on top of pallet.">
                            </CardMedia>
                            <CardBody>"This card has an image of a kitten walking along a pallet."</CardBody>
                        </Card>
                        <Card class="card-media-demo">
                            <CardMedia>
                                <video controls>
                                    <source src="{DOG_VIDEO}">
                                    <p>"Your browser doesn't support HTML video"</p>
                                </video>
                            </CardMedia>
                            <CardBody>"This card has a video of a dog wearing shades."</CardBody>
                        </Card>
                    </div>
                    <style>"
                        .card-media-demo {{
                            max-width: 300px;
                        }}
                    "</style>
                "#))</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="appearance" anchor=true>
            "Appearance"
        </Head>
        <p>"Use the "<code>appearance</code>" property to change the card's visual appearance."</p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=GRID>
                    <Card>
                        <CardMedia><img src=KITTEN alt=KITTEN_ALT></CardMedia>
                        <CardBody>"Outlined (default)"</CardBody>
                    </Card>
                    @for (appearance, name) in [
                        (FilledOutlined, "Filled-outlined"),
                        (Plain, "Plain"),
                        (Filled, "Filled"),
                        (Accent, "Accent"),
                    ] {
                        <Card appearance=(appearance)>
                            <CardMedia><img src=KITTEN alt=KITTEN_ALT></CardMedia>
                            <CardBody>(name)</CardBody>
                        </Card>
                    }
                </div>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">(formatdoc!(r#"
                    <div class=GRID>
                        <Card>
                            <CardMedia><img src="{KITTEN}" alt="{KITTEN_ALT}"></CardMedia>
                            <CardBody>"Outlined (default)"</CardBody>
                        </Card>
                        @for (appearance, name) in [
                            (FilledOutlined, "Filled-outlined"),
                            (Plain, "Plain"),
                            (Filled, "Filled"),
                            (Accent, "Accent"),
                        ] {{
                            <Card appearance=(appearance)>
                                <CardMedia><img src="{KITTEN}" alt="{KITTEN_ALT}"></CardMedia>
                                <CardBody>(name)</CardBody>
                            </Card>
                        }}
                    </div>
                "#))</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>

        <Head level=H3 id="orientation" anchor=true>
            "Orientation"
        </Head>
        <p>"Set the "<code>orientation</code>" property to "<code>Horizontal</code>" to create a card with a "
            "horizontal, side-by-side layout of "<code>"CardMedia"</code>", "<code>"CardBody"</code>" and "
            <code>"CardActions"</code>". Make sure to set a width or maximum width for the media. Horizontal "
            "cards are not meant to contain a header and a footer."
        </p>
        <CodeExample>
            <CodeExamplePreview resize=true>
                <div class=GRID>
                    <Card orientation=Horizontal class="horizontal-card">
                        <CardMedia><img src=KITTEN alt=KITTEN_ALT></CardMedia>
                        <CardBody>
                            "This card has a horizontal orientation with media, body, and actions arranged "
                            "side-by-side."
                        </CardBody>
                        <CardActions>
                            <Button variant=Neutral appearance=Plain attrs=(attrs!["aria-label" = &"Actions"])>
                                <span class=ICON>
                                    (fontawesome::solid::Ellipsis)
                                </span>
                            </Button>
                        </CardActions>
                    </Card>
                </div>
                <style>"
                    .horizontal-card .card-media img {
                        max-width: 300px;
                    }
                "</style>
            </CodeExamplePreview>
            <CodeExampleSource copy_button=true>
                <code class="language-html">(formatdoc!(r#"
                    <div class=GRID>
                        <Card orientation=Horizontal class="horizontal-card">
                            <CardMedia><img src="{KITTEN}" alt="{KITTEN_ALT}"></CardMedia>
                            <CardBody>
                                "This card has a horizontal orientation with media, body, and actions arranged "
                                "side-by-side."
                            </CardBody>
                            <CardActions>
                                <Button variant=Neutral appearance=Plain attrs=(attrs!["aria-label" = &"Actions"])>
                                    <span class=ICON>
                                        (fontawesome::solid::Ellipsis)
                                    </span>
                                </Button>
                            </CardActions>
                        </Card>
                    </div>
                    <style>"
                        .horizontal-card .card-media img {{
                            max-width: 300px;
                        }}
                    "</style>
                "#))</code>
            </CodeExampleSource>
            <CodeExampleButton color_scheme=true direction=true>"Code"</CodeExampleButton>
        </CodeExample>
    }
}
