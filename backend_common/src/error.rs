use common::*;
use std::convert::Infallible;
use std::io;
use std::string::FromUtf8Error;
use thiserror::Error;

/// A generic crypto error.
#[derive(Debug, Error)]
pub enum CryptoError {
    /// An error in an I/O operation.
    #[error("An error occurred in an I/O operation: {0}")]
    IoError(#[from] io::Error),
    /// An error in an AES operation.
    #[error("An error occurred in an AES operation")]
    AesError,
}

impl From<aes_gcm::Error> for CryptoError {
    fn from(_: aes_gcm::Error) -> Self {
        Self::AesError
    }
}

/// A generic crypto result.
pub type CryptoResult<T> = Result<T, CryptoError>;

/// A generic database error.
#[derive(Debug, Error)]
pub enum DBError<E = Infallible> {
    /// An error in an I/O operation.
    #[error("An error occurred in an I/O operation: {0}")]
    IoError(#[from] io::Error),
    /// An error in a database operation.
    #[error("An error occurred in a database operation: {0}")]
    SqlError(#[from] sqlx::Error),
    /// An error in a UTF-8 conversion operation.
    #[error("An error occurred during UTF-8 conversion: {0}")]
    Utf8Error(#[from] FromUtf8Error),
    /// A different error.
    #[error("An error occurred: {0}")]
    Other(E),
}

/// A generic database result.
pub type DBResult<T, E = Infallible> = Result<T, DBError<E>>;

/// An error involving a save file.
#[derive(Debug, Error)]
pub enum SaveError {
    /// A save with the given name already exists.
    #[error("A save with the given name already exists")]
    SaveAlreadyExists,
    /// The save file could not be found.
    #[error("The save file could not be found")]
    SaveNotFound,
    /// An I/O error occurred.
    #[error("An I/O error occurred: {0}")]
    IoError(#[from] io::Error),
    /// An error in an AES operation.
    #[error("An error occurred in an AES operation")]
    AesError,
    /// An error in a database operation.
    #[error("An error occurred in a database operation: {0}")]
    SqlError(#[from] sqlx::Error),
    /// An error in a UTF-8 conversion operation.
    #[error("An error occurred during UTF-8 conversion: {0}")]
    Utf8Error(#[from] FromUtf8Error),
}

impl From<CryptoError> for SaveError {
    fn from(value: CryptoError) -> Self {
        match value {
            CryptoError::IoError(err) => Self::IoError(err),
            CryptoError::AesError => Self::AesError,
        }
    }
}

impl From<DBError<Infallible>> for SaveError {
    fn from(value: DBError) -> Self {
        match value {
            DBError::IoError(err) => Self::IoError(err),
            DBError::SqlError(err) => Self::SqlError(err),
            DBError::Utf8Error(err) => Self::Utf8Error(err),
            DBError::Other(_) => unreachable!(),
        }
    }
}

impl<E> From<DBError<E>> for SaveError
where
    E: Into<Self>,
{
    fn from(value: DBError<E>) -> Self {
        match value {
            DBError::IoError(err) => Self::IoError(err),
            DBError::SqlError(err) => Self::SqlError(err),
            DBError::Utf8Error(err) => Self::Utf8Error(err),
            DBError::Other(err) => err.into(),
        }
    }
}

/// A result involving a save file.
pub type SaveResult<T> = Result<T, SaveError>;

/// An error involving the application state.
#[derive(Debug, Error)]
pub enum StateError {
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
    /// An I/O error occurred.
    #[error("An I/O error occurred: {0}")]
    IoError(io::Error),
    /// An error in an AES operation.
    #[error("An error occurred in an AES operation")]
    AesError,
    /// An error in a database operation.
    #[error("An error occurred in a database operation: {0}")]
    SqlError(#[from] sqlx::Error),
    /// An error in a UTF-8 conversion operation.
    #[error("An error occurred during UTF-8 conversion: {0}")]
    Utf8Error(FromUtf8Error),
}

impl From<SaveError> for StateError {
    fn from(value: SaveError) -> Self {
        match value {
            SaveError::SaveAlreadyExists => Self::SaveAlreadyExists,
            SaveError::SaveNotFound => Self::SaveNotFound,
            SaveError::IoError(err) => Self::IoError(err),
            SaveError::AesError => Self::AesError,
            SaveError::SqlError(err) => Self::SqlError(err),
            SaveError::Utf8Error(err) => Self::Utf8Error(err),
        }
    }
}

impl From<StateError> for CommandError {
    fn from(val: StateError) -> Self {
        match val {
            StateError::NoSaveOpen => Self::Expected(ExpectedCommandError::NoSaveOpen),
            StateError::SaveAlreadyOpen => Self::Expected(ExpectedCommandError::SaveAlreadyOpen),
            StateError::SaveAlreadyExists => {
                Self::Expected(ExpectedCommandError::SaveAlreadyExists)
            }
            StateError::SaveNotFound => Self::Expected(ExpectedCommandError::SaveNotFound),
            StateError::IoError(err) => {
                Self::Unexpected(UnexpectedCommandError::IoError(GenericError::new(&err)))
            }
            StateError::AesError => Self::Expected(ExpectedCommandError::InvalidSavePassword),
            StateError::SqlError(err) => {
                Self::Unexpected(UnexpectedCommandError::SqlError(GenericError::new(&err)))
            }
            StateError::Utf8Error(err) => {
                Self::Unexpected(UnexpectedCommandError::Utf8Error(GenericError::new(&err)))
            }
        }
    }
}

impl From<SaveError> for CommandError {
    fn from(value: SaveError) -> Self {
        StateError::from(value).into()
    }
}

/// A result involving the application state.
pub type StateResult<T> = Result<T, StateError>;
