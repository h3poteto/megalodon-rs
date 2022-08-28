use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: DateTime<Utc>,
}
