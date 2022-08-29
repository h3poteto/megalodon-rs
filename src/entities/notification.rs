use super::{Account, Emoji, Status};
use chrono::{DateTime, Utc};

pub struct Notification {
    pub account: Account,
    pub created_at: DateTime<Utc>,
    pub id: String,
    pub status: Option<Status>,
    pub emoji: Option<Emoji>,
    pub r#type: NotificationType,
}

pub enum NotificationType {
    Follow,
    FollowRequest,
    Mention,
    Reblog,
    Favourite,
    Poll,
    Status,
}
