use proc_macro::TokenStream;
use syn::parse_macro_input;

mod attribute;
mod derive;
mod functional;

#[proc_macro_derive(Props, attributes(props, prop))]
pub fn derive_props(input: TokenStream) -> TokenStream {
    derive::props(parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

#[proc_macro_attribute]
pub fn const_str(args: TokenStream, input: TokenStream) -> TokenStream {
    attribute::const_str(parse_macro_input!(args), parse_macro_input!(input))
        .unwrap_or_else(|err| err.to_compile_error())
        .into()
}

/// Expands `htmx=[expr]` attributes inside the body into the full set of `hx-*` attributes, then
/// forwards to [`hypertext::rsx!`].
#[proc_macro]
pub fn htmx_rsx(input: TokenStream) -> TokenStream {
    functional::htmx::rsx(input.into()).into()
}
