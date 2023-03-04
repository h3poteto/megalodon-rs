use core::fmt;

use serde::{Deserialize, Serialize};

use super::Account;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
    pub category: Category,
    pub comment: String,
    pub forwarded: bool,
    pub status_ids: Option<Vec<String>>,
    pub rule_ids: Option<Vec<String>>,
    pub target_account: Account,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spam,
    Violation,
    Other,
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Category::Spam => write!(f, "spam"),
            Category::Violation => write!(f, "violation"),
            Category::Other => write!(f, "other"),
        }
    }
}
