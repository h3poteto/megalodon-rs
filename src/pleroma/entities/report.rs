use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Report {
    pub id: String,
    pub action_taken: bool,
}

impl Into<MegalodonEntities::Report> for Report {
    fn into(self) -> MegalodonEntities::Report {
        MegalodonEntities::Report {
            id: self.id,
            action_taken: self.action_taken,
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
