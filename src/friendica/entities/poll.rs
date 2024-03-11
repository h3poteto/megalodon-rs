use super::{Emoji, PollOption};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Poll {
    id: String,
    expires_at: Option<DateTime<Utc>>,
    expired: bool,
    multiple: bool,
    votes_count: u32,
    voters_count: Option<u32>,
    options: Vec<PollOption>,
    voted: Option<bool>,
    emojis: Vec<Emoji>,
}

impl From<Poll> for MegalodonEntities::Poll {
    fn from(val: Poll) -> Self {
        MegalodonEntities::Poll {
            id: val.id,
            expires_at: val.expires_at,
            expired: val.expired,
            multiple: val.multiple,
            votes_count: val.votes_count,
            voters_count: val.voters_count,
            options: val.options.into_iter().map(|i| i.into()).collect(),
            voted: val.voted,
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
        }
    }
}
