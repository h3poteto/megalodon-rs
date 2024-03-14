use super::{status, Emoji};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Announcement {
    id: String,
    content: String,
    starts_at: Option<DateTime<Utc>>,
    ends_at: Option<DateTime<Utc>>,
    published: bool,
    all_day: bool,
    published_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    mentions: Vec<Account>,
    statuses: Vec<Status>,
    tags: Vec<status::Tag>,
    emojis: Vec<Emoji>,
    reactions: Vec<Reaction>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Account {
    id: String,
    username: String,
    url: String,
    acct: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Status {
    id: String,
    url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Reaction {
    name: String,
    count: i64,
    me: Option<bool>,
    url: Option<String>,
    static_url: Option<String>,
}

impl From<Announcement> for MegalodonEntities::Announcement {
    fn from(val: Announcement) -> Self {
        MegalodonEntities::Announcement {
            id: val.id,
            content: val.content,
            starts_at: val.starts_at,
            ends_at: val.ends_at,
            published: val.published,
            all_day: val.all_day,
            published_at: val.published_at,
            updated_at: Some(val.updated_at),
            read: None,
            mentions: val.mentions.into_iter().map(|i| i.into()).collect(),
            statuses: val.statuses.into_iter().map(|i| i.into()).collect(),
            tags: val.tags.into_iter().map(|i| i.into()).collect(),
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            reactions: val.reactions.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<Account> for MegalodonEntities::announcement::Account {
    fn from(val: Account) -> Self {
        MegalodonEntities::announcement::Account {
            id: val.id,
            username: val.username,
            url: val.url,
            acct: val.acct,
        }
    }
}

impl From<Status> for MegalodonEntities::announcement::Status {
    fn from(val: Status) -> Self {
        MegalodonEntities::announcement::Status {
            id: val.id,
            url: val.url,
        }
    }
}

impl From<Reaction> for MegalodonEntities::announcement::Reaction {
    fn from(val: Reaction) -> Self {
        MegalodonEntities::announcement::Reaction {
            name: val.name,
            count: val.count,
            me: val.me,
            url: val.url,
            static_url: val.static_url,
        }
    }
}
