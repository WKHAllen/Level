#![forbid(unsafe_code)]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod state;

use anyhow::Result;
use db::Quote;
pub use state::{command, State};

/// Populate the database with some values and create the backend application state.
async fn init_state() -> Result<State> {
    let state = State::new().await?;

    let list = Quote::list(&state.db).await;

    for quote in list {
        quote.delete(&state.db).await;
    }

    let quotes = [
        "We're not free in what we do because we're not free in what we want.",
        "Give a man a gun and he can rob a bank, but give a man a bank, and he can rob the world.",
        "Where I once would fear the cost of truth, now I only ask: What is the cost of lies?",
    ];

    for quote in &quotes {
        Quote::create(&state.db, quote).await;
    }

    Ok(state)
}

/// Start the backend Tauri application.
#[tokio::main]
async fn main() -> Result<()> {
    let state = init_state().await?;

    tauri::Builder::default()
        .manage(state)
        .invoke_handler(tauri::generate_handler![command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
