use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PollOption {
    pub title: String,
    pub votes_count: Option<u32>,
}
