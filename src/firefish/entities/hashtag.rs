use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hashtag {
    pub tag: String,
}

impl From<Hashtag> for MegalodonEntities::Tag {
    fn from(val: Hashtag) -> Self {
        MegalodonEntities::Tag {
            name: val.tag.clone(),
            url: val.tag,
            history: [].to_vec(),
            following: None,
        }
    }
}
