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
    subscribing: bool,
}

impl Into<MegalodonEntities::Relationship> for Relationship {
    fn into(self) -> MegalodonEntities::Relationship {
        MegalodonEntities::Relationship {
            id: self.id,
            following: self.following,
            followed_by: self.followed_by,
            delivery_following: None,
            blocking: self.blocking,
            blocked_by: self.blocked_by,
            muting: self.muting,
            muting_notifications: self.muting_notifications,
            requested: self.requested,
            domain_blocking: self.domain_blocking,
            showing_reblogs: self.showing_reblogs,
            endorsed: self.endorsed,
            notifying: self.subscribing,
        }
    }
}
