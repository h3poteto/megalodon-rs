use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CustomEmoji {
    name: String,
    category: Option<String>,
    url: String,
}

impl From<CustomEmoji> for MegalodonEntities::Emoji {
    fn from(value: CustomEmoji) -> Self {
        Self {
            shortcode: value.name,
            static_url: value.url.clone(),
            url: value.url,
            visible_in_picker: true,
            category: value.category,
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct CustomEmojiResponse {
    pub emojis: Vec<CustomEmoji>,
}
