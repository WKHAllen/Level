use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of a link between transactions and tags in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct AccountTransactionTag {
    /// The ID of the account transaction.
    pub account_transaction_id: String,
    /// The ID of the tag.
    pub tag_id: String,
    /// When the account transaction tag was created.
    pub created_at: NaiveDateTime,
}
