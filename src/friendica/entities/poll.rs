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

impl Into<MegalodonEntities::Poll> for Poll {
    fn into(self) -> MegalodonEntities::Poll {
        MegalodonEntities::Poll {
            id: self.id,
            expires_at: self.expires_at,
            expired: self.expired,
            multiple: self.multiple,
            votes_count: self.votes_count,
            voters_count: self.voters_count,
            options: self.options.into_iter().map(|i| i.into()).collect(),
            voted: self.voted,
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
        }
    }
}
