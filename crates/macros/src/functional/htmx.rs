use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use quote::{format_ident, quote};

/// Canonical list of supported HTMX attributes, in render order.
///
/// `hx-on` (a namespaced attribute) and the deprecated `hx-vars` are intentionally omitted.
const HTMX_ATTRS: &[&str] = &[
    "hx-get",
    "hx-post",
    "hx-put",
    "hx-patch",
    "hx-delete",
    "hx-push-url",
    "hx-select",
    "hx-select-oob",
    "hx-swap",
    "hx-swap-oob",
    "hx-target",
    "hx-trigger",
    "hx-vals",
    "hx-boost",
    "hx-confirm",
    "hx-disable",
    "hx-disabled-elt",
    "hx-disinherit",
    "hx-encoding",
    "hx-ext",
    "hx-headers",
    "hx-history",
    "hx-history-elt",
    "hx-include",
    "hx-indicator",
    "hx-inherit",
    "hx-params",
    "hx-preserve",
    "hx-prompt",
    "hx-replace-url",
    "hx-request",
    "hx-sync",
    "hx-validate",
    "sse-connect",
    "sse-swap",
    "sse-close",
    "ws-connect",
    "ws-send",
];

/// Rewrites every `htmx=[EXPR]` attribute inside an `rsx!`-like body into the full set of
/// `hx-*=[(EXPR).hx_*()]` attributes, then forwards the result to `::hypertext::rsx!`.
pub fn rsx(input: TokenStream) -> TokenStream {
    let rewritten = rewrite(input);

    quote! {
        ::hypertext::rsx! { #rewritten }
    }
}

/// Walks the token stream, replacing `htmx = [ … ]` triples and recursing into nested groups.
fn rewrite(input: TokenStream) -> TokenStream {
    let mut out = TokenStream::new();
    let mut iter = input.into_iter().peekable();

    while let Some(tt) = iter.next() {
        if matches!(&tt, TokenTree::Ident(ident) if *ident == "htmx") {
            // Try to match the `htmx = [ EXPR ]` attribute shape.
            if matches!(iter.peek(), Some(TokenTree::Punct(punct)) if punct.as_char() == '=') {
                let eq = iter.next().expect("peeked");

                if matches!(iter.peek(), Some(TokenTree::Group(group)) if group.delimiter() == Delimiter::Bracket) {
                    let Some(TokenTree::Group(group)) = iter.next() else {
                        unreachable!("peeked a bracket group")
                    };
                    out.extend(expand(&group.stream()));
                } else {
                    // Not our pattern after all: emit the consumed tokens verbatim.
                    out.extend([tt, eq]);
                }
            } else {
                out.extend([tt]);
            }

            continue;
        }

        if let TokenTree::Group(group) = &tt {
            let mut new_group = Group::new(group.delimiter(), rewrite(group.stream()));
            new_group.set_span(group.span());
            out.extend([TokenTree::Group(new_group)]);
            continue;
        }

        out.extend([tt]);
    }

    out
}

/// Builds the `hx-*=[(EXPR).hx_*()]` attribute tokens for a single `htmx=[EXPR]`.
fn expand(expr: &TokenStream) -> TokenStream {
    let mut out = TokenStream::new();

    for attr in HTMX_ATTRS {
        let name: TokenStream = attr.parse().expect("attribute name is valid tokens");
        let method = format_ident!("{}", attr.replace('-', "_"));

        out.extend(quote! {
            #name=[(#expr).#method()]
        });
    }

    out
}
