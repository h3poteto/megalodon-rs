use super::{Account, Emoji, Status};
use chrono::{DateTime, Utc};

pub struct Notification {
    account: Account,
    created_at: DateTime<Utc>,
    id: String,
    status: Option<Status>,
    emoji: Option<Emoji>,
    r#type: NotificationType,
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
