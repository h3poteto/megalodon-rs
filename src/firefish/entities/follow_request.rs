use super::User;
use chrono::Utc;
use serde::Deserialize;

use crate::{entities as MegalodonEntities, megalodon};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    id: String,
    follower: User,
    followee: User,
}

impl Into<MegalodonEntities::FollowRequest> for FollowRequest {
    fn into(self) -> MegalodonEntities::FollowRequest {
        let user = self.follower;
        let mut acct = user.username.clone();
        if let Some(host) = user.host {
            acct = format!("{}@{}", user.username, host);
        }
        let mut display_name = "".to_string();
        if let Some(name) = user.name {
            display_name = name;
        }
        let mut bot = false;
        if let Some(is_bot) = user.is_bot {
            bot = is_bot;
        }
        let mut avatar = "".to_string();
        if let Some(avatar_url) = user.avatar_url {
            avatar = avatar_url;
        }
        let mut avatar_static = "".to_string();
        if let Some(avatar_color) = user.avatar_color {
            avatar_static = avatar_color;
        }
        MegalodonEntities::FollowRequest {
            id: self.id.parse::<u64>().unwrap(),
            username: user.username,
            acct: acct.clone(),
            display_name,
            locked: false,
            bot,
            discoverable: None,
            group: false,
            created_at: Utc::now(),
            note: "".to_string(),
            url: acct,
            avatar,
            avatar_static,
            header: "".to_string(),
            header_static: "".to_string(),
            followers_count: 0,
            following_count: 0,
            statuses_count: 0,
            emojis: user.emojis.into_iter().map(|i| i.into()).collect(),
            fields: [].to_vec(),
        }
    }
}

impl Into<megalodon::FollowRequestOutput> for FollowRequest {
    fn into(self) -> megalodon::FollowRequestOutput {
        megalodon::FollowRequestOutput::FollowRequest(self.into())
    }
}
