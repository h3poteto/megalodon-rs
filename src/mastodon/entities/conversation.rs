use super::{Account, Status};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Conversation {
    id: String,
    accounts: Vec<Account>,
    last_status: Option<Status>,
    unread: bool,
}

impl Into<MegalodonEntities::Conversation> for Conversation {
    fn into(self) -> MegalodonEntities::Conversation {
        MegalodonEntities::Conversation {
            id: self.id,
            accounts: self.accounts.into_iter().map(|i| i.into()).collect(),
            last_status: self.last_status.map(|i| i.into()),
            unread: self.unread,
        }
    }
}
