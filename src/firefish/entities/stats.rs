use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    notes_count: u64,
    // original_notes_count: u32,
    users_count: u32,
    // original_users_count: u32,
    instances: u32,
}

impl From<Stats> for MegalodonEntities::Stats {
    fn from(val: Stats) -> Self {
        MegalodonEntities::Stats {
            user_count: val.users_count,
            status_count: val.notes_count,
            domain_count: val.instances,
        }
    }
}
