use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::parse::ParseStream;
use syn::spanned::Spanned;
use syn::{
    AngleBracketedGenericArguments, Data, DeriveInput, Error, GenericArgument, GenericParam, Generics, Ident,
    PathArguments, Token, Type, TypeParam, parse_quote,
};

pub fn props(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "#[derive(Props)] may only be used on structs",
        ));
    };

    let vis = &input.vis;
    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let generic_params = input
        .generics
        .params
        .iter()
        .filter_map(|param| {
            if let GenericParam::Type(type_param) = param
                && type_param.default.is_some()
            {
                None
            } else {
                Some(quote!(#param))
            }
        })
        .collect::<Vec<_>>();
    let impl_needed_generics = if !generic_params.is_empty() {
        quote!(<#(#generic_params,)*>)
    } else {
        quote!()
    };

    let mut generate_builder = false;
    let mut impl_builder_struct_ga = None;
    let mut builder_impl = None;
    let mut methods = Vec::new();
    let mut from_impls = Vec::new();

    if let Some(props_attr) = input.attrs.iter().find(|attr| attr.path().is_ident("props")) {
        props_attr.parse_args_with(|props_args: ParseStream| {
            let prop_ident = props_args.parse::<Ident>()?;
            if prop_ident == "builder" {
                generate_builder = true;
                let builder_start_fn = quote! {
                    #[must_use]
                    #vis fn builder() -> Self {
                        Self::default()
                    }
                };
                let builder_finish_fn = quote! {
                    #[must_use]
                    #vis fn build(self) -> Self {
                        self
                    }
                };

                if !props_args.is_empty() {
                    props_args.parse::<Token![for]>()?;
                    let struct_ga = props_args.parse::<AngleBracketedGenericArguments>()?;
                    impl_builder_struct_ga = Some(quote!(#struct_ga));
                } else if input.generics.type_params().all(|param| param.default.is_some()) {
                    let params = input.generics.params.iter().map(|param| {
                        if let GenericParam::Type(type_param) = param
                            && let Some(default) = &type_param.default
                        {
                            quote!(#default)
                        } else {
                            quote!(#param)
                        }
                    });
                    impl_builder_struct_ga = Some(quote!(<#(#params,)*>));
                }

                if let Some(struct_ga) = &impl_builder_struct_ga {
                    builder_impl = Some(quote! {
                        #[automatically_derived]
                        impl #impl_needed_generics #struct_name #struct_ga {
                            #builder_start_fn
                        }
                    });
                }

                if builder_impl.is_none() {
                    methods.push(builder_start_fn);
                }
                methods.push(builder_finish_fn);

                Ok(())
            } else {
                Err(props_args.error("unexpected param for `#[props(...)]`"))
            }
        })?;
    }

    for field in &data_struct.fields {
        if let Some(name) = &field.ident {
            let ty = &field.ty;
            let mut is_skip = false;
            let mut is_skip_setter = false;
            let mut is_impl_from = false;
            let mut is_into = false;
            let mut is_convert = false;

            if let Some(prop_attr) = field.attrs.iter().find(|attr| attr.path().is_ident("prop")) {
                prop_attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("skip") {
                        is_skip = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("impl_from") {
                        is_impl_from = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("into") {
                        is_into = true;
                        return Ok(());
                    }

                    if meta.path.is_ident("convert") {
                        is_convert = true;
                        return Ok(());
                    }

                    Err(meta.error("unexpected param for `#[prop(...)]`"))
                })?;
            }

            if !is_skip {
                let (is_optional, ty) = extract_option_type(ty);
                let value = if is_into { quote!(#name.into()) } else { quote!(#name) };
                let value_ty = if is_into {
                    quote!(impl ::std::convert::Into<#ty>)
                } else {
                    quote!(#ty)
                };
                let set_value = if is_optional {
                    quote!(self.#name = Some(#value))
                } else {
                    quote!(self.#name = #value)
                };

                if is_convert {
                    if let Some(ty_ident) = type_as_ident(ty) {
                        let mut generics = input.generics.clone();
                        let Some(new_ty_param) =
                            replace_generic_param(&mut generics, ty_ident.to_string(), parse_quote!(PropConvertNewTy))
                        else {
                            return Err(Error::new_spanned(ty, "type param not found for `#[prop(convert)]`"));
                        };

                        let new_ty_ident = new_ty_param.ident.clone();
                        // The method must declare only the replaced param: the rest are already
                        // in scope from the surrounding impl block.
                        let method_generics = Generics {
                            lt_token: Some(Default::default()),
                            params: [GenericParam::Type(new_ty_param)].into_iter().collect(),
                            gt_token: Some(Default::default()),
                            where_clause: None,
                        };
                        let (impl_generics, ..) = method_generics.split_for_impl();
                        let (_, new_ty_generics, where_clause) = generics.split_for_impl();

                        let fields = data_struct.fields.iter().filter_map(|field| {
                            field.ident.as_ref().map(|ident| {
                                if ident == name {
                                    if is_optional {
                                        quote!(#name: Some(#name))
                                    } else {
                                        quote!(#name)
                                    }
                                } else {
                                    quote!(#ident: self.#ident)
                                }
                            })
                        });

                        methods.push(quote! {
                            #[must_use]
                            #vis fn #name #impl_generics(self, #name: #new_ty_ident) -> #struct_name #new_ty_generics #where_clause {
                                #struct_name {
                                    #(#fields,)*
                                }
                            }
                        });
                        is_skip_setter = true;
                    } else {
                        return Err(Error::new_spanned(ty, "unsupported type for `#[prop(convert)]`"));
                    }
                } else {
                    methods.push(quote! {
                        #[must_use]
                        #vis fn #name(mut self, #name: #value_ty) -> Self {
                            #set_value;
                            self
                        }
                    });
                }

                if !is_skip_setter {
                    let setter_name = format_ident!("set_{}", name);
                    methods.push(quote! {
                        #vis fn #setter_name(&mut self, #name: #value_ty) -> &mut Self {
                            #set_value;
                            self
                        }
                    });
                }
            }

            if is_impl_from {
                let builder = if generate_builder {
                    quote!(Self::builder())
                } else {
                    quote!(Self::default())
                };

                let body = if is_skip {
                    quote! {
                        #builder.#name(#name)
                    }
                } else {
                    quote! {
                        let mut this = #builder;
                        this.#name = #name;
                        this
                    }
                };

                from_impls.push(if let Some(struct_ga) = &impl_builder_struct_ga {
                    quote! {
                        #[automatically_derived]
                        impl #impl_needed_generics From<#ty> for #struct_name #struct_ga {
                            fn from(#name: #ty) -> Self {
                                #body
                            }
                        }
                    }
                } else {
                    quote! {
                        #[automatically_derived]
                        impl #impl_generics From<#ty> for #struct_name #ty_generics #where_clause {
                            fn from(#name: #ty) -> Self {
                                #body
                            }
                        }
                    }
                });
            }
        }
    }

    let output = if methods.is_empty() {
        quote! {
            #builder_impl

            #(#from_impls)*
        }
    } else {
        quote! {
            #builder_impl

            #[automatically_derived]
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#methods)*
            }

            #(#from_impls)*
        }
    };

    Ok(output)
}

pub fn dyn_renderable(input: DeriveInput) -> syn::Result<TokenStream> {
    let Data::Struct(data_struct) = &input.data else {
        return Err(syn::Error::new(
            input.span(),
            "#[derive(DynRenderable)] may only be used on structs",
        ));
    };

    let struct_name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let method = if let Some(attr) = input.attrs.iter().find(|attr| attr.path().is_ident("render_to")) {
        attr.parse_args::<Ident>()?
    } else {
        format_ident!("render_to")
    };

    let erased: Vec<&Ident> = data_struct
        .fields
        .iter()
        .filter_map(|field| {
            if field.attrs.iter().any(|attr| attr.path().is_ident("skip_render")) {
                return None;
            }

            let name = field.ident.as_ref()?;
            let (is_optional, inner_ty) = extract_option_type(&field.ty);
            let inner_ident = type_as_ident(inner_ty)?;

            (is_optional
                && input
                    .generics
                    .type_params()
                    .any(|type_param| type_param.ident == *inner_ident))
            .then_some(name)
        })
        .collect();

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::hypertext::Renderable for #struct_name #ty_generics #where_clause {
            fn render_to(&self, buffer: &mut ::hypertext::Buffer) {
                #(let #erased = self.#erased.as_ref().map(|renderable| renderable as &dyn ::hypertext::Renderable);)*
                self.#method(buffer #(, #erased)*)
            }
        }
    })
}

fn extract_option_type(ty: &Type) -> (bool, &Type) {
    if let Type::Path(type_path) = ty
        && let Some(segment) = type_path.path.segments.last()
        && segment.ident == "Option"
        && let PathArguments::AngleBracketed(args) = &segment.arguments
        && let Some(GenericArgument::Type(inner_ty)) = args.args.first()
    {
        (true, inner_ty)
    } else {
        (false, ty)
    }
}

fn type_as_ident(ty: &Type) -> Option<&Ident> {
    if let Type::Path(type_path) = ty
        && type_path.qself.is_none()
        && type_path.path.segments.len() == 1
    {
        Some(&type_path.path.segments[0].ident)
    } else {
        None
    }
}

fn replace_generic_param(generics: &mut Generics, target: impl AsRef<str>, new_ty: Ident) -> Option<TypeParam> {
    for param in &mut generics.params {
        if let GenericParam::Type(ty_param) = param
            && ty_param.ident == target.as_ref()
        {
            ty_param.ident = new_ty;
            return Some(ty_param.clone());
        }
    }
    None
}
