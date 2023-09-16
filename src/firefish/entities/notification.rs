use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Note, User};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    id: String,
    created_at: DateTime<Utc>,
    is_read: bool,
    r#type: NotificationType,
    user_id: Option<String>,
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
