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

impl Into<MegalodonEntities::Account> for FollowRequest {
    fn into(self) -> MegalodonEntities::Account {
        self.follower.into()
    }
}

impl Into<megalodon::FollowRequestOutput> for FollowRequest {
    fn into(self) -> megalodon::FollowRequestOutput {
        megalodon::FollowRequestOutput::Account(self.into())
    }
}
