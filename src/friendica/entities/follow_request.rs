use super::{Emoji, Field};
use crate::{entities as MegalodonEntities, megalodon};
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

impl From<FollowRequest> for MegalodonEntities::FollowRequest {
    fn from(val: FollowRequest) -> Self {
        MegalodonEntities::FollowRequest {
            id: val.id,
            username: val.username,
            acct: val.acct,
            display_name: val.display_name,
            locked: val.locked,
            bot: val.bot,
            discoverable: val.discoverable,
            group: val.group,
            created_at: val.created_at,
            note: val.note,
            url: val.url,
            avatar: val.avatar,
            avatar_static: val.avatar_static,
            header: val.header,
            header_static: val.header_static,
            followers_count: val.followers_count,
            following_count: val.following_count,
            statuses_count: val.statuses_count,
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            fields: val.fields.into_iter().map(|i| i.into()).collect(),
        }
    }
}

impl From<FollowRequest> for megalodon::FollowRequestOutput {
    fn from(val: FollowRequest) -> Self {
        megalodon::FollowRequestOutput::FollowRequest(val.into())
    }
}
