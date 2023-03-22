use dotenv::dotenv;

/// Build the backend Tauri application.
fn main() {
    dotenv().ok();

    tauri_build::build()
}
