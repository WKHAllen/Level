#![forbid(unsafe_code)]

use commands::BACKENDCOMMANDS_METHODS;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, token::Comma, FnArg, ItemImpl, Signature};

/// Wrap the backend's implementation of application commands in Tauri's command interface.
#[proc_macro_attribute]
pub fn backend_commands(_: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemImpl);

    let mut method_matches = Vec::new();
    let mut method_arg_structs = Vec::new();

    for method_str in BACKENDCOMMANDS_METHODS {
        let method_tokens = method_str.parse::<TokenStream>().unwrap();
        let method = parse_macro_input!(method_tokens as Signature);
        let method_async = method.asyncness.is_some();
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

        let method_call = if method_async {
            quote! { state.#method_name(#input_names).await }
        } else {
            quote! { state.#method_name(#input_names) }
        };

        method_matches.push(quote! {
            #method_name_str => {
                let deserialized_args = ::serde_json::from_str(&args).unwrap();
                let #struct_name { #input_names } = deserialized_args;
                let res = #method_call;
                Ok(::serde_json::to_string(&res).unwrap())
            },
        });
    }

    quote! {
        #[::async_trait::async_trait]
        #input

        #(#method_arg_structs)*

        #[::tauri::command(async)]
        pub async fn command(name: String, args: String, state: ::tauri::State<'_, State>) -> Result<String, String> {
            match name.as_str() {
                #(#method_matches)*
                cmd => Err(format!("Invalid command: {}", cmd)),
            }
        }
    }
    .into()
}
