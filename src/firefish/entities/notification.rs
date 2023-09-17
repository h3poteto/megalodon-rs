use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::entities as MegalodonEntities;

use super::{Note, User};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    id: String,
    created_at: DateTime<Utc>,
    // is_read: bool,
    r#type: NotificationType,
    // user_id: Option<String>,
    user: Option<User>,
    note: Option<Note>,
    reaction: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Follow,
    Mention,
    Reply,
    Renote,
    Quote,
    Reaction,
    PollVote,
    PollEnded,
    ReceiveFollowRequest,
    FollowRequestAccepted,
    GroupInvited,
    App,
}

impl Into<MegalodonEntities::notification::NotificationType> for NotificationType {
    fn into(self) -> MegalodonEntities::notification::NotificationType {
        match self {
            NotificationType::Follow => MegalodonEntities::notification::NotificationType::Follow,
            NotificationType::Mention => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Reply => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Renote => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Quote => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Reaction => {
                MegalodonEntities::notification::NotificationType::EmojiReaction
            }
            NotificationType::PollVote => {
                MegalodonEntities::notification::NotificationType::PollVote
            }
            NotificationType::PollEnded => {
                MegalodonEntities::notification::NotificationType::PollExpired
            }
            NotificationType::ReceiveFollowRequest => {
                MegalodonEntities::notification::NotificationType::FollowRequest
            }
            NotificationType::FollowRequestAccepted => {
                MegalodonEntities::notification::NotificationType::Follow
            }
            NotificationType::GroupInvited => {
                MegalodonEntities::notification::NotificationType::GroupInvited
            }
            NotificationType::App => MegalodonEntities::notification::NotificationType::App,
        }
    }
}

impl Into<MegalodonEntities::Notification> for Notification {
    fn into(self) -> MegalodonEntities::Notification {
        MegalodonEntities::Notification {
            account: self.user.map(|u| u.into()),
            created_at: self.created_at,
            id: self.id,
            status: self.note.map(|n| n.into()),
            emoji: Some(self.reaction),
            target: None,
            r#type: self.r#type.into(),
        }
    }
}
