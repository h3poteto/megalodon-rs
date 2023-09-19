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

impl Into<MegalodonEntities::Relationship> for Relation {
    fn into(self) -> MegalodonEntities::Relationship {
        MegalodonEntities::Relationship {
            id: self.id,
            following: self.is_following,
            followed_by: self.is_followed,
            blocking: self.is_blocking,
            blocked_by: self.is_blocked,
            muting: self.is_muted,
            muting_notifications: false,
            requested: self.has_pending_follow_request_from_you,
            domain_blocking: false,
            showing_reblogs: self.is_renote_muted,
            endorsed: false,
            notifying: false,
            note: None,
        }
    }
}
