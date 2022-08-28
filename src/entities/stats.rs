use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    user_count: u32,
    status_count: u64,
    domain_count: u32,
}
