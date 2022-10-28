use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Stats {
    pub user_count: u32,
    pub status_count: u64,
    pub domain_count: u32,
}
