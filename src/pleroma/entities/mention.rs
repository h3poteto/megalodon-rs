use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Mention {
    id: String,
    username: String,
    url: String,
    acct: String,
}

impl From<Mention> for MegalodonEntities::Mention {
    fn from(val: Mention) -> Self {
        MegalodonEntities::Mention {
            id: val.id,
            username: val.username,
            url: val.url,
            acct: val.acct,
        }
    }
}
