#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, ItemTrait, TraitItem};

/// Rewrite the command trait differently for the frontend and backend.
#[proc_macro_attribute]
pub fn command_trait(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemTrait);

    let ident = &input.ident;
    let vis = &input.vis;
    let items = input.items;
    let backend_ident = format_ident!("Backend{}", ident.to_string());
    let frontend_ident = format_ident!("Frontend{}", ident.to_string());

    quote! {
        #[::macros::note_trait_methods]
        #[::async_trait::async_trait]
        #vis trait #backend_ident {
            #(#items)*
        }

        #[::macros::note_trait_methods]
        #[::async_trait::async_trait(?Send)]
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

    quote! {
        #input

        #vis const #note_ident: &[&'static str] = &[#(#methods),*];
    }
    .into()
}
