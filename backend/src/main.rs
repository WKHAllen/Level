#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod state;

use anyhow::Result;
pub use state::{command, State};
use tauri::Manager;

/// Start the backend Tauri application.
#[tokio::main]
async fn main() -> Result<()> {
    let state = State::new();

    tauri::async_runtime::set(tokio::runtime::Handle::current());

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![command])
        .on_window_event(|event| {
            let state = event.window().state::<State>();

            tokio::task::block_in_place(|| {
                tauri::async_runtime::block_on(async {
                    state.handle_event(event.event()).await.unwrap();
                });
            });
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
