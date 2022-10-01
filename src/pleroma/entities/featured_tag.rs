use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct FeaturedTag {
    id: String,
    name: String,
    statuses_count: u32,
    last_status_at: DateTime<Utc>,
}

impl Into<MegalodonEntities::FeaturedTag> for FeaturedTag {
    fn into(self) -> MegalodonEntities::FeaturedTag {
        MegalodonEntities::FeaturedTag {
            id: self.id,
            name: self.name,
            statuses_count: self.statuses_count,
            last_status_at: self.last_status_at,
        }
    }
}
