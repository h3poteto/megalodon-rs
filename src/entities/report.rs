use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Report {
    pub id: String,
    pub action_taken: String,
    pub comment: String,
    pub account_id: String,
    pub status_ids: Vec<String>,
}
