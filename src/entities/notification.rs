use super::{Account, Status};
use crate::error::{Error, Kind};
use chrono::{DateTime, Utc};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notification {
    pub account: Account,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub status: Option<Status>,
    pub emoji: Option<String>,
    pub target: Option<Account>,
    pub r#type: NotificationType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    PollVote,
    PollExpired,
    Status,
    EmojiReaction,
    Update,
    Move,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reblog => write!(f, "reblog"),
            NotificationType::Favourite => write!(f, "favourite"),
            NotificationType::PollVote => write!(f, "poll_vote"),
            NotificationType::PollExpired => write!(f, "poll_expired"),
            NotificationType::FollowRequest => write!(f, "follow_request"),
            NotificationType::Status => write!(f, "status"),
            NotificationType::EmojiReaction => write!(f, "emoji_reaction"),
            NotificationType::Update => write!(f, "update"),
            NotificationType::Move => write!(f, "move"),
        }
    }
}

impl FromStr for NotificationType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "follow" => Ok(NotificationType::Follow),
            "mention" => Ok(NotificationType::Mention),
            "reblog" => Ok(NotificationType::Reblog),
            "favourite" => Ok(NotificationType::Favourite),
            "poll_vote" => Ok(NotificationType::PollVote),
            "follow_request" => Ok(NotificationType::FollowRequest),
            "status" => Ok(NotificationType::Status),
            "emoji_reaction" => Ok(NotificationType::EmojiReaction),
            "update" => Ok(NotificationType::Update),
            "move" => Ok(NotificationType::Move),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}
