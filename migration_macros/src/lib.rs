#![forbid(unsafe_code)]

mod version;

use proc_macro::TokenStream;
use quote::quote;
use std::fs;
use syn::{punctuated::Punctuated, token::Comma, Expr};
use version::*;

/// Migrate from one application version to another.
#[proc_macro]
pub fn migrate_from(item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item with Punctuated::<Expr, Comma>::parse_terminated)
        .into_iter()
        .collect::<Vec<_>>();
    let (current_version_expr, arg_exprs) = match input.split_first() {
        Some(x) => x,
        None => return quote!(compile_error!("expected version number")).into(),
    };
    let arg_exprs = arg_exprs
        .iter()
        .map(|x| x.to_owned())
        .collect::<Punctuated<Expr, Comma>>();

    let current_version_expr_str = quote!(#current_version_expr).to_string().replace("\"", "");
    let current_version = match parse_version_from_str(&current_version_expr_str) {
        Ok(x) => x,
        Err(err) => {
            let err_str = err.to_string();
            return quote!(compile_error!(#err_str)).into();
        }
    };

    let current_version: Option<Version> = current_version.map(|v| v.into());

    let entries = match fs::read_dir("migration/src/migrations") {
        Ok(x) => x,
        Err(err) => {
            let err_str = err.to_string();
            return quote!(compile_error!(#err_str)).into();
        }
    };

    let good_entries = entries
        .filter_map(|entry| entry.ok().filter(|e| e.file_name() != "mod.rs"))
        .collect::<Vec<_>>();
    let possible_versions = good_entries
        .iter()
        .map(|entry| parse_version_from_path(entry.path()).map(|e| e.into()))
        .collect::<Vec<Result<Version, _>>>();
    let err_versions = possible_versions
        .iter()
        .filter_map(|v| v.as_ref().err())
        .collect::<Vec<_>>();

    match err_versions.get(0) {
        Some(&err) => return quote!(compile_error!(#err)).into(),
        None => (),
    };

    let mut versions = possible_versions
        .into_iter()
        .map(|entry| entry.unwrap())
        .collect::<Vec<_>>();
    versions.sort();

    let current_version_index = match current_version {
        Some(cv) => match versions.iter().position(|v| *v == cv) {
            Some(x) => x + 1,
            None => {
                let err_msg = format!(
                    "could not find migration implementation for version: {}",
                    cv
                );
                return quote!(compile_error!(#err_msg)).into();
            }
        },
        None => 0,
    };

    let (_, migration_versions) = versions.split_at(current_version_index);
    let migration_fn_calls = migration_versions
        .iter()
        .map(|v| {
            let migration_version_str = quote::format_ident!("{}", v.file_id());
            quote!(migrations::#migration_version_str::migrate(#arg_exprs).unwrap();)
        })
        .collect::<Vec<_>>();

    quote! {
        #(
            #migration_fn_calls
        )*
    }
    .into()
}
