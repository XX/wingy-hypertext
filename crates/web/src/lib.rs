pub mod components;
pub mod dom;
pub mod layouts;
pub mod utils;

pub use components::copy_button::register_copy_action;
pub use components::head::init_scroll_to_anchor;
pub use layouts::code_example::{init_code_examples, listen_code_examples};
pub use layouts::page::init_page_element;
pub use utils::action::listen_click_actions;
