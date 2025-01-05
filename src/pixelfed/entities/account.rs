use super::{Emoji, Field, Source};
use crate::{entities as MegalodonEntities, megalodon};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Account {
    id: String,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    followers_count: i32,
    following_count: u32,
    statuses_count: u32,
    note: String,
    url: String,
    avatar: String,
    created_at: DateTime<Utc>,
    avatar_static: String,
    bot: bool,
    emojis: Vec<Emoji>,
    fields: Vec<Field>,
    header: String,
    header_static: String,
    /// CredentialAccount entity attributes
    source: Option<Source>,
}

impl From<MegalodonEntities::Account> for Account {
    fn from(item: MegalodonEntities::Account) -> Self {
        Self {
            id: item.id,
            username: item.username,
            acct: item.acct,
            display_name: item.display_name,
            locked: item.locked,
            created_at: item.created_at,
            followers_count: item.followers_count,
            following_count: item.following_count,
            statuses_count: item.statuses_count,
            note: item.note,
            url: item.url,
            avatar: item.avatar,
            avatar_static: item.avatar_static,
            header: item.header,
            header_static: item.header_static,
            emojis: item.emojis.into_iter().map(|i| i.into()).collect(),
            fields: item.fields.into_iter().map(|j| j.into()).collect(),
            bot: item.bot,
            source: item.source.map(|i| i.into()),
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
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            moved: None,
            fields: val.fields.into_iter().map(|j| j.into()).collect(),
            bot: val.bot,
            source: val.source.map(|i| i.into()),
            role: None,
            mute_expires_at: None,
        }
    }
}

impl From<Account> for megalodon::FollowRequestOutput {
    fn from(val: Account) -> Self {
        megalodon::FollowRequestOutput::Account(val.into())
    }
}
