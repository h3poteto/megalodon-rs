use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Emoji {
    shortcode: String,
    static_url: String,
    url: String,
    visible_in_picker: bool,
}
