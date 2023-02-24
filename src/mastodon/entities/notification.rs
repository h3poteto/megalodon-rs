use super::{Account, Status};
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

impl Into<MegalodonEntities::notification::NotificationType> for NotificationType {
    fn into(self) -> MegalodonEntities::notification::NotificationType {
        match self {
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
            NotificationType::Status => MegalodonEntities::notification::NotificationType::Status,
        }
    }
}

impl Into<MegalodonEntities::Notification> for Notification {
    fn into(self) -> MegalodonEntities::Notification {
        MegalodonEntities::Notification {
            account: self.account.into(),
            created_at: self.created_at,
            id: self.id,
            status: self.status.map(|i| i.into()),
            emoji: None,
            r#type: self.r#type.into(),
        }
    }
}
