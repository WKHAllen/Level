use super::Timeframe;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// A representation of an account budget in the database.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Budget {
    /// The ID of the account the budget is associated with.
    pub account_id: String,
    /// The note associated with the budget.
    pub note: Option<String>,
    /// The budget limit.
    pub total_limit: f64,
    /// The budget timeframe.
    pub timeframe: String,
    /// The time offset of the budget.
    pub timeframe_offset: NaiveDateTime,
    /// When the budget was created.
    pub created_at: NaiveDateTime,
}

impl Budget {
    /// Gets the timeframe.
    pub fn get_timeframe(&self) -> Timeframe {
        Timeframe::from_internal_name(&self.timeframe).unwrap()
    }
}
