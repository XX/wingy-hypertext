use js_sys::Date;
use web_sys::Element;

/// How long the type-to-select buffer lives without new keystrokes.
pub const TYPEAHEAD_TIMEOUT_MILLIS: f64 = 1000.0;

pub fn typeahead_buffer(element: &Element) -> String {
    let last = element
        .get_attribute("data-typeahead-time")
        .and_then(|value| value.parse::<f64>().ok())
        .unwrap_or(0.0);

    if Date::now() - last > TYPEAHEAD_TIMEOUT_MILLIS {
        String::new()
    } else {
        element.get_attribute("data-typeahead").unwrap_or_default()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeaheadKey {
    Backspace,
    Char(char),
}

impl TypeaheadKey {
    pub fn new(key: &str) -> Option<Self> {
        if key == "Backspace" {
            Some(Self::Backspace)
        } else if key.chars().count() == 1 {
            Some(Self::Char(key.chars().next()?.to_ascii_lowercase()))
        } else {
            None
        }
    }
}

pub fn update_typeahead_buffer(element: &Element, key: TypeaheadKey) -> String {
    let mut buffer = typeahead_buffer(element);
    match key {
        TypeaheadKey::Backspace => drop(buffer.pop()),
        TypeaheadKey::Char(ch) => buffer.push(ch),
    }
    element.set_attribute("data-typeahead", &buffer).ok();
    element
        .set_attribute("data-typeahead-time", &Date::now().to_string())
        .ok();
    buffer
}
