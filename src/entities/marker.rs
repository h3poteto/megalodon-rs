use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Marker {
    pub home: Option<InnerMarker>,
    pub notifications: Option<InnerMarker>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct InnerMarker {
    pub last_read_id: String,
    pub version: u32,
    pub updated_at: DateTime<Utc>,
    pub unread_count: Option<u32>,
}
