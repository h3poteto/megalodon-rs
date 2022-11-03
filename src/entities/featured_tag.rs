use serde::{Deserialize, Serialize};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FeaturedTag {
    pub id: String,
    pub name: String,
    pub statuses_count: u32,
    pub last_status_at: DateTime<Utc>,
}
