//! Macros for all parts of the application.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Fields, ItemEnum, ItemTrait, TraitItem};

/// Rewrite the command trait differently for the frontend and backend.
#[proc_macro_attribute]
pub fn command_trait(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let ident = &input.ident;
    let vis = &input.vis;
    let items = input.items;
    let attrs = input.attrs;
    let backend_ident = format_ident!("Backend{}", ident.to_string());
    let frontend_ident = format_ident!("Frontend{}", ident.to_string());

    quote! {
        #[::macros::note_trait_methods]
        #[::async_trait::async_trait]
        #(#attrs)*
        #vis trait #backend_ident {
            #(#items)*
        }

        #[::macros::note_trait_methods]
        #[::async_trait::async_trait(?Send)]
        #(#attrs)*
        #vis trait #frontend_ident {
            #(#items)*
        }
    }
    .into()
}

/// Note the methods on a trait for future use.
#[proc_macro_attribute]
pub fn note_trait_methods(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let ident = &input.ident;
    let vis = &input.vis;
    let note_ident = format_ident!("{}_METHODS", ident.to_string().to_uppercase());
    let methods = input
        .items
        .iter()
        .filter_map(|item| match &item {
            &TraitItem::Method(method) => Some(method.sig.to_token_stream().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let doc = format!("Methods on the `{}` trait", ident);

    quote! {
        #input

        #[doc = #doc]
        #vis const #note_ident: &[&'static str] = &[#(#methods),*];
    }
    .into()
}

/// Derive selection options behavior on an enum.
#[proc_macro_derive(SelectOptions)]
pub fn derive_select_options(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemEnum);

    let ident = &input.ident;

    let variants = match input
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| match variant.fields {
            Fields::Unit => Ok((index, &variant.ident)),
            _ => Err((
                "`SelectOptions` cannot be derived for enums containing non-unit variants",
                variant,
            )),
        })
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(val) => val,
        Err((msg, tokens)) => {
            return syn::Error::new_spanned(tokens, msg)
                .to_compile_error()
                .into();
        }
    };

    let from_index_branches = variants
        .iter()
        .map(|(index, variant)| {
            quote! {
                #index => Self::#variant,
            }
        })
        .collect::<Vec<_>>();

    let current_index_branches = variants
        .iter()
        .map(|(index, variant)| {
            quote! {
                Self::#variant => #index,
            }
        })
        .collect::<Vec<_>>();

    let num_options = variants.len();

    quote! {
        impl SelectOptions for #ident {
            const NUM_OPTIONS: usize = #num_options;

            fn from_index(index: usize) -> Self {
                match index {
                    #(#from_index_branches)*
                    _ => unreachable!("Invalid option index"),
                }
            }

            fn current_index(&self) -> usize {
                match self {
                    #(#current_index_branches)*
                }
            }
        }
    }
    .into()
}
