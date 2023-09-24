use super::Emoji;
use crate::entities as MegalodonEntities;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    // id: String,
    // created_at: DateTime<Utc>,
    // user: User,
    // r#type: String,
}

pub(crate) fn map_reaction(
    emojis: Vec<Emoji>,
    reactions: HashMap<String, u32>,
    my_reaction: Option<String>,
) -> Vec<MegalodonEntities::Reaction> {
    let emoji_urls =
        HashMap::<String, String>::from_iter(emojis.into_iter().map(|e| (e.name, e.url)));
    reactions
        .clone()
        .into_iter()
        .map(|(key, _value)| {
            let shortcode = key.replace(":", "");
            let url = emoji_urls.get::<String>(&shortcode).map(|u| u.to_string());
            let name = shortcode.replace("@.", "");
            let me = if let Some(my) = &my_reaction {
                key == my.clone()
            } else {
                false
            };
            MegalodonEntities::Reaction {
                count: reactions[&key],
                me,
                name,
                url: url.clone(),
                static_url: url,
                accounts: None,
            }
        })
        .collect()
}
