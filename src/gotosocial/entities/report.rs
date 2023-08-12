use super::Account;
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
    pub action_taken_at: Option<DateTime<Utc>>,
    pub action_taken_comment: Option<String>,
    pub category: Category,
    pub comment: String,
    pub forwarded: bool,
    pub status_ids: Option<Vec<String>>,
    pub rule_ids: Option<Vec<String>>,
    pub target_account: Account,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spam,
    Violation,
    Other,
}

impl Into<MegalodonEntities::report::Category> for Category {
    fn into(self) -> MegalodonEntities::report::Category {
        match self {
            Category::Spam => MegalodonEntities::report::Category::Spam,
            Category::Violation => MegalodonEntities::report::Category::Violation,
            Category::Other => MegalodonEntities::report::Category::Other,
        }
    }
}

impl Into<MegalodonEntities::Report> for Report {
    fn into(self) -> MegalodonEntities::Report {
        MegalodonEntities::Report {
            id: self.id,
            action_taken: self.action_taken,
            category: self.category.into(),
            comment: self.comment,
            forwarded: self.forwarded,
            status_ids: self.status_ids,
            rule_ids: self.rule_ids,
            target_account: self.target_account.into(),
        }
    }
}
