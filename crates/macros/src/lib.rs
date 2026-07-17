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

/// Derives `hypertext::Renderable` as a thin wrapper delegating to an inherent render method —
/// `render_to` by default, overridable with `#[render_to(name)]` on the struct.
///
/// Every field typed `Option<P>`, where `P` is one of the struct's type params, is erased to
/// `Option<&dyn Renderable>` and passed to the delegate after the buffer, in field declaration
/// order; fields marked `#[skip_render]` are excluded, and without erased fields the call is
/// delegated as is. The delegate must render the passed arguments and never touch the erased
/// fields through `self`, so LLVM can merge the per-child-type instantiations into one copy —
/// critical for WASM binary size.
#[proc_macro_derive(DynRenderable, attributes(render_to, skip_render))]
pub fn derive_dyn_renderable(input: TokenStream) -> TokenStream {
    derive::dyn_renderable(parse_macro_input!(input))
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
