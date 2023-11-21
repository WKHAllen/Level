use super::Timeframe;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of a reminder in the database.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Reminder {
    /// The reminder's identifier.
    pub id: String,
    /// The ID of the account the reminder is associated with.
    pub account_id: String,
    /// The note associated with the reminder.
    pub note: Option<String>,
    /// The reminder timeframe.
    pub timeframe: String,
    /// The time offset of the reminder.
    pub timeframe_offset: NaiveDateTime,
    /// When the reminder was created.
    pub created_at: NaiveDateTime,
}

impl Reminder {
    /// Gets the timeframe.
    pub fn get_timeframe(&self) -> Timeframe {
        Timeframe::from_internal_name(&self.timeframe).unwrap()
    }
}
