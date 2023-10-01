#![forbid(unsafe_code)]

use commands::FRONTENDCOMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, Fields, FnArg, ItemEnum, ItemStruct, Signature};

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
        impl ::frontend_common::SelectOptions for #ident {
            const NUM_OPTIONS: usize = #num_options;

            fn from_index(index: usize) -> Self {
                match index {
                    #(#from_index_branches)*
                    _ => unreachable!(),
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
