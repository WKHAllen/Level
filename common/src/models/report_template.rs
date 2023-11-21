use chrono::NaiveDateTime;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

/// A representation of a report template in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ReportTemplate {
    /// The report template's identifier.
    pub id: String,
    /// The name of the report template.
    pub name: String,
    /// A description of the report template.
    pub description: Option<String>,
    /// The dynamic report template structure, serialized as a String.
    pub data: String,
    /// When the report template was created.
    pub created_at: NaiveDateTime,
}

impl ReportTemplate {
    /// Gets the deserialized template data. This can fail if deserialization fails.
    pub fn get_data<T>(&self) -> Result<T, serde_json::Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str(&self.data)
    }
}
