use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of a tag on a transaction in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Tag {
    /// The tag's identifier.
    pub id: String,
    /// The name of the tag.
    pub name: String,
    /// A description of the tag.
    pub description: Option<String>,
    /// When the tag was created.
    pub created_at: NaiveDateTime,
}
