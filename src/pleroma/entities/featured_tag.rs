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

impl From<FeaturedTag> for MegalodonEntities::FeaturedTag {
    fn from(val: FeaturedTag) -> Self {
        MegalodonEntities::FeaturedTag {
            id: val.id,
            name: val.name,
            statuses_count: val.statuses_count,
            last_status_at: val.last_status_at,
        }
    }
}
