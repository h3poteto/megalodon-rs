use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    id: String,
    is_following: bool,
    has_pending_follow_request_from_you: bool,
    // has_pending_follow_request_to_you: bool,
    is_followed: bool,
    is_blocking: bool,
    is_blocked: bool,
    is_muted: bool,
    is_renote_muted: bool,
}

impl From<Relation> for MegalodonEntities::Relationship {
    fn from(val: Relation) -> Self {
        MegalodonEntities::Relationship {
            id: val.id,
            following: val.is_following,
            followed_by: val.is_followed,
            blocking: val.is_blocking,
            blocked_by: val.is_blocked,
            muting: val.is_muted,
            muting_notifications: false,
            requested: val.has_pending_follow_request_from_you,
            domain_blocking: false,
            showing_reblogs: val.is_renote_muted,
            endorsed: false,
            notifying: false,
            note: None,
        }
    }
}
