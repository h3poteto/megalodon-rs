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

impl From<Reaction> for MegalodonEntities::Reaction {
    fn from(val: Reaction) -> Self {
        MegalodonEntities::Reaction {
            count: val.count,
            me: val.me,
            name: val.name,
            url: val.url.clone(),
            static_url: val.url,
            account_ids: val.account_ids,
            accounts: val
                .accounts
                .clone()
                .map(|i| i.into_iter().map(|a| a.into()).collect()),
        }
    }
}
