use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of a subcategory in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Subcategory {
    /// The subcategory's identifier.
    pub id: String,
    /// The ID of the category in which the subcategory exists.
    pub category_id: String,
    /// The name of the category.
    pub name: String,
    /// A description of the category.
    pub description: Option<String>,
    /// When the category was created.
    pub created_at: NaiveDateTime,
}
