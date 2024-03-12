use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Marker {
    notifications: InnerMarker,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct InnerMarker {
    last_read_id: String,
    version: u32,
    #[serde(with = "date_format_without_tz")]
    updated_at: DateTime<Utc>,
    pleroma: PleromaMarker,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct PleromaMarker {
    unread_count: u32,
}

impl From<Marker> for MegalodonEntities::Marker {
    fn from(val: Marker) -> Self {
        MegalodonEntities::Marker {
            home: None,
            notifications: Some(val.notifications.into()),
        }
    }
}

impl From<InnerMarker> for MegalodonEntities::marker::InnerMarker {
    fn from(val: InnerMarker) -> Self {
        MegalodonEntities::marker::InnerMarker {
            last_read_id: val.last_read_id,
            version: val.version,
            updated_at: val.updated_at,
            unread_count: Some(val.pleroma.unread_count),
        }
    }
}

mod date_format_without_tz {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%dT%H:%M:%S";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}
