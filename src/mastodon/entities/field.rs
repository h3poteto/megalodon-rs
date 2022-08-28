use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    pub name: String,
    pub value: String,
    pub verified_at: DateTime<Utc>,
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

impl Into<MegalodonEntities::Field> for Field {
    fn into(self) -> MegalodonEntities::Field {
        MegalodonEntities::Field {
            name: self.name,
            value: self.value,
            verified_at: self.verified_at,
        }
    }
}
