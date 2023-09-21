use serde::Deserialize;

use super::Emoji;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub emojis: Vec<Emoji>,
}
