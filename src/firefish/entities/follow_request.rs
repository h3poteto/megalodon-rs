use super::User;
use serde::Deserialize;

use crate::{entities as MegalodonEntities, megalodon};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FollowRequest {
    // id: String,
    follower: User,
    // followee: User,
}

impl From<FollowRequest> for MegalodonEntities::Account {
    fn from(val: FollowRequest) -> Self {
        val.follower.into()
    }
}

impl From<FollowRequest> for megalodon::FollowRequestOutput {
    fn from(val: FollowRequest) -> Self {
        megalodon::FollowRequestOutput::Account(val.into())
    }
}
