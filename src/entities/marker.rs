use chrono::{DateTime, Utc};

pub struct Marker {
    pub home: InnerMarker,
    pub notifications: InnerMarker,
}

pub struct InnerMarker {
    pub last_read_id: String,
    pub version: u32,
    pub updated_at: DateTime<Utc>,
}
