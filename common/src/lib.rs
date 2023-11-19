//! Common interfaces for all parts of the application.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::error::Error as StdError;
use std::fmt::Display;
use std::ops::Deref;
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

/// A generic error that only cares about the error message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GenericError(pub String);

impl GenericError {
    /// Creates a new generic error from any error type.
    pub fn new<E>(err: &E) -> Self
    where
        E: ToString,
    {
        Self(err.to_string())
    }
}

impl Deref for GenericError {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl StdError for GenericError {}

/// An expected command error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum ExpectedCommandError {
    /// An unexpected error. Counterintuitive as it may seem, this does serve
    /// a legitimate purpose. It exists to indicate to the frontend that an
    /// unexpected error has occurred and is being correctly handled. This
    /// error will likely never appear in the UI, as it will only be set
    /// between renders. This error variant should never be set by the
    /// backend.
    #[error("An unexpected error occurred")]
    UnexpectedError,
    /// A save operation was attempted, but no save was open.
    #[error("No save file is open")]
    NoSaveOpen,
    /// An attempt was made to open a save, but one was already open.
    #[error("A save file is already open")]
    SaveAlreadyOpen,
    /// A save with the given name already exists.
    #[error("A save with the given name already exists")]
    SaveAlreadyExists,
    /// The save file could not be found.
    #[error("The save file could not be found")]
    SaveNotFound,
    /// The save file could not be unlocked with the provided password.
    #[error("The save file could not be unlocked with the provided password")]
    InvalidSavePassword,
    /// The specified subcategory is not within the specified category.
    #[error("The specified subcategory is not within the specified category")]
    InvalidSubcategory,
    /// An invalid account type was specified.
    #[error("An invalid account type was specified")]
    InvalidAccountType,
    /// An invalid transaction type was specified.
    #[error("An invalid transaction type was specified")]
    InvalidTransactionType,
    /// A budget already exists for the specified account.
    #[error("A budget already exists for the specified account")]
    BudgetAlreadyExists,
    /// An invalid timeframe was specified.
    #[error("An invalid timeframe was specified")]
    InvalidTimeframe,
}

/// An unexpected command error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum UnexpectedCommandError {
    /// An I/O error occurred.
    #[error("An I/O error occurred: {0}")]
    IoError(GenericError),
    /// An error in a database operation.
    #[error("An error occurred in a database operation: {0}")]
    SqlError(GenericError),
    /// An error in a UTF-8 conversion operation.
    #[error("An error occurred during UTF-8 conversion: {0}")]
    Utf8Error(GenericError),
    /// A JSON serialization error.
    #[error("An error occurred while processing JSON data: {0}")]
    JsonError(GenericError),
}

/// A generic command error. This makes a distinction between errors that are
/// expected to occur and those that are not.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Error)]
pub enum CommandError {
    /// An expected command error. This kind of error will likely be shown to
    /// the user.
    #[error("An error occurred: {0}")]
    Expected(#[from] ExpectedCommandError),
    /// An unexpected command error. This kind of error will not commonly be
    /// shown to the user, but will instead be immediately unwrapped.
    #[error("An unexpected error occurred: {0}")]
    Unexpected(#[from] UnexpectedCommandError),
    /// A different error occurred. Custom errors usually should not make it
    /// to this point.
    #[error("An error occurred: {0}")]
    Other(GenericError),
}

/// A generic command result.
pub type CommandResult<T> = Result<T, CommandError>;

/// Formats log messages with the current timestamp.
pub fn format_log_message(message: &str) -> String {
    let now = Local::now().format("%a %Y-%m-%d %H:%M:%S%.3f");
    format!("[{}] {}\n", now, message)
}
