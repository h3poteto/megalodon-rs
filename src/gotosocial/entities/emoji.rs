use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    shortcode: String,
    static_url: String,
    url: String,
    visible_in_picker: bool,
    category: Option<String>,
}

impl From<MegalodonEntities::Emoji> for Emoji {
    fn from(item: MegalodonEntities::Emoji) -> Self {
        Self {
            shortcode: item.shortcode,
            static_url: item.static_url,
            url: item.url,
            visible_in_picker: item.visible_in_picker,
            category: item.category,
        }
    }
}

impl From<Emoji> for MegalodonEntities::Emoji {
    fn from(item: Emoji) -> MegalodonEntities::Emoji {
        MegalodonEntities::Emoji {
            shortcode: item.shortcode,
            static_url: item.static_url,
            url: item.url,
            visible_in_picker: item.visible_in_picker,
            category: item.category,
        }
    }
}
