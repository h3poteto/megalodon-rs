use super::{Account, Reaction, Status};
use crate::error::{Error, Kind};

use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use core::str::FromStr;
use serde::{de, ser, Deserialize};
use std::fmt;

#[derive(Debug, Deserialize, Clone)]
pub struct Notification {
    account: Account,
    created_at: DateTime<Utc>,
    id: String,
    status: Option<Status>,
    r#type: NotificationType,
    emoji: Option<String>,
    emoji_url: Option<String>,
    target: Option<Account>,
}

impl Notification {
    fn map_reaction(&self) -> Option<Reaction> {
        let shortcode = self.emoji.clone()?;
        let name = shortcode.replace(":", "");
        Some(Reaction {
            count: 1,
            me: false,
            name,
            url: self.emoji_url.clone(),
            accounts: None,
            account_ids: None,
        })
    }
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    PleromaEmojiReaction,
    Update,
    Move,
    Status,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reblog => write!(f, "reblog"),
            NotificationType::Favourite => write!(f, "favourite"),
            NotificationType::PleromaEmojiReaction => write!(f, "pleroma:emoji_reaction"),
            NotificationType::Poll => write!(f, "poll"),
            NotificationType::FollowRequest => write!(f, "follow_request"),
            NotificationType::Update => write!(f, "update"),
            NotificationType::Move => write!(f, "move"),
            NotificationType::Status => write!(f, "status"),
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
            "pleroma:emoji_reaction" => Ok(NotificationType::PleromaEmojiReaction),
            "poll" => Ok(NotificationType::Poll),
            "follow_request" => Ok(NotificationType::FollowRequest),
            "update" => Ok(NotificationType::Update),
            "move" => Ok(NotificationType::Move),
            "status" => Ok(NotificationType::Status),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

impl ser::Serialize for NotificationType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for NotificationType {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match NotificationType::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl From<NotificationType> for MegalodonEntities::notification::NotificationType {
    fn from(val: NotificationType) -> Self {
        match val {
            NotificationType::Follow => MegalodonEntities::notification::NotificationType::Follow,
            NotificationType::FollowRequest => {
                MegalodonEntities::notification::NotificationType::FollowRequest
            }
            NotificationType::Mention => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Reblog => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Favourite => {
                MegalodonEntities::notification::NotificationType::Favourite
            }
            NotificationType::Poll => {
                MegalodonEntities::notification::NotificationType::PollExpired
            }
            NotificationType::PleromaEmojiReaction => {
                MegalodonEntities::notification::NotificationType::Reaction
            }
            NotificationType::Update => MegalodonEntities::notification::NotificationType::Update,
            NotificationType::Move => MegalodonEntities::notification::NotificationType::Move,
            NotificationType::Status => MegalodonEntities::notification::NotificationType::Status,
        }
    }
}

impl From<Notification> for MegalodonEntities::Notification {
    fn from(val: Notification) -> Self {
        let reaction = val.clone().map_reaction();
        MegalodonEntities::Notification {
            account: Some(val.account.into()),
            created_at: val.created_at,
            id: val.id,
            status: val.status.map(|i| i.into()),
            reaction: reaction.map(|i| i.into()),
            target: val.target.map(|i| i.into()),
            r#type: val.r#type.into(),
        }
    }
}
