//! The color scheme selector of the top app bar: an icon button opening a menu
//! with the light, dark and system schemes, like the one in the Web Awesome docs.

use hypertext::prelude::{GlobalAttributes, hypertext_elements};
use hypertext::{Renderable, rsx};
use iconic::fontawesome_ext;
use wasm_bindgen::JsCast;
use wasm_dom as dom;
use wasm_dom::event::EventListener;
use wasm_dom::existing::JsObjectAccess;
use wasm_dom::existing::access::CastToElement;
use web_sys::{CustomEvent, Event};
use wingy_hypertext::appearance::Appearance::*;
use wingy_hypertext::attributes::CommonAttributeSetters;
use wingy_hypertext::attrs;
use wingy_hypertext::class::{CHECKED, DROPDOWN_ITEM, ICON, SIZE_SMALL};
use wingy_hypertext::component::button::Button;
use wingy_hypertext::component::dropdown::{Dropdown, DropdownItem, DropdownItemIcon, DropdownItemLabel, DropdownMenu};
use wingy_hypertext::helper::popup::PopupPlacement::BottomEnd;
use wingy_hypertext::layout::divider::Divider;
use wingy_hypertext_web::util::color_scheme::{ColorScheme, color_scheme, listen_color_scheme, set_color_scheme};
use wingy_hypertext_web::util::event;

const SELECTOR_ID: &str = "color-scheme-selector";

pub fn selector() -> impl Renderable {
    rsx! {
        <Dropdown id=SELECTOR_ID placement=BottomEnd>
            <Button
                class=SIZE_SMALL
                appearance=Plain
                pill=true
                attrs=(attrs!["title" = &"Toggle color scheme", "aria-label" = &"Toggle color scheme"])
            >
                <span class=(ICON, " ", "only-light")>
                    (fontawesome_ext::regular::SunBright)
                </span>
                <span class=(ICON, " ", "only-dark")>
                    (fontawesome_ext::regular::MoonStars)
                </span>
            </Button>
            <DropdownMenu>
                <DropdownItem checkbox=true value=(ColorScheme::Light.as_ref())>
                    <DropdownItemIcon>(fontawesome_ext::regular::SunBright)</DropdownItemIcon>
                    <DropdownItemLabel>"Light"</DropdownItemLabel>
                </DropdownItem>
                <DropdownItem checkbox=true value=(ColorScheme::Dark.as_ref())>
                    <DropdownItemIcon>(fontawesome_ext::regular::MoonStars)</DropdownItemIcon>
                    <DropdownItemLabel>"Dark"</DropdownItemLabel>
                </DropdownItem>
                <Divider/>
                <DropdownItem checkbox=true value=(ColorScheme::Auto.as_ref())>
                    <DropdownItemLabel>"System"</DropdownItemLabel>
                </DropdownItem>
            </DropdownMenu>
        </Dropdown>
    }
}

/// Applies the stored color scheme, checks the matching item of the selector
/// and switches the scheme on selection.
pub fn listen_color_scheme_selector() {
    listen_color_scheme();
    sync_selector(color_scheme());

    dom::existing::document().add_steady_event_listener(event::SELECT, |event| {
        handle_select(&event);
    });
}

fn handle_select(event: &Event) -> Option<()> {
    let dropdown = event.target()?.maybe_into_element()?;
    if dropdown.id() != SELECTOR_ID {
        return None;
    }

    let custom: &CustomEvent = event.dyn_ref()?;
    let value = custom.detail().get("value").as_string()?;
    let scheme = ColorScheme::parse(&value)?;

    set_color_scheme(scheme);
    sync_selector(scheme);

    Some(())
}

/// Checks the item of `scheme` only: selecting a checkable item toggles it, so
/// the selected one is re-checked and the others are unchecked.
fn sync_selector(scheme: ColorScheme) {
    let selector = format!("#{SELECTOR_ID} .{DROPDOWN_ITEM}[role=menuitemcheckbox]");
    for item in dom::existing::select_all_elements(&selector) {
        let checked = item.get_attribute("data-value").as_deref() == Some(scheme.as_ref());
        item.class_list().toggle_with_force(CHECKED, checked).ok();
        item.set_attribute("aria-checked", if checked { "true" } else { "false" })
            .ok();
    }
}
