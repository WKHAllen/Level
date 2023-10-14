#![forbid(unsafe_code)]

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Metadata associated with a database save file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveMetadata {
    /// The name of the save.
    pub name: String,
    /// A description of the save.
    pub description: String,
    /// When the save was created.
    pub created_at: NaiveDateTime,
    /// When the save was last opened.
    pub last_opened_at: NaiveDateTime,
}

/// An error occurred while opening a save file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum OpenSaveError {
    /// A save file is already open.
    #[error("A save file is already open")]
    SaveAlreadyOpen,
    /// The save file could not be found.
    #[error("The save file could not be found")]
    SaveNotFound,
    /// The save file could not be unlocked with the provided password.
    #[error("The save file could not be unlocked with the provided password")]
    InvalidPassword,
}
