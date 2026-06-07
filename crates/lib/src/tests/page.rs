use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};

use crate::layouts::page::{Page, PageBody, PageMenu};

#[test]
fn empty() {
    let page_markup = r#"
        <div class="page">
            <header></header>
            <div class="page-body">
                <div class="page-menu"></div>
                <main></main>
                <aside></aside>
            </div>
            <footer></footer>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let page = rsx! {
        <Page>
            <header></header>
            <PageBody>
                <PageMenu>
                </PageMenu>
                <main></main>
                <aside></aside>
            </PageBody>
            <footer></footer>
        </Page>
    };
    assert_eq!(page.render().as_inner(), &page_markup);
}
