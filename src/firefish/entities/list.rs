use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List {
    id: String,
    created_at: DateTime<Utc>,
    name: String,
    user_ids: Vec<String>,
}
