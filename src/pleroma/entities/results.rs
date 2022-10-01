use super::{Account, Status, Tag};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Results {
    accounts: Vec<Account>,
    statuses: Vec<Status>,
    hashtags: Vec<Tag>,
}

impl Into<MegalodonEntities::Results> for Results {
    fn into(self) -> MegalodonEntities::Results {
        MegalodonEntities::Results {
            accounts: self.accounts.into_iter().map(|i| i.into()).collect(),
            statuses: self.statuses.into_iter().map(|i| i.into()).collect(),
            hashtags: self.hashtags.into_iter().map(|i| i.into()).collect(),
        }
    }
}
