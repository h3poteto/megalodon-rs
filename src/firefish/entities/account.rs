use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::Field;
use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    created_at: DateTime<Utc>,
    followers_count: u32,
    following_count: u32,
    statuses_count: u32,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    emojis: Vec<Emoji>,
    fields: Vec<Field>,
    bot: bool,
}

#[derive(Debug, Deserialize, Clone)]
struct Emoji {
    shortcode: String,
    static_url: String,
    url: String,
    visible_in_picker: bool,
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

impl From<Account> for MegalodonEntities::Account {
    fn from(val: Account) -> Self {
        MegalodonEntities::Account {
            id: val.id,
            username: val.username,
            acct: val.acct,
            display_name: val.display_name,
            locked: val.locked,
            discoverable: None,
            group: None,
            noindex: None,
            moved: None,
            suspended: None,
            limited: None,
            created_at: val.created_at,
            followers_count: val.followers_count,
            following_count: val.following_count,
            statuses_count: val.statuses_count,
            note: val.note,
            url: val.url,
            avatar: val.avatar,
            avatar_static: val.avatar_static,
            header: val.header,
            header_static: val.header_static,
            emojis: val.emojis.into_iter().map(|e| e.into()).collect(),
            fields: val.fields.into_iter().map(|f| f.into()).collect(),
            bot: val.bot,
            source: None,
            role: None,
            mute_expires_at: None,
        }
    }
}
