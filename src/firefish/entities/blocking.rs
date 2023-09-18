use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::UserDetail;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Blocking {
    id: String,
    created_at: DateTime<Utc>,
    blockee_id: String,
    pub blockee: UserDetail,
}
