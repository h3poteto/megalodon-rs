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

impl Into<MegalodonEntities::Emoji> for Emoji {
    fn into(self) -> MegalodonEntities::Emoji {
        MegalodonEntities::Emoji {
            shortcode: self.shortcode,
            static_url: self.static_url,
            url: self.url,
            visible_in_picker: self.visible_in_picker,
            category: None,
        }
    }
}

impl Into<MegalodonEntities::Account> for Account {
    fn into(self) -> MegalodonEntities::Account {
        MegalodonEntities::Account {
            id: self.id,
            username: self.username,
            acct: self.acct,
            display_name: self.display_name,
            locked: self.locked,
            discoverable: None,
            group: None,
            noindex: None,
            moved: None,
            suspended: None,
            limited: None,
            created_at: self.created_at,
            followers_count: self.followers_count,
            following_count: self.following_count,
            statuses_count: self.statuses_count,
            note: self.note,
            url: self.url,
            avatar: self.avatar,
            avatar_static: self.avatar_static,
            header: self.header,
            header_static: self.header_static,
            emojis: self.emojis.into_iter().map(|e| e.into()).collect(),
            fields: self.fields.into_iter().map(|f| f.into()).collect(),
            bot: self.bot,
            source: None,
            role: None,
            mute_expires_at: None,
        }
    }
}
