use super::{Account, Emoji, Status};
use crate::error::Error;
use chrono::{DateTime, Utc};
use core::str::FromStr;
use serde::{de, ser};
use std::fmt;

pub struct Notification {
    pub account: Account,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub status: Option<Status>,
    pub emoji: Option<Emoji>,
    pub r#type: NotificationType,
}

#[derive(Debug, Clone)]
pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    Status,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reblog => write!(f, "reblog"),
            NotificationType::Favourite => write!(f, "favourite"),
            NotificationType::Poll => write!(f, "poll"),
            NotificationType::FollowRequest => write!(f, "follow_request"),
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
            "poll" => Ok(NotificationType::Poll),
            "follow_request" => Ok(NotificationType::FollowRequest),
            "status" => Ok(NotificationType::Status),
            _ => Err(Error::new(s.to_owned())),
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
