use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: Option<DateTime<Utc>>,
    pub verified: Option<bool>,
}
