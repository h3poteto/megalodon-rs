use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    pub shortcode: String,
    pub static_url: String,
    pub url: String,
    pub visible_in_picker: bool,
}
