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
    discoverable: Option<bool>,
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
    moved: Option<Box<Account>>,
    fields: Vec<Field>,
    bot: bool,
    source: Source,
}

impl From<MegalodonEntities::Account> for Account {
    fn from(item: MegalodonEntities::Account) -> Self {
        let mut moved_account: Option<Box<Account>> = None;
        if let Some(moved) = item.moved {
            let ma: MegalodonEntities::Account = *moved;
            moved_account = Some(Box::new(ma.into()));
        }
        Self {
            id: item.id,
            username: item.username,
            acct: item.acct,
            display_name: item.display_name,
            locked: item.locked,
            discoverable: item.discoverable,
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
            moved: moved_account,
            fields: item.fields.into_iter().map(|j| j.into()).collect(),
            bot: item.bot,
            source: item.source.into(),
        }
    }
}

impl Into<MegalodonEntities::Account> for Account {
    fn into(self) -> MegalodonEntities::Account {
        let mut moved_account: Option<Box<MegalodonEntities::Account>> = None;
        if let Some(moved) = self.moved {
            let ma: Account = *moved;
            moved_account = Some(Box::new(ma.into()));
        }
        MegalodonEntities::Account {
            id: self.id,
            username: self.username,
            acct: self.acct,
            display_name: self.display_name,
            locked: self.locked,
            discoverable: self.discoverable,
            group: None,
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
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
            moved: moved_account,
            fields: self.fields.into_iter().map(|j| j.into()).collect(),
            bot: self.bot,
            source: self.source.into(),
        }
    }
}

impl Into<megalodon::FollowRequest> for Account {
    fn into(self) -> megalodon::FollowRequest {
        megalodon::FollowRequest::Account(self.into())
    }
}
