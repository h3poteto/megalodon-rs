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

impl From<Conversation> for MegalodonEntities::Conversation {
    fn from(val: Conversation) -> Self {
        MegalodonEntities::Conversation {
            id: val.id,
            accounts: val.accounts.into_iter().map(|i| i.into()).collect(),
            last_status: val.last_status.map(|i| i.into()),
            unread: val.unread,
        }
    }
}
