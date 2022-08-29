use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Mention {
    id: String,
    username: String,
    url: String,
    acct: String,
}

impl Into<MegalodonEntities::Mention> for Mention {
    fn into(self) -> MegalodonEntities::Mention {
        MegalodonEntities::Mention {
            id: self.id,
            username: self.username,
            url: self.url,
            acct: self.acct,
        }
    }
}
