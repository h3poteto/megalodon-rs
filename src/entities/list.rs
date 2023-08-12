use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct List {
    pub id: String,
    pub title: String,
    pub replies_policy: Option<RepliesPolicy>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepliesPolicy {
    Followed,
    List,
    None,
}

impl fmt::Display for RepliesPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepliesPolicy::Followed => write!(f, "followed"),
            RepliesPolicy::List => write!(f, "list"),
            RepliesPolicy::None => write!(f, "none"),
        }
    }
}
