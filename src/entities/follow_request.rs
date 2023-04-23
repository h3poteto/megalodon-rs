use super::{Emoji, Field};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FollowRequest {
    pub id: u64,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub bot: bool,
    pub discoverable: Option<bool>,
    pub group: bool,
    pub created_at: DateTime<Utc>,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub followers_count: u32,
    pub following_count: u32,
    pub statuses_count: u32,
    pub emojis: Vec<Emoji>,
    pub fields: Vec<Field>,
}
