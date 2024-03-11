use super::{Account, Status};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Notification {
    account: Account,
    created_at: DateTime<Utc>,
    id: String,
    status: Option<Status>,
    r#type: NotificationType,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    Status,
    Update,
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
            NotificationType::Status => MegalodonEntities::notification::NotificationType::Status,
            NotificationType::Update => MegalodonEntities::notification::NotificationType::Update,
        }
    }
}

impl From<Notification> for MegalodonEntities::Notification {
    fn from(val: Notification) -> Self {
        MegalodonEntities::Notification {
            account: Some(val.account.into()),
            created_at: val.created_at,
            id: val.id,
            status: val.status.map(|i| i.into()),
            reaction: None,
            target: None,
            r#type: val.r#type.into(),
        }
    }
}
