use chrono::{DateTime, Utc};

pub struct Marker {
    home: InnerMarker,
    notifications: InnerMarker,
}

struct InnerMarker {
    last_read_id: String,
    version: u32,
    updated_at: DateTime<Utc>,
}
