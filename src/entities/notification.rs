use super::{Account, Reaction, Status};
use crate::error::{Error, Kind};
use chrono::{DateTime, Utc};
use core::str::FromStr;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Notification {
    pub account: Option<Account>,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub status: Option<Status>,
    pub emoji: Option<String>,
    pub reaction: Option<Reaction>,
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
    // EmojiReaction contains only emoji as string.
    EmojiReaction,
    // Reaction contains reaction object instead of emoji.
    Reaction,
    Update,
    Move,
    AdminSignup,
    AdminReport,
    GroupInvited,
    App,
    Unknown,
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
            NotificationType::Reaction => write!(f, "reaction"),
            NotificationType::Update => write!(f, "update"),
            NotificationType::Move => write!(f, "move"),
            NotificationType::AdminSignup => write!(f, "admin.sign_up"),
            NotificationType::AdminReport => write!(f, "admin.report"),
            NotificationType::GroupInvited => write!(f, "group_invited"),
            NotificationType::App => write!(f, "app"),
            NotificationType::Unknown => write!(f, "unknown"),
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
            "reaction" => Ok(NotificationType::Reaction),
            "update" => Ok(NotificationType::Update),
            "move" => Ok(NotificationType::Move),
            "admin.sign_up" => Ok(NotificationType::AdminSignup),
            "admin.report" => Ok(NotificationType::AdminReport),
            "group_invited" => Ok(NotificationType::GroupInvited),
            "app" => Ok(NotificationType::App),
            "unknown" => Ok(NotificationType::Unknown),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}
