use web_sys::Element;

pub fn is_open(element: &Element) -> bool {
    element.class_list().contains("open")
}

pub fn is_disabled(element: &Element) -> bool {
    element.class_list().contains("disabled")
}

pub fn is_multiple(element: &Element) -> bool {
    element.class_list().contains("multiple")
}

pub fn is_selected(element: &Element) -> bool {
    element.class_list().contains("selected")
}
