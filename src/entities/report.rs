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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spam,
    Violation,
    Other,
}
