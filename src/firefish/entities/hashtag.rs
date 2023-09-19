use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Hashtag {
    pub tag: String,
}

impl Into<MegalodonEntities::Tag> for Hashtag {
    fn into(self) -> MegalodonEntities::Tag {
        MegalodonEntities::Tag {
            name: self.tag.clone(),
            url: self.tag,
            history: [].to_vec(),
            following: None,
        }
    }
}
