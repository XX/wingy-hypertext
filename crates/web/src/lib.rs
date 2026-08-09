pub mod component;
pub mod helper;
pub mod layout;
pub mod util;

pub use component::copy_button::register_copy_action;
pub use component::head::init_scroll_to_anchor;
pub use layout::code_example::{init_code_examples, listen_code_examples};
pub use layout::page::init_page_element;
pub use util::action::listen_click_actions;
