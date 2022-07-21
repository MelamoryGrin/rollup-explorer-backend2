use chrono::NaiveDateTime;
use serde::Serialize;
use std::fmt;

#[derive(sqlx::Type, Clone, Debug, Serialize)]
#[repr(i32)]
pub enum RollupStatus {
    Pending = 0,
    Committing = 1,
    Committed = 2,
    Finalizing = 3,
    Finalized = 4,
}

impl fmt::Display for RollupStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(sqlx::FromRow, Clone, Debug, Serialize)]
pub struct RollupResult {
    pub number: i32,
    pub status: RollupStatus,
    pub rollup_tx_hash: Option<String>,
    pub finalize_tx_hash: Option<String>,
    pub created_time: NaiveDateTime,
    pub updated_time: NaiveDateTime,
}
