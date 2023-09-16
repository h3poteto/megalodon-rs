use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::User;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    id: String,
    created_at: DateTime<Utc>,
    user: User,
    r#type: String,
}
