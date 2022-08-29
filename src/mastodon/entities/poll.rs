use super::PollOption;
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Poll {
    id: String,
    expires_at: Option<DateTime<Utc>>,
    expired: bool,
    multiple: bool,
    votes_count: bool,
    options: Vec<PollOption>,
    voted: bool,
}

impl Into<MegalodonEntities::Poll> for Poll {
    fn into(self) -> MegalodonEntities::Poll {
        MegalodonEntities::Poll {
            id: self.id,
            expires_at: self.expires_at,
            expired: self.expired,
            multiple: self.multiple,
            votes_count: self.votes_count,
            options: self.options.into_iter().map(|i| i.into()).collect(),
            voted: self.voted,
        }
    }
}
