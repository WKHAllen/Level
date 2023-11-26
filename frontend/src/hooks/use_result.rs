use crate::backend::BackendHandle;
use crate::hooks::{use_alert, use_backend, use_view, UseAlert, UseAlertHandle, UseViewHandle};
use crate::util::*;
use crate::view::View;
use commands::FrontendCommands;
use common::*;
use std::rc::Rc;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

/// An intermediate error representation.
#[derive(Debug, Clone, PartialEq)]
enum UseResultError {
    /// An expected error.
    Expected(ExpectedCommandError),
    /// An unexpected error.
    Unexpected(String),
}

impl From<CommandError> for UseResultError {
    fn from(value: CommandError) -> Self {
        match value {
            CommandError::Expected(err) => Self::Expected(err),
            CommandError::Unexpected(err) => Self::Unexpected(format_log_message(&format!(
                "An unexpected error occurred: {}",
                err
            ))),
            CommandError::Other(err) => {
                Self::Unexpected(format_log_message(&format!("An error occurred: {}", err)))
            }
        }
    }
}

/// A handle to an application frontend `CommandResult` handler.
#[derive(Clone)]
pub struct UseResultHandle {
    /// A handle to the application view.
    view: UseViewHandle,
    /// A handle to the global alert.
    alert: UseAlertHandle,
    /// A handle to the backend.
    backend: Rc<BackendHandle>,
}

impl UseResultHandle {
    /// Handle a `CommandResult`. When an unexpected error occurs, this will
    /// log the error in the JS console, close any open save file, return to
    /// the home view, and indicate that an error occurred.
    pub fn handle<T>(&self, res: CommandResult<T>) -> Result<T, ExpectedCommandError> {
        match res {
            Ok(value) => Ok(value),
            Err(err) => {
                match UseResultError::from(err) {
                    UseResultError::Expected(err) => Err(err),
                    UseResultError::Unexpected(message) => {
                        console_error!("{}", message);

                        let view = self.view.clone();
                        let alert = self.alert.clone();
                        let backend = self.backend.clone();

                        spawn_local(async move {
                            // This error can be discarded. If it occurs it's probably
                            // because no save is open. But even for other errors we
                            // will still discard it, as we are already handling
                            // another unexpected error.
                            _ = backend.close_save_file().await;

                            view.set(View::Home);
                            alert.open(UseAlert::new().title("Unexpected Error").text(
                                "An unexpected error occurred. See the logs for more details.",
                            ));
                        });

                        // This should be the only place in the entire
                        // application where this error variant is used.
                        Err(ExpectedCommandError::UnexpectedError)
                    }
                }
            }
        }
    }
}

/// Gets a `CommandResult` handler. The handler can be used to handle any
/// potentially failure occurring on the backend.
#[hook]
pub fn use_result() -> UseResultHandle {
    let view = use_view();
    let alert = use_alert();
    let (backend, _) = use_backend();

    UseResultHandle {
        view,
        alert,
        backend,
    }
}
