use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PollOption {
    title: String,
    votes_count: Option<u32>,
}

impl From<PollOption> for MegalodonEntities::PollOption {
    fn from(val: PollOption) -> Self {
        MegalodonEntities::PollOption {
            title: val.title,
            votes_count: val.votes_count,
        }
    }
}
