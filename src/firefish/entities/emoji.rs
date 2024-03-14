use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    pub(crate) name: String,
    pub(crate) url: String,
    pub(crate) category: Option<String>,
}

impl From<Emoji> for MegalodonEntities::Emoji {
    fn from(val: Emoji) -> Self {
        MegalodonEntities::Emoji {
            shortcode: val.name,
            static_url: val.url.clone(),
            url: val.url,
            visible_in_picker: true,
            category: val.category,
        }
    }
}
