use hypertext::{RenderableExt, rsx};

use crate::layouts::page::{Page, PageAside, PageBody, PageFooter, PageHeader, PageMain, PageMenu};

#[test]
fn empty() {
    let page_markup = r#"
        <div class="page">
            <header class="page-header">
            </header>
            <div class="page-body">
                <div class="page-menu"></div>
                <main class="page-main"></main>
                <aside class="page-aside"></aside>
            </div>
            <footer class="page-footer"></footer>
        </div>
    "#
    .lines()
    .map(str::trim_start)
    .collect::<String>();

    let page = rsx! {
        <Page>
            <PageHeader>
            </PageHeader>
            <PageBody>
                <PageMenu>
                </PageMenu>
                <PageMain>
                </PageMain>
                <PageAside>
                </PageAside>
            </PageBody>
            <PageFooter>
            </PageFooter>
        </Page>
    };
    assert_eq!(page.render().as_inner(), &page_markup);
}
