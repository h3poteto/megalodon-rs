use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    name: String,
    url: String,
    following: Option<bool>,
}

impl Into<MegalodonEntities::Tag> for Tag {
    fn into(self) -> MegalodonEntities::Tag {
        MegalodonEntities::Tag {
            name: self.name,
            url: self.url,
            history: [].to_vec(),
            following: self.following,
        }
    }
}
