use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    name: String,
    value: String,
    verified: Option<bool>,
}

impl From<Field> for MegalodonEntities::Field {
    fn from(val: Field) -> Self {
        MegalodonEntities::Field {
            name: val.name,
            value: val.value,
            verified_at: None,
            verified: val.verified,
        }
    }
}
