use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    name: String,
    value: String,
    verified_at: DateTime<Utc>,
}
