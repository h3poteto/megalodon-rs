use super::{Emoji, Field, Source};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub display_name: String,
    pub locked: bool,
    pub created_at: DateTime<Utc>,
    pub followers_count: usize,
    pub following_count: usize,
    pub statuses_count: usize,
    pub note: String,
    pub url: String,
    pub avatar: String,
    pub avatar_static: String,
    pub header: String,
    pub header_static: String,
    pub emojis: Vec<Emoji>,
    pub moved: Option<Box<Account>>,
    pub fields: Option<Vec<Field>>,
    pub bot: Option<bool>,
    pub source: Option<Source>,
}
