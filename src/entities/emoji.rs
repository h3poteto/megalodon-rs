use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji {
    shortcode: String,
    static_url: String,
    url: String,
    visible_in_picker: bool,
}
