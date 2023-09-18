use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::UserDetail;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mute {
    id: String,
    created_at: DateTime<Utc>,
    mutee_id: String,
    pub mutee: UserDetail,
}
