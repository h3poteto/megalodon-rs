use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    shortcode: String,
    static_url: String,
    url: String,
    visible_in_picker: bool,
}

impl From<MegalodonEntities::Emoji> for Emoji {
    fn from(item: MegalodonEntities::Emoji) -> Self {
        Self {
            shortcode: item.shortcode,
            static_url: item.static_url,
            url: item.url,
            visible_in_picker: item.visible_in_picker,
        }
    }
}

impl From<Emoji> for MegalodonEntities::Emoji {
    fn from(val: Emoji) -> Self {
        MegalodonEntities::Emoji {
            shortcode: val.shortcode,
            static_url: val.static_url,
            url: val.url,
            visible_in_picker: val.visible_in_picker,
            category: None,
        }
    }
}
