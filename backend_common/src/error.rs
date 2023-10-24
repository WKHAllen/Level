use common::*;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::io;
use std::ops::Deref;
use std::result::Result as StdResult;
use std::string::FromUtf8Error;
use thiserror::Error;

/// An expected application error.
#[derive(Debug, Error)]
#[error("{0}")]
pub struct ExpectedError(#[from] pub ExpectedCommandError);

impl Deref for ExpectedError {
    type Target = ExpectedCommandError;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<aes_gcm::Error> for ExpectedError {
    fn from(_: aes_gcm::Error) -> Self {
        Self(ExpectedCommandError::InvalidSavePassword)
    }
}
/// An unexpected application error.
#[derive(Debug, Error)]
pub enum UnexpectedError {
    /// An error in an I/O operation.
    #[error("An error occurred in an I/O operation: {0}")]
    IoError(#[from] io::Error),
    /// An error in a database operation.
    #[error("An error occurred in a database operation: {0}")]
    SqlError(#[from] sqlx::Error),
    /// An error in a UTF-8 conversion operation.
    #[error("An error occurred during UTF-8 conversion: {0}")]
    Utf8Error(#[from] FromUtf8Error),
    /// A JSON serialization error.
    #[error("An error occurred while processing JSON data: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// A generic application error.
#[derive(Debug, Error)]
pub enum Error<E = Infallible> {
    /// An expected error.
    #[error("An error occurred: {0}")]
    Expected(#[from] ExpectedError),
    /// An unexpected error.
    #[error("An unexpected error occurred: {0}")]
    Unexpected(#[from] UnexpectedError),
    /// A custom error.
    #[error("An error occurred: {0}")]
    Other(E),
}

impl<E> From<aes_gcm::Error> for Error<E> {
    fn from(value: aes_gcm::Error) -> Self {
        ExpectedError::from(value).into()
    }
}

impl<E> From<io::Error> for Error<E> {
    fn from(value: io::Error) -> Self {
        UnexpectedError::from(value).into()
    }
}

impl<E> From<sqlx::Error> for Error<E> {
    fn from(value: sqlx::Error) -> Self {
        UnexpectedError::from(value).into()
    }
}

impl<E> From<FromUtf8Error> for Error<E> {
    fn from(value: FromUtf8Error) -> Self {
        UnexpectedError::from(value).into()
    }
}

impl<E> From<serde_json::Error> for Error<E> {
    fn from(value: serde_json::Error) -> Self {
        UnexpectedError::from(value).into()
    }
}

impl<E> From<ExpectedCommandError> for Error<E> {
    fn from(value: ExpectedCommandError) -> Self {
        ExpectedError::from(value).into()
    }
}

impl<E> From<Error<Error<E>>> for Error<E> {
    fn from(value: Error<Error<E>>) -> Self {
        match value {
            Error::Expected(err) => Self::Expected(err),
            Error::Unexpected(err) => Self::Unexpected(err),
            Error::Other(err) => err,
        }
    }
}

impl From<ExpectedError> for CommandError {
    fn from(value: ExpectedError) -> Self {
        Self::Expected(value.0)
    }
}

impl From<UnexpectedError> for CommandError {
    fn from(value: UnexpectedError) -> Self {
        match value {
            UnexpectedError::IoError(err) => {
                Self::Unexpected(UnexpectedCommandError::IoError(GenericError::new(&err)))
            }
            UnexpectedError::SqlError(err) => {
                Self::Unexpected(UnexpectedCommandError::SqlError(GenericError::new(&err)))
            }
            UnexpectedError::Utf8Error(err) => {
                Self::Unexpected(UnexpectedCommandError::Utf8Error(GenericError::new(&err)))
            }
            UnexpectedError::JsonError(err) => {
                Self::Unexpected(UnexpectedCommandError::JsonError(GenericError::new(&err)))
            }
        }
    }
}

impl<E> From<Error<E>> for CommandError
where
    E: ToString,
{
    fn from(value: Error<E>) -> Self {
        match value {
            Error::Expected(err) => err.into(),
            Error::Unexpected(err) => err.into(),
            Error::Other(err) => CommandError::Other(GenericError::new(&err)),
        }
    }
}

/// A generic application result.
pub type Result<T, E = Infallible> = StdResult<T, Error<E>>;

/// A error occurring from a Tauri command.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Error)]
pub enum TauriCommandError {
    /// An invalid Tauri command.
    #[error("Invalid command: {0}")]
    InvalidCommand(String),
}
