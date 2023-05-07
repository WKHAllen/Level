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

    let save_name = "test";
    let save_description = "A test save.";
    let save_password = "password123";

    if state.save_exists(save_name) {
        state.open_save(save_name, save_password).await?;
    } else {
        state
            .create_save(save_name, save_description, save_password)
            .await?;
    };

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
