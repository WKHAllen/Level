use super::AccountType;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of an account in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Account {
    /// The account's identifier.
    pub id: String,
    /// The account type.
    pub account_type: String,
    /// The name of the account.
    pub name: String,
    /// A description of the account.
    pub description: Option<String>,
    /// When the account was created.
    pub created_at: NaiveDateTime,
    /// When the account was last edited.
    pub edited_at: Option<NaiveDateTime>,
    /// When the account was last reconciled.
    pub reconciled_at: Option<NaiveDateTime>,
}

impl Account {
    /// Gets the account type.
    pub fn get_account_type(&self) -> AccountType {
        AccountType::from_internal_name(&self.account_type).unwrap()
    }
}
