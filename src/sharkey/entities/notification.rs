use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

use crate::entities as MegalodonEntities;

use super::{Note, User, reaction::map_reaction};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    id: String,
    created_at: DateTime<Utc>,
    // is_read: bool,
    pub r#type: NotificationType,
    // user_id: Option<String>,
    user: Option<User>,
    note: Option<Note>,
    reaction: Option<String>,
}

// https://activitypub.software/TransFem-org/Sharkey/-/blob/develop/packages/backend/src/models/Notification.ts
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum NotificationType {
    Note,
    Follow,
    Mention,
    Reply,
    Renote,
    Quote,
    Reaction,
    PollEnded,
    ReceiveFollowRequest,
    FollowRequestAccepted,
    RoleAssigned,
    ChatRoomInvitationReceived,
    AchievementEarned,
    ExportCompleted,
    ImportCompleted,
    Login,
    CreateToken,
    App,
    Test,
    Edited,
    ScheduledNoteFailed,
    ScheduledNotePosted,
    SharedAccessGranted,
    SharedAccessRevoked,
    SharedAccessLogin,
    Unknown,
}

impl fmt::Display for NotificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NotificationType::Follow => write!(f, "follow"),
            NotificationType::Mention => write!(f, "mention"),
            NotificationType::Reply => write!(f, "reply"),
            NotificationType::Renote => write!(f, "renote"),
            NotificationType::Quote => write!(f, "quote"),
            NotificationType::Reaction => write!(f, "reaction"),
            NotificationType::PollEnded => write!(f, "pollEnded"),
            NotificationType::ReceiveFollowRequest => write!(f, "receiveFollowRequest"),
            NotificationType::FollowRequestAccepted => write!(f, "followRequestAccepted"),
            NotificationType::Note => write!(f, "note"),
            NotificationType::RoleAssigned => write!(f, "roleAssigned"),
            NotificationType::ChatRoomInvitationReceived => {
                write!(f, "chatRoomInvitationReceived")
            }
            NotificationType::AchievementEarned => write!(f, "achievementEarned"),
            NotificationType::ExportCompleted => write!(f, "exportCompleted"),
            NotificationType::ImportCompleted => write!(f, "importCompleted"),
            NotificationType::Login => write!(f, "login"),
            NotificationType::CreateToken => write!(f, "createToken"),
            NotificationType::App => write!(f, "app"),
            NotificationType::Test => write!(f, "test"),
            NotificationType::Edited => write!(f, "edited"),
            NotificationType::ScheduledNoteFailed => write!(f, "scheduledNoteFailed"),
            NotificationType::ScheduledNotePosted => write!(f, "scheduledNotePosted"),
            NotificationType::SharedAccessGranted => write!(f, "sharedAccessGranted"),
            NotificationType::SharedAccessRevoked => write!(f, "sharedAccessRevoked"),
            NotificationType::SharedAccessLogin => write!(f, "sharedAccessLogin"),
            NotificationType::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<MegalodonEntities::notification::NotificationType> for NotificationType {
    fn from(value: MegalodonEntities::notification::NotificationType) -> Self {
        match value {
            MegalodonEntities::notification::NotificationType::Follow => NotificationType::Follow,
            MegalodonEntities::notification::NotificationType::Mention => NotificationType::Mention,
            MegalodonEntities::notification::NotificationType::Reblog => NotificationType::Renote,
            MegalodonEntities::notification::NotificationType::Reaction => {
                NotificationType::Reaction
            }
            MegalodonEntities::notification::NotificationType::PollVote => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::PollExpired => {
                NotificationType::PollEnded
            }
            MegalodonEntities::notification::NotificationType::FollowRequest => {
                NotificationType::ReceiveFollowRequest
            }
            MegalodonEntities::notification::NotificationType::GroupInvited => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::App => NotificationType::App,
            MegalodonEntities::notification::NotificationType::AdminReport => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::AdminSignup => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::Favourite => {
                NotificationType::Unknown
            }
            MegalodonEntities::notification::NotificationType::Move => NotificationType::Unknown,
            MegalodonEntities::notification::NotificationType::Status => NotificationType::Note,
            MegalodonEntities::notification::NotificationType::Unknown => NotificationType::Unknown,
            MegalodonEntities::notification::NotificationType::Update => NotificationType::Edited,
            MegalodonEntities::notification::NotificationType::Quote => NotificationType::Quote,
            MegalodonEntities::notification::NotificationType::QuotedUpdate => {
                NotificationType::Edited
            }
        }
    }
}

impl From<NotificationType> for MegalodonEntities::notification::NotificationType {
    fn from(val: NotificationType) -> Self {
        match val {
            NotificationType::Follow => MegalodonEntities::notification::NotificationType::Follow,
            NotificationType::Mention => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Reply => MegalodonEntities::notification::NotificationType::Mention,
            NotificationType::Renote => MegalodonEntities::notification::NotificationType::Reblog,
            NotificationType::Quote => MegalodonEntities::notification::NotificationType::Quote,
            NotificationType::Reaction => {
                MegalodonEntities::notification::NotificationType::Reaction
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
            NotificationType::Note => MegalodonEntities::notification::NotificationType::Status,
            NotificationType::RoleAssigned => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::ChatRoomInvitationReceived => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::AchievementEarned => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::ExportCompleted => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::ImportCompleted => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::Login => MegalodonEntities::notification::NotificationType::Unknown,
            NotificationType::CreateToken => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::App => MegalodonEntities::notification::NotificationType::App,
            NotificationType::Test => MegalodonEntities::notification::NotificationType::Unknown,
            NotificationType::Edited => MegalodonEntities::notification::NotificationType::Update,
            NotificationType::ScheduledNoteFailed => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::ScheduledNotePosted => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::SharedAccessGranted => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::SharedAccessRevoked => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::SharedAccessLogin => {
                MegalodonEntities::notification::NotificationType::Unknown
            }
            NotificationType::Unknown => MegalodonEntities::notification::NotificationType::Unknown,
        }
    }
}

impl From<Notification> for MegalodonEntities::Notification {
    fn from(val: Notification) -> Self {
        let emojis = if let Some(note) = &val.note {
            note.clone().reaction_emojis
        } else {
            HashMap::<String, String>::new()
        };
        let reactions = if let Some(reaction) = val.reaction {
            map_reaction(emojis, HashMap::<String, u32>::from([(reaction, 1)]), None)
        } else {
            [].to_vec()
        };
        let reaction = if reactions.len() > 0 {
            Some(reactions[0].clone())
        } else {
            None
        };
        MegalodonEntities::Notification {
            account: val.user.map(|u| u.into()),
            created_at: val.created_at,
            id: val.id,
            status: val.note.map(|n| n.into()),
            reaction,
            target: None,
            r#type: val.r#type.into(),
        }
    }
}
