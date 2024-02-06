use super::Account;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub name: String,
    pub accounts: Option<Vec<Account>>,
    pub account_ids: Option<Vec<String>>,
    pub url: Option<String>,
}

impl Into<MegalodonEntities::Reaction> for Reaction {
    fn into(self) -> MegalodonEntities::Reaction {
        MegalodonEntities::Reaction {
            count: self.count,
            me: self.me,
            name: self.name,
            url: self.url.clone(),
            static_url: self.url,
            account_ids: self.account_ids,
            accounts: self
                .accounts
                .clone()
                .map(|i| i.into_iter().map(|a| a.into()).collect()),
        }
    }
}
