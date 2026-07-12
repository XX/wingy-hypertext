use hypertext::{RenderableExt, rsx};

use crate::appearance::Appearance::*;
use crate::attributes::CommonAttributeSetters;
use crate::components::select::{Select, SelectOption};

const CHECK_ICON: &str = r#"<span class="check" aria-hidden="true"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M434.8 70.1c14.3 10.4 17.5 30.4 7.1 44.7l-256 352c-5.5 7.6-14 12.3-23.4 13.1s-18.5-2.7-25.1-9.3l-128-128c-12.5-12.5-12.5-32.8 0-45.3s32.8-12.5 45.3 0l101.5 101.5 234-321.7c10.4-14.3 30.4-17.5 44.7-7.1z"/></svg></span>"#;

fn combobox(placeholder: Option<&str>, name: Option<&str>) -> String {
    let placeholder = placeholder
        .map(|text| format!(r#" placeholder="{text}""#))
        .unwrap_or_default();
    let name = name.map(|name| format!(r#" name="{name}""#)).unwrap_or_default();
    format!(
        r#"<div class="combobox"><input class="display-input" type="text"{placeholder} autocomplete="off" spellcheck="false" autocapitalize="off" readonly role="combobox" aria-haspopup="listbox" aria-expanded="false"><input class="value-input" type="text"{name} tabindex="-1" aria-hidden="true"><span class="expand-icon" aria-hidden="true"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"/></svg></span></div>"#
    )
}

fn select_popup(combobox: &str, multiselectable: &str, options: &str) -> String {
    format!(
        r#"<div class="select-popup popup" data-placement="bottom" data-flip data-shift data-sync="width" data-auto-size="vertical" data-auto-size-padding="10">{combobox}<div class="popup-body"><div class="listbox" role="listbox" tabindex="-1" aria-multiselectable="{multiselectable}" hidden>{options}</div></div></div>"#
    )
}

fn option_markup(value: &str, classes: &str, extra: &str, label: &str) -> String {
    format!(
        r#"<div class="option{classes}" role="option" tabindex="-1"{extra} data-value="{value}">{CHECK_ICON}<span class="option-label">{label}</span></div>"#
    )
}

#[test]
fn default() {
    let select_markup = format!(
        r#"<div class="select accent">{}</div>"#,
        select_popup(&combobox(None, None), "false", "")
    );

    let select = Select::builder();
    assert_eq!(select.render().as_inner(), select_markup.as_str());

    let select = rsx! { <Select /> };
    assert_eq!(select.render().as_inner(), select_markup.as_str());
}

#[test]
fn options() {
    let options = [
        option_markup("a", "", r#" aria-selected="false""#, "Ant"),
        option_markup("b", " selected", r#" aria-selected="true""#, "Bee"),
        option_markup(
            "c",
            " disabled",
            r#" aria-selected="false" aria-disabled="true""#,
            "Cat",
        ),
    ]
    .join("");
    let select_markup = format!(
        r#"<div class="select accent">{}</div>"#,
        select_popup(&combobox(None, None), "false", &options)
    );

    let select = rsx! {
        <Select>
            <SelectOption value="a">"Ant"</SelectOption>
            <SelectOption value="b" selected=true>"Bee"</SelectOption>
            <SelectOption value="c" disabled=true>"Cat"</SelectOption>
        </Select>
    };
    assert_eq!(select.render().as_inner(), select_markup.as_str());
}

#[test]
fn shorthand_label_and_hint() {
    let select_markup = format!(
        r#"<div class="select accent"><label class="label">Pet</label>{}<small class="hint">Choose a pet</small></div>"#,
        select_popup(&combobox(Some("Select a pet"), Some("pet")), "false", "")
    );

    let select = Select::builder()
        .label("Pet")
        .hint("Choose a pet")
        .placeholder("Select a pet")
        .name("pet");
    assert_eq!(select.render().as_inner(), select_markup.as_str());
}

#[test]
fn states() {
    let combobox_markup = r#"<div class="combobox"><input class="display-input" type="text" disabled="true" autocomplete="off" spellcheck="false" autocapitalize="off" readonly role="combobox" aria-haspopup="listbox" aria-expanded="false"><div class="tags"></div><input class="value-input" type="text" disabled="true" required="true" tabindex="-1" aria-hidden="true"><span class="expand-icon" aria-hidden="true"><svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 448 512"><path fill="currentColor" d="M201.4 406.6c12.5 12.5 32.8 12.5 45.3 0l192-192c12.5-12.5 12.5-32.8 0-45.3s-32.8-12.5-45.3 0L224 338.7 54.6 169.4c-12.5-12.5-32.8-12.5-45.3 0s-12.5 32.8 0 45.3l192 192z"/></svg></span></div>"#;
    let select_markup = format!(
        r#"<div class="select required multiple disabled outlined">{}</div>"#,
        select_popup(combobox_markup, "true", "")
    );

    let select = Select::builder()
        .appearance(Outlined)
        .disabled(true)
        .required(true)
        .multiple(true);
    assert_eq!(select.render().as_inner(), select_markup.as_str());
}

#[test]
fn max_options_visible() {
    let select = Select::builder().multiple(true).max_options_visible(5);
    let markup = select.render();
    let markup = markup.as_inner();

    assert!(markup.contains(r#"data-max-options-visible="5""#));
    assert!(markup.contains(r#"<div class="tags"></div>"#));
}

#[test]
fn placement() {
    let select = Select::builder().placement(crate::components::select::SelectPlacement::Top);
    let markup = select.render();
    let markup = markup.as_inner();

    assert!(markup.contains(r#"data-placement="top""#));
}

#[test]
fn with_clear() {
    let select = Select::builder().with_clear(true);
    let markup = select.render();
    let markup = markup.as_inner();

    assert!(
        markup.contains(r#"<button class="clear-button" type="button" tabindex="-1" aria-label="Clear entry" hidden>"#)
    );
}

#[test]
fn option_explicit_label() {
    let option_markup = option_markup("a", "", r#" aria-selected="false" data-label="Ant""#, "");
    let expected = option_markup.replace(
        r#" data-label="Ant" data-value="a""#,
        r#" data-value="a" data-label="Ant""#,
    );

    let option = SelectOption::builder().value("a").label("Ant");
    assert_eq!(option.render().as_inner(), expected.as_str());
}

#[test]
fn additional_attributes() {
    let select_markup = format!(
        r#"<div id="pet" class="select pill accent test" style="color: red">{}</div>"#,
        select_popup(&combobox(None, Some("pet")), "false", "")
    );

    let select = Select::builder()
        .pill(true)
        .name("pet")
        .id("pet")
        .class("test")
        .style("color: red");
    assert_eq!(select.render().as_inner(), select_markup.as_str());

    let select = rsx! {
        <Select pill=true name="pet" id="pet" class="test" style="color: red"></Select>
    };
    assert_eq!(select.render().as_inner(), select_markup.as_str());
}
