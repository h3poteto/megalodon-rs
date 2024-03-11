use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Marker {
    home: Option<InnerMarker>,
    notifications: Option<InnerMarker>,
}

#[derive(Debug, Deserialize, Clone)]
struct InnerMarker {
    last_read_id: String,
    version: u32,
    updated_at: DateTime<Utc>,
}

impl From<Marker> for MegalodonEntities::Marker {
    fn from(val: Marker) -> Self {
        MegalodonEntities::Marker {
            home: val.home.map(|i| i.into()),
            notifications: val.notifications.map(|i| i.into()),
        }
    }
}

impl From<InnerMarker> for MegalodonEntities::marker::InnerMarker {
    fn from(val: InnerMarker) -> Self {
        MegalodonEntities::marker::InnerMarker {
            last_read_id: val.last_read_id,
            version: val.version,
            updated_at: val.updated_at,
            unread_count: None,
        }
    }
}
