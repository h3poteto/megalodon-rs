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

impl Into<MegalodonEntities::list::RepliesPolicy> for RepliesPolicy {
    fn into(self) -> MegalodonEntities::list::RepliesPolicy {
        match self {
            Self::Followed => MegalodonEntities::list::RepliesPolicy::Followed,
            Self::List => MegalodonEntities::list::RepliesPolicy::List,
            Self::None => MegalodonEntities::list::RepliesPolicy::None,
        }
    }
}

impl Into<MegalodonEntities::List> for List {
    fn into(self) -> MegalodonEntities::List {
        MegalodonEntities::List {
            id: self.id,
            title: self.title,
            replies_policy: self.replies_policy.map(|r| r.into()),
        }
    }
}
