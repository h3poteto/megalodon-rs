use super::{Account, Status, Tag};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Results {
    accounts: Vec<Account>,
    statuses: Vec<Status>,
    hashtags: Vec<Tag>,
}

impl From<Results> for MegalodonEntities::Results {
    fn from(val: Results) -> Self {
        MegalodonEntities::Results {
            accounts: val.accounts.into_iter().map(|i| i.into()).collect(),
            statuses: val.statuses.into_iter().map(|i| i.into()).collect(),
            hashtags: val.hashtags.into_iter().map(|i| i.into()).collect(),
        }
    }
}
