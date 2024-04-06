use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    name: String,
    url: String,
    following: Option<bool>,
}

impl From<Tag> for MegalodonEntities::Tag {
    fn from(val: Tag) -> MegalodonEntities::Tag {
        MegalodonEntities::Tag {
            name: val.name,
            url: val.url,
            history: [].to_vec(),
            following: val.following,
        }
    }
}
