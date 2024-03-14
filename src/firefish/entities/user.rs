use super::Emoji;
use chrono::Utc;
use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub host: Option<String>,
    pub avatar_url: Option<String>,
    pub avatar_blurhash: Option<String>,
    pub avatar_color: Option<String>,
    pub is_admin: Option<bool>,
    pub is_moderator: Option<bool>,
    pub is_bot: Option<bool>,
    pub is_cat: Option<bool>,
    pub is_indexable: Option<bool>,
    pub speak_as_cat: Option<bool>,
    pub emojis: Vec<Emoji>,
    pub online_status: Option<String>,
}

impl From<User> for MegalodonEntities::Account {
    fn from(val: User) -> Self {
        let mut acct = val.username.clone();
        if let Some(host) = val.host {
            acct = format!("{}@{}", val.username, host);
        }
        let mut display_name = "".to_string();
        if let Some(name) = val.name {
            display_name = name;
        }
        let mut avatar = "".to_string();
        if let Some(avatar_url) = val.avatar_url {
            avatar = avatar_url;
        }
        let mut avatar_static = "".to_string();
        if let Some(avatar_color) = val.avatar_color {
            avatar_static = avatar_color;
        }
        let mut bot = false;
        if let Some(is_bot) = val.is_bot {
            bot = is_bot;
        }

        MegalodonEntities::Account {
            id: val.id,
            username: val.username,
            acct: acct.clone(),
            display_name,
            locked: false,
            discoverable: None,
            group: None,
            noindex: val.is_indexable,
            moved: None,
            suspended: None,
            limited: None,
            created_at: Utc::now(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            note: "".to_string(),
            url: acct,
            avatar,
            avatar_static,
            header: "".to_string(),
            header_static: "".to_string(),
            emojis: val.emojis.into_iter().map(|e| e.into()).collect(),
            fields: [].to_vec(),
            bot,
            source: None,
            role: None,
            mute_expires_at: None,
        }
    }
}
