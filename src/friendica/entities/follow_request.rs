use super::{Emoji, Field};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct FollowRequest {
    id: u64,
    username: String,
    acct: String,
    display_name: String,
    locked: bool,
    bot: bool,
    discoverable: Option<bool>,
    group: bool,
    created_at: DateTime<Utc>,
    note: String,
    url: String,
    avatar: String,
    avatar_static: String,
    header: String,
    header_static: String,
    followers_count: u32,
    following_count: u32,
    statuses_count: u32,
    emojis: Vec<Emoji>,
    fields: Vec<Field>,
}

impl From<MegalodonEntities::FollowRequest> for FollowRequest {
    fn from(item: MegalodonEntities::FollowRequest) -> Self {
        Self {
            id: item.id,
            username: item.username,
            acct: item.acct,
            display_name: item.display_name,
            locked: item.locked,
            bot: item.bot,
            discoverable: item.discoverable,
            group: item.group,
            created_at: item.created_at,
            note: item.note,
            url: item.url,
            avatar: item.avatar,
            avatar_static: item.avatar_static,
            header: item.header,
            header_static: item.header_static,
            followers_count: item.followers_count,
            following_count: item.following_count,
            statuses_count: item.statuses_count,
            emojis: item.emojis.into_iter().map(|i| i.into()).collect(),
            fields: item.fields.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl Into<MegalodonEntities::FollowRequest> for FollowRequest {
    fn into(self) -> MegalodonEntities::FollowRequest {
        MegalodonEntities::FollowRequest {
            id: self.id,
            username: self.username,
            acct: self.acct,
            display_name: self.display_name,
            locked: self.locked,
            bot: self.bot,
            discoverable: self.discoverable,
            group: self.group,
            created_at: self.created_at,
            note: self.note,
            url: self.url,
            avatar: self.avatar,
            avatar_static: self.avatar_static,
            header: self.header,
            header_static: self.header_static,
            followers_count: self.followers_count,
            following_count: self.following_count,
            statuses_count: self.statuses_count,
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
            fields: self.fields.into_iter().map(|i| i.into()).collect(),
        }
    }
}
