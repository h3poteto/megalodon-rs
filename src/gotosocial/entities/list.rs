use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    id: String,
    title: String,
    replies_policy: Option<RepliesPolicy>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepliesPolicy {
    Followed,
    List,
    None,
}

impl From<RepliesPolicy> for MegalodonEntities::list::RepliesPolicy {
    fn from(val: RepliesPolicy) -> MegalodonEntities::list::RepliesPolicy {
        match val {
            RepliesPolicy::Followed => MegalodonEntities::list::RepliesPolicy::Followed,
            RepliesPolicy::List => MegalodonEntities::list::RepliesPolicy::List,
            RepliesPolicy::None => MegalodonEntities::list::RepliesPolicy::None,
        }
    }
}

impl From<List> for MegalodonEntities::List {
    fn from(val: List) -> MegalodonEntities::List {
        MegalodonEntities::List {
            id: val.id,
            title: val.title,
            replies_policy: val.replies_policy.map(|r| r.into()),
        }
    }
}
