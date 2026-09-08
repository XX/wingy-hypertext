use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome;

use crate::layout::INVISIBLE;
use crate::layout::page::{Page, PageBody, PageMenu, PageNavigationToggle};

#[test]
fn empty() {
    let page_markup = r#"
        <div class="page" view="desktop">
            <style>@container page (width < 920px){.page{--menu-width:auto;--aside-width:auto}.page .wa-desktop-only{display:none!important}.page .wa-mobile-only{display:revert!important}.page [data-toggle-nav]{display:revert}}</style>
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

#[test]
fn navigation() {
    let page = rsx! {
        <Page navigation=true mobile_breakpoint="60rem">
            <header><PageNavigationToggle/></header>
        </Page>
    };

    let bars = fontawesome::solid::Bars.render().into_inner();
    let xmark = fontawesome::solid::Xmark.render().into_inner();
    let expected = format!(
        concat!(
            r#"<div class="page" view="desktop" data-mobile-breakpoint="60rem">"#,
            r#"<style>@container page (width < 60rem){{.page{{--menu-width:0;--aside-width:auto}}"#,
            r#".page .wa-desktop-only{{display:none!important}}.page .wa-mobile-only{{display:revert!important}}"#,
            r#".page [data-toggle-nav]{{display:revert}}"#,
            r#".page .page-menu{{display:none}}.page .page-navigation-toggle{{display:inline-flex}}}}</style>"#,
            r#"<header>"#,
            r#"<button class="page-navigation-toggle" type="button" data-toggle-nav="""#,
            r#" data-drawer="open page-navigation" aria-label="Open navigation""#,
            r#" aria-controls="page-navigation"><span class="icon">{bars}</span></button></header>"#,
            r#"<dialog id="page-navigation" class="drawer start page-navigation-drawer""#,
            r#" data-light-dismiss=""><div class="drawer-header">"#,
            r#"<h2 class="drawer-title">{invisible}</h2><div class="drawer-header-actions">"#,
            r#"<button class="button neutral plain drawer-close" data-drawer="close" aria-label="Close">"#,
            r#"<span class="icon">{xmark}</span></button></div></div>"#,
            r#"<div class="drawer-body"></div></dialog></div>"#,
        ),
        bars = bars,
        xmark = xmark,
        invisible = INVISIBLE,
    );
    assert_eq!(page.render().as_inner(), &expected);
}

/// A breakpoint reaches a `@media` prelude verbatim, so anything that is not a
/// CSS length has to be refused rather than escaped.
#[test]
fn breakpoint_injection_is_refused() {
    let hostile = rsx! { <Page mobile_breakpoint="1px){} body{display:none}@media(width>0px"/> };
    let rendered = hostile.render();
    let rendered = rendered.as_inner();

    let style = rendered
        .split_once("<style>")
        .and_then(|(_, rest)| rest.split_once("</style>"))
        .map(|(style, _)| style)
        .expect("the page renders a stylesheet");

    assert_eq!(
        style,
        "@container page (width < 920px){.page{--menu-width:auto;--aside-width:auto}\
         .page .wa-desktop-only{display:none!important}.page .wa-mobile-only{display:revert!important}\
         .page [data-toggle-nav]{display:revert}}",
        "the hostile breakpoint reached the stylesheet"
    );
    // The attribute drives the client-side breakpoint, so it is refused as well.
    assert!(
        rendered.contains(r#"data-mobile-breakpoint="920px""#),
        "the attribute kept the hostile value: {rendered}"
    );

    // Units and decimals are legitimate and must survive.
    let ok = rsx! { <Page mobile_breakpoint="48.5em"/> };
    assert!(ok.render().as_inner().contains("(width < 48.5em)"));
}
