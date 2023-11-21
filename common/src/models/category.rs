use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of a category in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Category {
    /// The category's identifier.
    pub id: String,
    /// The name of the category.
    pub name: String,
    /// A description of the category.
    pub description: Option<String>,
    /// When the category was created.
    pub created_at: NaiveDateTime,
}
