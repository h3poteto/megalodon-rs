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
    read: Option<bool>,
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

impl Into<MegalodonEntities::Announcement> for Announcement {
    fn into(self) -> MegalodonEntities::Announcement {
        MegalodonEntities::Announcement {
            id: self.id,
            content: self.content,
            starts_at: self.starts_at,
            ends_at: self.ends_at,
            published: self.published,
            all_day: self.all_day,
            published_at: self.published_at,
            updated_at: Some(self.updated_at),
            read: self.read,
            mentions: self.mentions.into_iter().map(|i| i.into()).collect(),
            statuses: self.statuses.into_iter().map(|i| i.into()).collect(),
            tags: self.tags.into_iter().map(|i| i.into()).collect(),
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
            reactions: self.reactions.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl Into<MegalodonEntities::announcement::Account> for Account {
    fn into(self) -> MegalodonEntities::announcement::Account {
        MegalodonEntities::announcement::Account {
            id: self.id,
            username: self.username,
            url: self.url,
            acct: self.acct,
        }
    }
}

impl Into<MegalodonEntities::announcement::Status> for Status {
    fn into(self) -> MegalodonEntities::announcement::Status {
        MegalodonEntities::announcement::Status {
            id: self.id,
            url: self.url,
        }
    }
}

impl Into<MegalodonEntities::announcement::Reaction> for Reaction {
    fn into(self) -> MegalodonEntities::announcement::Reaction {
        MegalodonEntities::announcement::Reaction {
            name: self.name,
            count: self.count,
            me: self.me,
            url: self.url,
            static_url: self.static_url,
        }
    }
}
