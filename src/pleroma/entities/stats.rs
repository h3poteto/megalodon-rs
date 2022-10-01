use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Stats {
    user_count: u32,
    status_count: u64,
    domain_count: u32,
}

impl Into<MegalodonEntities::Stats> for Stats {
    fn into(self) -> MegalodonEntities::Stats {
        MegalodonEntities::Stats {
            user_count: self.user_count,
            status_count: self.status_count,
            domain_count: self.domain_count,
        }
    }
}
