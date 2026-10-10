pub use hypertext;
pub use wingy_hypertext_macros as macros;

pub mod action;
pub mod appearance;
pub mod attributes;
pub mod class;
pub mod component;
pub mod convert;
pub mod helper;
pub mod htmx;
pub mod icon_placement;
pub mod layout;
pub mod link;
pub mod orientation;
#[cfg(test)]
pub mod tests;
pub mod variant;

#[macro_export]
macro_rules! tag_component {
    ($(#[$meta:meta])* $name:ident, $class:ident, $tag:ident) => {
        $(#[$meta])*
        #[derive(Default, ::derive_more::AsRef, ::derive_more::AsMut, $crate::macros::Props)]
        #[$crate::macros::const_str(CLASS = $class)]
        #[props(builder)]
        pub struct $name<'a> {
            #[as_ref]
            #[as_mut]
            pub attributes: $crate::attributes::CommonAttrs<'a>,

            pub children: Option<&'a dyn ::hypertext::Renderable>,
        }

        impl<'a> ::hypertext::Renderable for $name<'a> {
            fn render_to(&self, buffer: &mut ::hypertext::Buffer) {
                use ::hypertext::prelude::{GlobalAttributes, hypertext_elements};

                let id = self.not_empty_id();
                let class_line = self.class_line_with(&[Self::CLASS]);
                let style_line = self.style_line_with(&[]);

                ::hypertext::rsx! {
                    <$tag id=[id] class=[&class_line] style=[&style_line] (self.get_attrs())>
                        (self.children)
                    </$tag>
                }
                .render_to(buffer);
            }
        }
    };
}

#[macro_export]
macro_rules! div_component {
    ($(#[$meta:meta])* $name:ident, $class:ident) => { $crate::tag_component!($(#[$meta])* $name, $class, div); }
}

#[macro_export]
macro_rules! span_component {
    ($(#[$meta:meta])* $name:ident, $class:ident) => { $crate::tag_component!($(#[$meta])* $name, $class, span); }
}
