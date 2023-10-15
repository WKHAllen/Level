#![forbid(unsafe_code)]

use common::*;
use macros::command_trait;

/// Global application commands, designed to facilitate communication between the frontend and backend.
#[command_trait]
pub trait Commands {
    /// Checks whether the app is in demo mode.
    async fn demo_mode(&self) -> bool;

    /// Lists all existing save files.
    async fn list_save_files(&self) -> Vec<SaveMetadata>;

    /// Attempts to open an existing save file.
    async fn open_save_file(
        &self,
        save_name: String,
        save_password: String,
    ) -> Result<(), OpenSaveError>;
}
