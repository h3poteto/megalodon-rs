use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PollOption {
    title: String,
    votes_count: Option<u32>,
}

impl Into<MegalodonEntities::PollOption> for PollOption {
    fn into(self) -> MegalodonEntities::PollOption {
        MegalodonEntities::PollOption {
            title: self.title,
            votes_count: self.votes_count,
        }
    }
}
