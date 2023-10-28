use backend_common::*;
use commands::BackendCommands;
use common::*;
use db::{DBImpl, Save};
use log::{error, info};
use std::env;
use std::future::Future;
use std::sync::Arc;
use tauri::WindowEvent;
use tokio::sync::{MappedMutexGuard, Mutex, MutexGuard};

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
        if let WindowEvent::CloseRequested { .. } = event {
            info!("Window close requested, shutting down gracefully");

            if self.is_save_open().await {
                self.close_save().await?;
            }
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
            Err(ExpectedCommandError::SaveAlreadyOpen)?;
        }

        let save = Save::create(save_name, save_description, save_password).await?;
        *save_option = Some(save);

        Ok(())
    }

    /// Opens a save file.
    pub async fn open_save(&self, save_name: &str, save_password: &str) -> Result<()> {
        let mut save_option = self.save.lock().await;

        if save_option.is_some() {
            Err(ExpectedCommandError::SaveAlreadyOpen)?;
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
            None => Err(ExpectedCommandError::NoSaveOpen)?,
        }
    }

    /// Returns a handle to the inner save instance.
    pub async fn save_handle(&self) -> Result<MappedMutexGuard<Save>> {
        let save_option = self.save.lock().await;

        match &*save_option {
            Some(_) => Ok(MutexGuard::map(save_option, |guard| {
                guard.as_mut().unwrap()
            })),
            None => Err(ExpectedCommandError::NoSaveOpen)?,
        }
    }

    /// Grants exclusive access to the save instance via a closure.
    pub async fn with_save<F, T, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut Save) -> T,
        T: Future<Output = R>,
    {
        let mut handle = self.save_handle().await?;
        Ok(f(&mut handle).await)
    }

    /// Grants exclusive access to the database, automatically rolling back on
    /// failure.
    pub async fn with_db<F, T, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&mut DBImpl) -> T + Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send,
        R: Send,
    {
        let mut handle = self.save_handle().await?;
        handle.transaction(f).await
    }

    /// Grants exclusive access to the database, automatically rolls back on
    /// failure, and handles errors appropriately.
    pub async fn with<F, T, R>(&self, f: F) -> CommandResult<R>
    where
        F: FnOnce(&mut DBImpl) -> T + Send + Sync + 'static,
        T: Future<Output = Result<R>> + Send,
        R: Send,
    {
        match self.with_db(f).await {
            Ok(value) => Ok(value),
            Err(err) => {
                match &err {
                    Error::Expected(_) => {}
                    Error::Unexpected(inner) => {
                        error!("An unexpected error occurred: {}", inner);
                    }
                    Error::Other(_) => unreachable!("`Other` variant inner error is `Infallible`"),
                }

                Err(err.into())
            }
        }
    }

    /// Performs any async operation with automatic error handling.
    pub async fn with_result<F, R>(&self, f: F) -> CommandResult<R>
    where
        F: Future<Output = Result<R>>,
    {
        match f.await {
            Ok(value) => Ok(value),
            Err(err) => {
                match &err {
                    Error::Expected(_) => {}
                    Error::Unexpected(inner) => {
                        error!("An unexpected error occurred: {}", inner);
                    }
                    Error::Other(_) => unreachable!("`Other` variant inner error is `Infallible`"),
                }

                Err(err.into())
            }
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

#[backend_commands]
impl BackendCommands for State {
    async fn demo_mode(&self) -> bool {
        env::args().any(|arg| arg == "--demo")
    }

    async fn list_save_files(&self) -> CommandResult<Vec<SaveMetadata>> {
        self.with_result(Save::list()).await
    }

    async fn open_save_file(&self, save_name: String, save_password: String) -> CommandResult<()> {
        info!("Attempting to open save file: {}", &save_name);

        self.with_result(self.open_save(&save_name, &save_password))
            .await
    }

    async fn close_save_file(&self) -> CommandResult<()> {
        info!("Attempting to close the save file");

        self.with_result(self.close_save()).await
    }
}
