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

impl Into<MegalodonEntities::Stats> for Stats {
    fn into(self) -> MegalodonEntities::Stats {
        MegalodonEntities::Stats {
            user_count: self.users_count,
            status_count: self.notes_count,
            domain_count: self.instances,
        }
    }
}
