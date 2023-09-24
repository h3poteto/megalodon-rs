use core::fmt;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::Account;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
    pub action_taken_at: Option<DateTime<Utc>>,
    pub status_ids: Option<Vec<String>>,
    pub rule_ids: Option<Vec<String>>,
    // These parameters are optional in Pleroma.
    pub category: Option<Category>,
    pub comment: Option<String>,
    pub forwarded: Option<bool>,
    pub target_account: Option<Account>,
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
