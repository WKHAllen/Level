//! Macros for the level frontend.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use commands::FRONTENDCOMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Expr, FnArg, ItemStruct, Signature};

/// Implement application command methods for the frontend.
#[proc_macro_derive(FrontendCommands)]
pub fn derive_frontend_commands(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);

    let ident = &input.ident;

    let mut methods = Vec::new();
    let mut method_arg_structs = Vec::new();

    for method_str in FRONTENDCOMMANDS_METHODS {
        let method_tokens = method_str.parse::<TokenStream>().unwrap();
        let method = parse_macro_input!(method_tokens as Signature);
        let method_name = &method.ident;
        let method_name_str = method_name.to_string();
        let struct_name = quote::format_ident!("__command__{}", method_name);
        let inputs = method
            .inputs
            .iter()
            .filter(|arg| match arg {
                FnArg::Receiver(_) => false,
                FnArg::Typed(_) => true,
            })
            .collect::<Punctuated<_, Comma>>();
        let input_names = inputs
            .iter()
            .filter_map(|arg| match arg {
                FnArg::Typed(pat) => Some(*pat.pat.clone()),
                _ => None,
            })
            .collect::<Punctuated<_, Comma>>();

        method_arg_structs.push(quote! {
            #[allow(non_camel_case_types)]
            #[derive(Debug, ::serde::Serialize, ::serde::Deserialize)]
            struct #struct_name {
                #inputs
            }
        });

        methods.push(quote! {
            #method {
                let args = #struct_name {
                    #input_names
                };
                let res = ::frontend_common::tauri_command(#method_name_str, &args).await;
                res
            }
        });
    }

    quote! {
        #(#method_arg_structs)*

        #[::async_trait::async_trait(?Send)]
        impl ::commands::FrontendCommands for #ident {
            #(#methods)*
        }
    }
    .into()
}

/// Performs validation on a variable number of stateful values and collapses
/// the resulting `Option`s into a single `Option`, where the final `Option`
/// is `Some` if and only if all values in the original tuple of `Option`s are
/// also `Some`.
///
/// In other words, this runs all validator functions and converts the
/// resulting `(Option<T1>, Option<T2>, Option<T3>, ...)` into
/// `Option<(T1, T2, T3, ...)>`.
#[proc_macro]
pub fn validate_all(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item with Punctuated::<Expr, Comma>::parse_terminated);

    let value_idents = input
        .iter()
        .enumerate()
        .map(|(index, _)| format_ident!("value{}", index))
        .collect::<Vec<_>>();

    let some_values = value_idents.iter().map(|ident| quote! { Some(#ident) });

    quote! {
        match (#input) {
            (#(#some_values),*) => Some((#(#value_idents),*)),
            _ => None,
        }
    }
    .into()
}
