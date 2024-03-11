use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
}

impl From<Report> for MegalodonEntities::Report {
    fn from(val: Report) -> Self {
        MegalodonEntities::Report {
            id: val.id,
            action_taken: val.action_taken,
            action_taken_at: None,
            category: None,
            comment: None,
            forwarded: None,
            status_ids: None,
            rule_ids: None,
            target_account: None,
        }
    }
}
