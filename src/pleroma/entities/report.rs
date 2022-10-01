use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Report {
    id: String,
    action_taken: String,
    comment: String,
    account_id: String,
    status_ids: Vec<String>,
}

impl Into<MegalodonEntities::Report> for Report {
    fn into(self) -> MegalodonEntities::Report {
        MegalodonEntities::Report {
            id: self.id,
            action_taken: self.action_taken,
            comment: self.comment,
            account_id: self.account_id,
            status_ids: self.status_ids,
        }
    }
}
