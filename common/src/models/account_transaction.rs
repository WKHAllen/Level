use super::TransactionType;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

/// A representation of an account transaction in the database.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct AccountTransaction {
    /// The account transaction's identifier.
    pub id: String,
    /// The ID of the account which the transaction is associated with.
    pub account_id: String,
    /// The name of the account.
    pub name: String,
    /// A description of the account.
    pub description: Option<String>,
    /// The monetary amount of the transaction.
    pub amount: f64,
    /// The type of transaction.
    pub transaction_type: String,
    /// The ID of the institution which the transaction is associated with.
    pub institution_id: String,
    /// The date of the transaction.
    pub transaction_date: NaiveDateTime,
    /// The ID of the category in which the transaction exists.
    pub category_id: String,
    /// The ID of the subcategory in which the transaction exists.
    pub subcategory_id: Option<String>,
    /// Whether the transaction has been reconciled.
    pub reconciled: bool,
    /// When the transaction was created.
    pub created_at: NaiveDateTime,
    /// When the transaction was last edited.
    pub edited_at: Option<NaiveDateTime>,
    /// When the transaction was last reconciled.
    pub reconciled_at: Option<NaiveDateTime>,
}

impl AccountTransaction {
    /// Gets the type of the transaction.
    pub fn get_transaction_type(&self) -> TransactionType {
        TransactionType::from_internal_name(&self.transaction_type).unwrap()
    }

    /// Gets the date the transaction took place.
    pub fn get_date(&self) -> NaiveDate {
        self.transaction_date.date()
    }
}
