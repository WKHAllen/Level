//! Common interfaces for the level frontend.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use js_sys::{Function, Promise, Reflect};
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;

pub use frontend_macros::*;

/// Arguments passed to the backend Tauri command.
#[derive(Serialize)]
struct CommandArgs {
    /// The name of the command.
    name: String,
    /// A string representation of the serialized command arguments.
    args: String,
}

/// Invoke the backend command and return the response.
pub async fn tauri_command<S, R>(command: &str, args: &S) -> R
where
    S: Serialize + ?Sized,
    R: DeserializeOwned + ?Sized,
{
    let tauri = web_sys::window().unwrap().get("__TAURI__").unwrap();
    let invoke = Reflect::get(&tauri.into(), &"invoke".into()).unwrap();
    let invoke_function = invoke.dyn_ref::<Function>().unwrap();

    let serialized_args = serde_json::to_string(args).unwrap();
    let command_args = CommandArgs {
        name: command.to_owned(),
        args: serialized_args,
    };
    let js_args = serde_wasm_bindgen::to_value(&command_args).unwrap();

    let response = invoke_function
        .call2(invoke_function, &"command".into(), &js_args)
        .unwrap();
    let response_promise = response.dyn_into::<Promise>().unwrap();
    let response_future = JsFuture::from(response_promise);
    let command_res = response_future.await.unwrap();
    let serialized_res: String = serde_wasm_bindgen::from_value(command_res).unwrap();
    serde_json::from_str(&serialized_res).unwrap()
}
