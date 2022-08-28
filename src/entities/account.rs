use super::{Emoji, Field, Source};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    created_at: DateTime<Utc>,
    followers_count: usize,
    following_count: usize,
    statuses_count: usize,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    emojis: Vec<Emoji>,
    moved: Option<Box<Account>>,
    fields: Option<Vec<Field>>,
    bot: Option<bool>,
    source: Option<Source>,
}
