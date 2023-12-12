//! Backend/frontend communication interfaces.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

use chrono::NaiveDate;
use common::*;
use macros::command_trait;

/// Global application commands, designed to facilitate communication between
/// the frontend and backend.
#[command_trait]
pub trait Commands {
    /// Checks whether the app is in demo mode.
    async fn demo_mode(&self) -> bool;

    /// Lists all existing save files.
    async fn list_save_files(&self) -> CommandResult<Vec<SaveMetadata>>;

    /// Attempts to open an existing save file.
    async fn open_save_file(&self, save_name: String, save_password: String) -> CommandResult<()>;

    /// Attempts to close the currently open save file.
    async fn close_save_file(&self) -> CommandResult<()>;

    /// Attempts to create a new save file.
    async fn create_save_file(
        &self,
        save_name: String,
        save_description: String,
        save_password: String,
    ) -> CommandResult<()>;

    /// Gets information on the currently open save file.
    async fn save_info(&self) -> CommandResult<SaveMetadata>;

    /// Retrieves the accounts within the save file.
    async fn accounts(&self) -> CommandResult<Vec<Account>>;

    /// Creates an account.
    async fn create_account(
        &self,
        account_type: AccountType,
        name: String,
        description: String,
    ) -> CommandResult<Account>;

    /// Retrieves a batch of transactions within an account.
    async fn transaction_batch(
        &self,
        account: Account,
        num_transactions: usize,
        limit: usize,
    ) -> CommandResult<Vec<AccountTransaction>>;

    /// Creates a new transaction.
    async fn create_transaction(
        &self,
        account: Account,
        name: String,
        description: String,
        amount: f64,
        transaction_type: TransactionType,
        institution: Institution,
        date: NaiveDate,
        category: Category,
        subcategory: Option<Subcategory>,
        tags: Vec<Tag>,
    ) -> CommandResult<AccountTransaction>;

    /// Retrieves the institutions within the save file.
    async fn institutions(&self) -> CommandResult<Vec<Institution>>;

    /// Creates a new institution.
    async fn create_institution(
        &self,
        name: String,
        description: String,
    ) -> CommandResult<Institution>;

    /// Updates the institution details.
    async fn update_institution(
        &self,
        institution: Institution,
        name: String,
        description: String,
    ) -> CommandResult<()>;

    /// Deletes the institution.
    async fn delete_institution(&self, institution: Institution) -> CommandResult<()>;

    /// Retrieves the categories within the save file.
    async fn categories(&self) -> CommandResult<Vec<Category>>;

    /// Creates a new category.
    async fn create_category(&self, name: String, description: String) -> CommandResult<Category>;

    /// Updates the category details.
    async fn update_category(
        &self,
        category: Category,
        name: String,
        description: String,
    ) -> CommandResult<()>;

    /// Deletes the category.
    async fn delete_category(&self, category: Category) -> CommandResult<()>;

    /// Retrieves the subcategories that fall under a category.
    async fn subcategories_within(&self, category: Category) -> CommandResult<Vec<Subcategory>>;

    /// Creates a new subcategory within the given category.
    async fn create_subcategory_within(
        &self,
        category: Category,
        name: String,
        description: String,
    ) -> CommandResult<Subcategory>;

    /// Updates the subcategory details.
    async fn update_subcategory(
        &self,
        subcategory: Subcategory,
        name: String,
        description: String,
    ) -> CommandResult<()>;

    /// Deletes the subcategory.
    async fn delete_subcategory(&self, subcategory: Subcategory) -> CommandResult<()>;

    /// Retrieves the tags within the save file.
    async fn tags(&self) -> CommandResult<Vec<Tag>>;

    /// Creates a new tag.
    async fn create_tag(&self, name: String, description: String) -> CommandResult<Tag>;

    /// Updates the tag details.
    async fn update_tag(&self, tag: Tag, name: String, description: String) -> CommandResult<()>;

    /// Deletes the tag.
    async fn delete_tag(&self, tag: Tag) -> CommandResult<()>;
}
