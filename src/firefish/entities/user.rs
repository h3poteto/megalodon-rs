use super::Emoji;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    name: Option<String>,
    username: String,
    host: Option<String>,
    avatar_url: Option<String>,
    avatar_blurhash: Option<String>,
    avatar_color: Option<String>,
    is_admin: Option<bool>,
    is_moderator: Option<bool>,
    is_bot: Option<bool>,
    is_cat: Option<bool>,
    is_indexable: Option<bool>,
    speak_as_cat: Option<bool>,
    emojis: Vec<Emoji>,
    online_status: Option<String>,
}
