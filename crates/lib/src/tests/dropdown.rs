use hypertext::prelude::hypertext_elements;
use hypertext::{RenderableExt, rsx};
use iconic::{fontawesome, fontawesome_ext};

use crate::attributes::CommonAttributeSetters;
use crate::component::dropdown::DropdownSize::Small;
use crate::component::dropdown::{Dropdown, DropdownItem};
use crate::helper::popup::PopupPlacement::TopEnd;
use crate::variant::Variant::Danger;

/// The positioning options every dropdown menu is rendered with, after the
/// placement and the optional distance and skidding.
const POPUP_OPTIONS: &str =
    r#"data-flip="" data-shift="" data-shift-padding="10" data-auto-size="vertical" data-auto-size-padding="10""#;

/// The dropdown host (`host_attrs`) wrapping the popup that carries the trigger
/// and the menu. The menu itself is always rendered, even without items.
fn dropdown(host_attrs: &str, popup_attrs: &str, content: &str) -> String {
    format!(
        concat!(
            r#"<div {host_attrs}>"#,
            r#"<div class="popup" {popup_attrs} {options}>{content}</div>"#,
            r#"</div>"#,
        ),
        host_attrs = host_attrs,
        popup_attrs = popup_attrs,
        options = POPUP_OPTIONS,
        content = content,
    )
}

fn menu(items: &str) -> String {
    format!(
        concat!(
            r#"<div class="popup-body">"#,
            r#"<div class="dropdown-menu" role="menu" tabindex="-1" aria-orientation="vertical" hidden>{items}</div>"#,
            r#"</div>"#,
        ),
        items = items,
    )
}

/// The checkmark and the submenu indicator are rendered from the `iconic`
/// crate; build them dynamically so the tests don't hardcode the SVG.
fn icon(icon: impl hypertext::Renderable) -> String {
    rsx! { (icon) }.render().into_inner()
}

#[test]
fn default() {
    let expected = dropdown(r#"class="dropdown""#, r#"data-placement="bottom-start""#, &menu(""));

    let dropdown = Dropdown::builder();
    assert_eq!(dropdown.render().as_inner(), &expected);

    let dropdown = rsx! { <Dropdown/> };
    assert_eq!(dropdown.render().as_inner(), &expected);

    let dropdown = rsx! { <Dropdown></Dropdown> };
    assert_eq!(dropdown.render().as_inner(), &expected);
}

#[test]
fn trigger_and_items() {
    let items = concat!(
        r#"<div class="dropdown-item neutral" role="menuitem" tabindex="-1" data-value="edit">"#,
        r#"<span class="dropdown-item-label">Edit</span></div>"#,
        r#"<div class="dropdown-item neutral" role="menuitem" tabindex="-1" data-value="delete">"#,
        r#"<span class="dropdown-item-label">Delete</span></div>"#,
    );
    let expected = dropdown(
        r#"class="dropdown""#,
        r#"data-placement="bottom-start""#,
        &format!(r#"<button>Options</button>{}"#, menu(items)),
    );

    let dropdown = rsx! {
        <Dropdown trigger=(rsx! { <button>"Options"</button> })>
            <DropdownItem value="edit">"Edit"</DropdownItem>
            <DropdownItem value="delete">"Delete"</DropdownItem>
        </Dropdown>
    };
    assert_eq!(dropdown.render().as_inner(), &expected);
}

#[test]
fn positioning() {
    let expected = dropdown(
        r#"class="dropdown""#,
        r#"data-placement="top-end" data-distance="30" data-skidding="-10""#,
        &menu(""),
    );

    let dropdown = Dropdown::builder().placement(TopEnd).distance(30).skidding(-10);
    assert_eq!(dropdown.render().as_inner(), &expected);

    let dropdown = rsx! { <Dropdown placement=TopEnd distance=30 skidding=-10/> };
    assert_eq!(dropdown.render().as_inner(), &expected);
}

#[test]
fn size_and_open() {
    let expected = dropdown(
        r#"class="dropdown size-small" data-open="""#,
        r#"data-placement="bottom-start""#,
        &menu(""),
    );

    let dropdown = Dropdown::builder().size(Small).open(true);
    assert_eq!(dropdown.render().as_inner(), &expected);

    let dropdown = rsx! { <Dropdown size=Small open=true/> };
    assert_eq!(dropdown.render().as_inner(), &expected);
}

#[test]
fn item_states() {
    let check = icon(fontawesome::solid::Check);
    let expected = format!(
        concat!(
            r#"<div class="dropdown-item checked neutral" role="menuitemcheckbox" tabindex="-1""#,
            r#" aria-checked="true" data-value="grid" data-label="Show grid">"#,
            r#"<span class="check" aria-hidden="true">{check}</span>"#,
            r#"<span class="dropdown-item-label">Show grid</span></div>"#,
        ),
        check = check,
    );

    let item = DropdownItem::builder()
        .checkbox(true)
        .checked(true)
        .value("grid")
        .label("Show grid")
        .children(&"Show grid");
    assert_eq!(item.render().as_inner(), &expected);

    let item = rsx! {
        <DropdownItem checkbox=true checked=true value="grid" label="Show grid">"Show grid"</DropdownItem>
    };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn item_disabled_and_danger() {
    let expected = concat!(
        r#"<div class="dropdown-item disabled danger" role="menuitem" tabindex="-1" aria-disabled="true""#,
        r#" data-value="delete"><span class="dropdown-item-label">Delete</span></div>"#,
    );

    let item = rsx! { <DropdownItem variant=Danger disabled=true value="delete">"Delete"</DropdownItem> };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn item_icon_and_details() {
    let house = icon(fontawesome::solid::House);
    let expected = format!(
        concat!(
            r#"<div class="dropdown-item neutral" role="menuitem" tabindex="-1" data-value="home">"#,
            r#"<span class="dropdown-item-icon">{house}</span>"#,
            r#"<span class="dropdown-item-label">Home</span>"#,
            r#"<span class="dropdown-item-details">⌘H</span></div>"#,
        ),
        house = house,
    );

    let item = rsx! {
        <DropdownItem
            value="home"
            icon=(rsx! { (fontawesome::solid::House) })
            details=(rsx! { "⌘H" })
        >
            "Home"
        </DropdownItem>
    };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn item_submenu() {
    let chevron = icon(fontawesome_ext::regular::ChevronRight);
    let expected = format!(
        concat!(
            r#"<div class="dropdown-item has-submenu neutral" role="menuitem" tabindex="-1""#,
            r#" aria-haspopup="menu" aria-expanded="false">"#,
            r#"<span class="dropdown-item-label">Export</span>"#,
            r#"<span class="submenu-icon" aria-hidden="true">{chevron}</span>"#,
            r#"<div class="dropdown-submenu" role="menu" tabindex="-1" aria-orientation="vertical" hidden>"#,
            r#"<div class="dropdown-item neutral" role="menuitem" tabindex="-1" data-value="pdf">"#,
            r#"<span class="dropdown-item-label">PDF</span></div></div></div>"#,
        ),
        chevron = chevron,
    );

    let item = rsx! {
        <DropdownItem submenu=(rsx! {
            <DropdownItem value="pdf">"PDF"</DropdownItem>
        })>
            "Export"
        </DropdownItem>
    };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn item_adjacent_alignment() {
    let expected = concat!(
        r#"<div class="dropdown-item checkbox-adjacent submenu-adjacent neutral" role="menuitem" tabindex="-1">"#,
        r#"<span class="dropdown-item-label">Preferences</span></div>"#,
    );

    let item = rsx! {
        <DropdownItem checkbox_adjacent=true submenu_adjacent=true>"Preferences"</DropdownItem>
    };
    assert_eq!(item.render().as_inner(), &expected);
}

#[test]
fn additional_attributes() {
    let expected = dropdown(
        r#"id="the-dropdown" class="dropdown test""#,
        r#"data-placement="bottom-start""#,
        &menu(""),
    );

    let dropdown = Dropdown::builder().id("the-dropdown").class("test");
    assert_eq!(dropdown.render().as_inner(), &expected);
}
