use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Marker {
    home: InnerMarker,
    notifications: InnerMarker,
}

#[derive(Debug, Deserialize, Clone)]
struct InnerMarker {
    last_read_id: String,
    version: u32,
    updated_at: DateTime<Utc>,
}

impl Into<MegalodonEntities::Marker> for Marker {
    fn into(self) -> MegalodonEntities::Marker {
        MegalodonEntities::Marker {
            home: Some(self.home.into()),
            notifications: Some(self.notifications.into()),
        }
    }
}

impl Into<MegalodonEntities::marker::InnerMarker> for InnerMarker {
    fn into(self) -> MegalodonEntities::marker::InnerMarker {
        MegalodonEntities::marker::InnerMarker {
            last_read_id: self.last_read_id,
            version: self.version,
            updated_at: self.updated_at,
            unread_count: None,
        }
    }
}
