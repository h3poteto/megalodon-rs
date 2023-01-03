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

impl Into<MegalodonEntities::Marker> for Marker {
    fn into(self) -> MegalodonEntities::Marker {
        MegalodonEntities::Marker {
            home: None,
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
            unread_count: Some(self.pleroma.unread_count),
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
