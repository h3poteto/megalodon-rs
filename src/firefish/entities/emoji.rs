use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    name: String,
    url: String,
    category: Option<String>,
}

impl Into<MegalodonEntities::Emoji> for Emoji {
    fn into(self) -> MegalodonEntities::Emoji {
        MegalodonEntities::Emoji {
            shortcode: self.name,
            static_url: self.url.clone(),
            url: self.url,
            visible_in_picker: true,
            category: self.category,
        }
    }
}
