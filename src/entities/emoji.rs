use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Emoji {
    pub shortcode: String,
    pub static_url: String,
    pub url: String,
    pub visible_in_picker: bool,
    pub category: Option<String>,
}
