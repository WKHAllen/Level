use backend_common::*;
use chrono::NaiveDate;
use commands::BackendCommands;
use common::*;
use db::*;
use log::{error, info};
use std::collections::HashMap;
use std::env;
use std::future::Future;
use std::pin::Pin;
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
    pub fn with_save<'a, F, R>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = Result<R>> + Send + 'a>>
    where
        for<'c> F:
            FnOnce(&'c mut Save) -> Pin<Box<dyn Future<Output = R> + Send + 'c>> + Send + Sync + 'a,
    {
        Box::pin(async move {
            let mut handle = self.save_handle().await?;
            Ok(f(&mut handle).await)
        })
    }

    /// Grants exclusive access to the database, automatically rolling back on
    /// failure.
    pub fn with_db<'a, F, R>(&'a self, f: F) -> Pin<Box<dyn Future<Output = Result<R>> + Send + 'a>>
    where
        for<'c> F: FnOnce(&'c mut DBImpl) -> Pin<Box<dyn Future<Output = Result<R>> + Send + 'c>>
            + Send
            + Sync
            + 'a,
        R: Send,
    {
        Box::pin(async move {
            let mut handle = self.save_handle().await?;
            handle.transaction(f).await
        })
    }

    /// Grants exclusive access to the database, automatically rolls back on
    /// failure, and handles errors appropriately.
    pub fn with<'a, F, R>(
        &'a self,
        f: F,
    ) -> Pin<Box<dyn Future<Output = CommandResult<R>> + Send + 'a>>
    where
        for<'c> F: FnOnce(&'c mut DBImpl) -> Pin<Box<dyn Future<Output = Result<R>> + Send + 'c>>
            + Send
            + Sync
            + 'a,
        R: Send,
    {
        Box::pin(async move {
            match self.with_db(f).await {
                Ok(value) => Ok(value),
                Err(err) => {
                    match &err {
                        Error::Expected(_) => {}
                        Error::Unexpected(inner) => {
                            error!("An unexpected error occurred: {}", inner);
                        }
                        Error::Other(_) => {
                            unreachable!("`Other` variant inner error is `Infallible`")
                        }
                    }

                    Err(err.into())
                }
            }
        })
    }

    /// Performs any fallible async operation with automatic error handling.
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

    async fn create_save_file(
        &self,
        save_name: String,
        save_description: String,
        save_password: String,
    ) -> CommandResult<()> {
        info!("Attempting to create save file: {}", &save_name);

        self.with_result(self.create_save(&save_name, &save_description, &save_password))
            .await
    }

    async fn save_info(&self) -> CommandResult<SaveMetadata> {
        self.with_result(async {
            let handle = self.save_handle().await?;
            Ok(handle.this_metadata())
        })
        .await
    }

    async fn accounts(&self) -> CommandResult<Vec<Account>> {
        self.with(|db| Account::list(db)).await
    }

    async fn create_account(
        &self,
        account_type: AccountType,
        name: String,
        description: String,
    ) -> CommandResult<Account> {
        self.with(|db| {
            Box::pin(async move { Account::create(db, account_type, &name, &description).await })
        })
        .await
    }

    async fn transaction_batch(
        &self,
        account: Account,
        num_transactions: usize,
        limit: usize,
    ) -> CommandResult<Vec<(AccountTransaction, Vec<AccountTransactionTag>)>> {
        self.with(|db| {
            Box::pin(async move {
                let transactions =
                    AccountTransaction::batch(db, &account, num_transactions, limit).await?;
                let transaction_tags = AccountTransactionTag::list_by_transaction_batch(
                    db,
                    &account,
                    num_transactions,
                    limit,
                )
                .await?;
                let mut transaction_tags_map = transaction_tags.into_iter().fold(
                    HashMap::new(),
                    |mut map: HashMap<String, Vec<AccountTransactionTag>>, transaction_tag| {
                        let transaction_tags = map
                            .entry(transaction_tag.account_transaction_id.clone())
                            .or_default();
                        transaction_tags.push(transaction_tag);
                        map
                    },
                );
                let batch = transactions
                    .into_iter()
                    .map(|transaction| {
                        let this_transaction_tags = transaction_tags_map
                            .remove(&transaction.id)
                            .unwrap_or_default();
                        (transaction, this_transaction_tags)
                    })
                    .collect();
                Ok(batch)
            })
        })
        .await
    }

    async fn create_transaction(
        &self,
        mut account: Account,
        name: String,
        description: String,
        amount: f64,
        transaction_type: TransactionType,
        institution: Institution,
        date: NaiveDate,
        category: Category,
        subcategory: Option<Subcategory>,
        tags: Vec<Tag>,
    ) -> CommandResult<(AccountTransaction, Vec<AccountTransactionTag>)> {
        self.with(|db| {
            Box::pin(async move {
                let transaction = AccountTransaction::create(
                    db,
                    &mut account,
                    &name,
                    &description,
                    amount,
                    transaction_type,
                    &institution,
                    date,
                    &category,
                    subcategory.as_ref(),
                )
                .await?;

                let mut transaction_tags = Vec::new();

                for tag in tags.iter() {
                    let transaction_tag =
                        AccountTransactionTag::create(db, &transaction, tag).await?;
                    transaction_tags.push(transaction_tag);
                }

                Ok((transaction, transaction_tags))
            })
        })
        .await
    }

    async fn institutions(&self) -> CommandResult<Vec<Institution>> {
        self.with(|db| Institution::list(db)).await
    }

    async fn create_institution(
        &self,
        name: String,
        description: String,
    ) -> CommandResult<Institution> {
        self.with(|db| Box::pin(async move { Institution::create(db, &name, &description).await }))
            .await
    }

    async fn update_institution(
        &self,
        mut institution: Institution,
        name: String,
        description: String,
    ) -> CommandResult<()> {
        self.with(|db| {
            Box::pin(async move {
                institution.set_name(db, &name).await?;
                institution.set_description(db, &description).await?;
                Ok(())
            })
        })
        .await
    }

    async fn delete_institution(&self, institution: Institution) -> CommandResult<()> {
        self.with(|db| institution.delete(db)).await
    }

    async fn categories(&self) -> CommandResult<Vec<Category>> {
        self.with(|db| Category::list(db)).await
    }

    async fn create_category(&self, name: String, description: String) -> CommandResult<Category> {
        self.with(|db| Box::pin(async move { Category::create(db, &name, &description).await }))
            .await
    }

    async fn update_category(
        &self,
        mut category: Category,
        name: String,
        description: String,
    ) -> CommandResult<()> {
        self.with(|db| {
            Box::pin(async move {
                category.set_name(db, &name).await?;
                category.set_description(db, &description).await?;
                Ok(())
            })
        })
        .await
    }

    async fn delete_category(&self, category: Category) -> CommandResult<()> {
        self.with(|db| category.delete(db)).await
    }

    async fn subcategories(&self) -> CommandResult<Vec<Subcategory>> {
        self.with(|db| Subcategory::list(db)).await
    }

    async fn subcategories_within(&self, category: Category) -> CommandResult<Vec<Subcategory>> {
        self.with(|db| Box::pin(async move { Subcategory::list_within(db, &category).await }))
            .await
    }

    async fn create_subcategory_within(
        &self,
        category: Category,
        name: String,
        description: String,
    ) -> CommandResult<Subcategory> {
        self.with(|db| {
            Box::pin(async move { Subcategory::create(db, &category, &name, &description).await })
        })
        .await
    }

    async fn update_subcategory(
        &self,
        mut subcategory: Subcategory,
        name: String,
        description: String,
    ) -> CommandResult<()> {
        self.with(|db| {
            Box::pin(async move {
                subcategory.set_name(db, &name).await?;
                subcategory.set_description(db, &description).await?;
                Ok(())
            })
        })
        .await
    }

    async fn delete_subcategory(&self, subcategory: Subcategory) -> CommandResult<()> {
        self.with(|db| subcategory.delete(db)).await
    }

    async fn tags(&self) -> CommandResult<Vec<Tag>> {
        self.with(|db| Tag::list(db)).await
    }

    async fn create_tag(&self, name: String, description: String) -> CommandResult<Tag> {
        self.with(|db| Box::pin(async move { Tag::create(db, &name, &description).await }))
            .await
    }

    async fn update_tag(
        &self,
        mut tag: Tag,
        name: String,
        description: String,
    ) -> CommandResult<()> {
        self.with(|db| {
            Box::pin(async move {
                tag.set_name(db, &name).await?;
                tag.set_description(db, &description).await?;
                Ok(())
            })
        })
        .await
    }

    async fn delete_tag(&self, tag: Tag) -> CommandResult<()> {
        self.with(|db| tag.delete(db)).await
    }
}
