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

impl From<Category> for MegalodonEntities::report::Category {
    fn from(val: Category) -> MegalodonEntities::report::Category {
        match val {
            Category::Spam => MegalodonEntities::report::Category::Spam,
            Category::Violation => MegalodonEntities::report::Category::Violation,
            Category::Other => MegalodonEntities::report::Category::Other,
        }
    }
}

impl From<Report> for MegalodonEntities::Report {
    fn from(val: Report) -> MegalodonEntities::Report {
        MegalodonEntities::Report {
            id: val.id,
            action_taken: val.action_taken,
            action_taken_at: val.action_taken_at,
            status_ids: val.status_ids,
            category: Some(val.category.into()),
            comment: Some(val.comment),
            forwarded: Some(val.forwarded),
            rule_ids: val.rule_ids,
            target_account: Some(val.target_account.into()),
        }
    }
}
