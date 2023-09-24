use super::Account;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Reaction {
    count: u32,
    me: bool,
    name: String,
    accounts: Option<Vec<Account>>,
}

impl Into<MegalodonEntities::Reaction> for Reaction {
    fn into(self) -> MegalodonEntities::Reaction {
        MegalodonEntities::Reaction {
            count: self.count,
            me: self.me,
            name: self.name,
            url: None,
            static_url: None,
            accounts: self
                .accounts
                .map(|i| i.into_iter().map(|a| a.into()).collect()),
        }
    }
}
