use super::History;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Tag {
    name: String,
    url: String,
    history: Vec<History>,
    following: Option<bool>,
}

impl Into<MegalodonEntities::Tag> for Tag {
    fn into(self) -> MegalodonEntities::Tag {
        MegalodonEntities::Tag {
            name: self.name,
            url: self.url,
            history: self.history.into_iter().map(|j| j.into()).collect(),
            following: self.following,
        }
    }
}
