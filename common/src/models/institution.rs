use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of an institution in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Institution {
    /// The institution's identifier.
    pub id: String,
    /// The name of the institution.
    pub name: String,
    /// A description of the institution.
    pub description: Option<String>,
    /// When the institution was created.
    pub created_at: NaiveDateTime,
}
