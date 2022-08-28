use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    pub user_count: u32,
    pub status_count: u64,
    pub domain_count: u32,
}
