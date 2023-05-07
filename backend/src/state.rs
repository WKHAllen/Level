use anyhow::Result;
use backend_macros::backend_commands;
use commands::BackendCommands;
use db::Save;
use std::fmt::Display;
use std::sync::Arc;
use std::{error::Error as StdError, future::Future};
use tauri::WindowEvent;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

#[derive(Debug, Clone)]
pub enum StateError {
    NoSaveOpen,
    SaveAlreadyOpen,
}

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match *self {
            Self::NoSaveOpen => "no save is open",
            Self::SaveAlreadyOpen => "a save is already open",
        })
    }
}

impl StdError for StateError {}

/// The backend application state.
pub struct State {
    /// The backend database.
    save: Arc<Mutex<Option<Save>>>,
}

impl State {
    /// Initializes the backend state.
    pub fn new() -> Self {
        Self {
            save: Arc::new(Mutex::new(None)),
        }
    }

    /// Handle a tauri window event.
    pub async fn handle_event(&self, event: &WindowEvent) -> Result<()> {
        match event {
            WindowEvent::CloseRequested { .. } => {
                if self.is_save_open().await {
                    self.close_save().await?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Checks if a save with the given name exists.
    pub fn save_exists(&self, save_name: &str) -> bool {
        Save::exists(save_name)
    }

    /// Checks if a save is currently open.
    pub async fn is_save_open(&self) -> bool {
        self.save.lock().await.is_some()
    }

    /// Creates and opens a new save file.
    pub async fn create_save(
        &self,
        save_name: &str,
        save_description: &str,
        save_password: &str,
    ) -> Result<()> {
        let mut save_option = self.save.lock().await;

        if save_option.is_some() {
            Err(StateError::SaveAlreadyOpen)?;
        }

        let save = Save::create(save_name, save_description, save_password).await?;
        *save_option = Some(save);

        Ok(())
    }

    /// Opens a save file.
    pub async fn open_save(&self, save_name: &str, save_password: &str) -> Result<()> {
        let mut save_option = self.save.lock().await;

        if save_option.is_some() {
            Err(StateError::SaveAlreadyOpen)?;
        }

        let save = Save::open(save_name, save_password).await?;
        *save_option = Some(save);

        Ok(())
    }

    /// Closes the open save file.
    pub async fn close_save(&self) -> Result<()> {
        let mut save_option = self.save.lock().await;

        match save_option.take() {
            Some(save) => {
                save.close().await?;

                Ok(())
            }
            None => Err(StateError::NoSaveOpen)?,
        }
    }

    /// Returns a handle to the inner save instance.
    pub async fn save_handle(&self) -> Result<MappedMutexGuard<Save>> {
        let save_option = self.save.lock().await;

        match &*save_option {
            Some(_) => Ok(MutexGuard::map(save_option, |guard| {
                guard.as_mut().unwrap()
            })),
            None => Err(StateError::NoSaveOpen)?,
        }
    }

    /// Grants exclusive access to the save instance via a closure.
    pub async fn with_save<F, T, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Save) -> T,
        T: Future<Output = R>,
    {
        let mut handle = self.save_handle().await?;
        let ret = f(&mut handle).await;

        Ok(ret)
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[backend_commands]
impl BackendCommands for State {
    async fn say_hi(&self) {
        println!("Hi!");
    }

    async fn greet(&self, name: String) -> String {
        format!("Hello, {}!", name)
    }

    async fn get_random_quote(&self) -> String {
        "Quotes demo removed".to_owned()
    }
}
