use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Relation {
    id: String,
    is_following: bool,
    has_pending_follow_request_from_you: bool,
    has_pending_follow_request_to_you: bool,
    is_followed: bool,
    is_blocking: bool,
    is_blocked: bool,
    is_muted: bool,
    is_renote_muted: bool,
}
