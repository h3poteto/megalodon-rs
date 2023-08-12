use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Relationship {
    pub id: String,
    pub following: bool,
    pub followed_by: bool,
    pub blocking: bool,
    pub blocked_by: bool,
    pub muting: bool,
    pub muting_notifications: bool,
    pub requested: bool,
    pub domain_blocking: bool,
    pub showing_reblogs: bool,
    pub endorsed: bool,
    pub notifying: bool,
    // Friendica returns null as note.
    pub note: Option<String>,
}
