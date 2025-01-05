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

impl From<Tag> for MegalodonEntities::Tag {
    fn from(val: Tag) -> Self {
        MegalodonEntities::Tag {
            name: val.name,
            url: val.url,
            history: val.history.into_iter().map(|j| j.into()).collect(),
            following: val.following,
        }
    }
}
