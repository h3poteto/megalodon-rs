use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    user_count: u32,
    status_count: u64,
    domain_count: u32,
}

impl From<Stats> for MegalodonEntities::Stats {
    fn from(val: Stats) -> Self {
        MegalodonEntities::Stats {
            user_count: val.user_count,
            status_count: val.status_count,
            domain_count: val.domain_count,
        }
    }
}
