#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod state;

use anyhow::Result;
pub use state::{command, State};

/// Start the backend Tauri application.
#[tokio::main]
async fn main() -> Result<()> {
    let state = State::new().await?;

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
