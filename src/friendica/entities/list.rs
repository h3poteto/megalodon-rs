use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    id: String,
    title: String,
    replies_policy: RepliesPolicy,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum RepliesPolicy {
    Followed,
    List,
    None,
}

impl Into<MegalodonEntities::list::RepliesPolicy> for RepliesPolicy {
    fn into(self) -> MegalodonEntities::list::RepliesPolicy {
        match self {
            RepliesPolicy::Followed => MegalodonEntities::list::RepliesPolicy::Followed,
            RepliesPolicy::List => MegalodonEntities::list::RepliesPolicy::List,
            RepliesPolicy::None => MegalodonEntities::list::RepliesPolicy::None,
        }
    }
}

impl Into<MegalodonEntities::List> for List {
    fn into(self) -> MegalodonEntities::List {
        MegalodonEntities::List {
            id: self.id,
            title: self.title,
            replies_policy: Some(self.replies_policy.into()),
        }
    }
}
