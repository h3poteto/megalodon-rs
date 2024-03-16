use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Relationship {
    id: String,
    following: bool,
    followed_by: bool,
    blocking: bool,
    blocked_by: bool,
    muting: bool,
    muting_notifications: bool,
    requested: bool,
    domain_blocking: bool,
    showing_reblogs: bool,
    endorsed: bool,
    notifying: bool,
    note: String,
}

impl From<Relationship> for MegalodonEntities::Relationship {
    fn from(val: Relationship) -> MegalodonEntities::Relationship {
        MegalodonEntities::Relationship {
            id: val.id,
            following: val.following,
            followed_by: val.followed_by,
            blocking: val.blocking,
            blocked_by: val.blocked_by,
            muting: val.muting,
            muting_notifications: val.muting_notifications,
            requested: val.requested,
            domain_blocking: val.domain_blocking,
            showing_reblogs: val.showing_reblogs,
            endorsed: val.endorsed,
            notifying: val.notifying,
            note: Some(val.note),
        }
    }
}
