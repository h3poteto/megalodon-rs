use super::User;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    id: String,
    follower: User,
    followee: User,
}
