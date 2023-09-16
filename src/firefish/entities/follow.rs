use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::UserDetail;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    id: String,
    created_at: DateTime<Utc>,
    followee_id: String,
    follower_id: String,
    follower: UserDetail,
    followee: UserDetail,
}
