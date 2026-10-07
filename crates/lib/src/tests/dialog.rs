use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{RenderableExt, rsx};
use iconic::fontawesome;

use crate::attributes::CommonAttributeSetters;
use crate::layout::INVISIBLE;
use crate::layout::dialog::{Dialog, DialogBody, DialogFooter, DialogHeader, DialogHeaderActions, DialogTitle};

/// The close button icon is rendered from the `iconic` crate; build the
/// expected markup dynamically so the tests don't hardcode the SVG.
fn header_actions(actions: &str) -> String {
    let icon = rsx! { (fontawesome::solid::Xmark) }.render().into_inner();
    format!(
        concat!(
            r#"<div class="dialog-header-actions">{actions}"#,
            r#"<button class="button neutral plain dialog-close" data-dialog="close" aria-label="Close">"#,
            r#"<span class="icon">{icon}</span></button></div>"#,
        ),
        actions = actions,
        icon = icon,
    )
}

fn header(title: &str) -> String {
    format!(
        r#"<div class="dialog-header"><h2 class="dialog-title">{title}</h2>{actions}</div>"#,
        title = title,
        actions = header_actions(""),
    )
}

#[test]
fn default() {
    let expected = r#"<dialog class="dialog"></dialog>"#;

    let dialog = Dialog::builder();
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog/> };
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn flags() {
    let expected = r#"<dialog class="dialog" data-open="" data-light-dismiss=""></dialog>"#;

    let dialog = Dialog::builder().open(true).light_dismiss(true);
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog open=true light_dismiss=true/> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn title() {
    let expected = format!(r#"<dialog class="dialog">{}</dialog>"#, header("Dialog"));

    let dialog_header = DialogHeader::builder().children(&"Dialog");
    let dialog = Dialog::builder().children(&dialog_header);
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog><DialogHeader>"Dialog"</DialogHeader></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn title_markup() {
    let expected = format!(r#"<dialog class="dialog">{}</dialog>"#, header("<em>Dialog</em>"));

    let dialog = rsx! { <Dialog><DialogHeader><em>"Dialog"</em></DialogHeader></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn empty_title() {
    let expected = format!(r#"<dialog class="dialog">{}</dialog>"#, header(INVISIBLE));

    let dialog = rsx! { <Dialog><DialogHeader/></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog><DialogHeader bare=true><DialogTitle/><DialogHeaderActions/></DialogHeader></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn header_actions_bare() {
    let expected = format!(
        r#"<dialog class="dialog"><div class="dialog-header"><h2 class="dialog-title">Dialog</h2>{}</div></dialog>"#,
        header_actions(r#"<span class="new-window"></span>"#),
    );

    let dialog = rsx! {
        <Dialog>
            <DialogHeader bare=true>
                <DialogTitle>"Dialog"</DialogTitle>
                <DialogHeaderActions>
                    <span class="new-window"></span>
                </DialogHeaderActions>
            </DialogHeader>
        </Dialog>
    };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn body() {
    let expected = r#"<dialog class="dialog"><div class="dialog-body">Hello, world!</div></dialog>"#;

    let dialog_body = DialogBody::builder().children(&"Hello, world!");
    let dialog = Dialog::builder().children(&dialog_body);
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog><DialogBody>"Hello, world!"</DialogBody></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn without_header() {
    let expected = concat!(
        r#"<dialog class="dialog"><div class="dialog-body">Body</div>"#,
        r#"<div class="dialog-footer"><button>Close</button></div></dialog>"#,
    );

    let dialog = rsx! {
        <Dialog>
            <DialogBody>"Body"</DialogBody>
            <DialogFooter><button>"Close"</button></DialogFooter>
        </Dialog>
    };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn header_body_footer() {
    let expected = format!(
        concat!(
            r#"<dialog class="dialog">{}<div class="dialog-body">Body</div>"#,
            r#"<div class="dialog-footer"><button>Close</button></div></dialog>"#,
        ),
        header("Dialog"),
    );

    let dialog = rsx! {
        <Dialog>
            <DialogHeader>"Dialog"</DialogHeader>
            <DialogBody>"Body"</DialogBody>
            <DialogFooter><button>"Close"</button></DialogFooter>
        </Dialog>
    };
    assert_eq!(dialog.render().as_inner(), &expected);
}

#[test]
fn additional_attributes() {
    let expected = r#"<dialog id="the-dialog" class="dialog test" style="--width: 50vw"></dialog>"#;

    let dialog = Dialog::builder().id("the-dialog").class("test").style("--width: 50vw");
    assert_eq!(dialog.render().as_inner(), &expected);

    let dialog = rsx! { <Dialog id="the-dialog" class="test" style="--width: 50vw"/> };
    assert_eq!(dialog.render().as_inner(), &expected);

    let expected = r#"<dialog class="dialog"><h2 id="the-title" class="dialog-title test">Dialog</h2></dialog>"#;
    let dialog = rsx! { <Dialog><DialogTitle id="the-title" class="test">"Dialog"</DialogTitle></Dialog> };
    assert_eq!(dialog.render().as_inner(), &expected);
}
