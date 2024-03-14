use super::Account;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Spam,
    Violation,
    Other,
}

impl From<Category> for MegalodonEntities::report::Category {
    fn from(val: Category) -> Self {
        match val {
            Category::Spam => MegalodonEntities::report::Category::Spam,
            Category::Violation => MegalodonEntities::report::Category::Violation,
            Category::Other => MegalodonEntities::report::Category::Other,
        }
    }
}

impl From<Report> for MegalodonEntities::Report {
    fn from(val: Report) -> Self {
        MegalodonEntities::Report {
            id: val.id,
            action_taken: val.action_taken,
            action_taken_at: None,
            category: Some(val.category.into()),
            comment: Some(val.comment),
            forwarded: Some(val.forwarded),
            status_ids: val.status_ids,
            rule_ids: val.rule_ids,
            target_account: Some(val.target_account.into()),
        }
    }
}
