use super::{status, Emoji};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Announcement {
    pub id: String,
    pub content: String,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub published: bool,
    pub all_day: bool,
    pub published_at: DateTime<Utc>,
    // Firefish provides updated_at as optional.
    pub updated_at: Option<DateTime<Utc>>,
    pub read: Option<bool>,
    pub mentions: Vec<Account>,
    pub statuses: Vec<Status>,
    pub tags: Vec<status::Tag>,
    pub emojis: Vec<Emoji>,
    pub reactions: Vec<Reaction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub id: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Reaction {
    pub name: String,
    pub count: i64,
    pub me: Option<bool>,
    pub url: Option<String>,
    pub static_url: Option<String>,
}
