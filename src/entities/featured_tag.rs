use chrono::{DateTime, Utc};

pub struct FeaturedTag {
    id: String,
    name: String,
    statuses_count: u32,
    last_status_at: DateTime<Utc>,
}
