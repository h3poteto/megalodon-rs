use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    name: String,
    value: String,
    verified_at: Option<DateTime<Utc>>,
}

impl From<MegalodonEntities::Field> for Field {
    fn from(item: MegalodonEntities::Field) -> Self {
        Self {
            name: item.name,
            value: item.value,
            verified_at: item.verified_at,
        }
    }
}

impl From<Field> for MegalodonEntities::Field {
    fn from(item: Field) -> MegalodonEntities::Field {
        MegalodonEntities::Field {
            name: item.name,
            value: item.value,
            verified_at: item.verified_at,
            verified: None,
        }
    }
}
