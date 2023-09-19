use serde::Deserialize;

use super::UserDetail;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Follow {
    // id: String,
    // created_at: DateTime<Utc>,
    // followee_id: String,
    // follower_id: String,
    pub follower: UserDetail,
    pub followee: UserDetail,
}
