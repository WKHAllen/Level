use anyhow::Result;
use db::*;
use dotenv::dotenv;
use std::env;

/// Test save file name.
const TEST_SAVE_NAME: &str = "My Test Save";

/// Test save file description.
const TEST_SAVE_DESCRIPTION: &str = "A save file used primarily for application testing.";

/// Test save file password.
const TEST_SAVE_PASSWORD: &str = "password123";

/// Creates and populates a test save file.
async fn create_test_save() -> Result<()> {
    if Save::exists("My Test Save") {
        return Ok(());
    }

    // Create the save file
    let save = Save::create(TEST_SAVE_NAME, TEST_SAVE_DESCRIPTION, TEST_SAVE_PASSWORD).await?;

    // Add accounts

    // Add reminders

    // Add budgets

    // Add categories

    // Add subcategories

    // Add institutions

    // Add transactions

    // Add tags

    // Link transactions to tags

    // Add report templates

    // Close the save file
    save.close().await?;

    Ok(())
}

/// Build the backend Tauri application and create the test save file.
#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    if let Ok("true") = env::var("CREATE_TEST_SAVE").as_deref() {
        create_test_save().await?;
    }

    tauri_build::build();

    Ok(())
}
