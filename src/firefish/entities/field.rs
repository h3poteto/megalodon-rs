use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Field {
    name: String,
    value: String,
}

impl Into<MegalodonEntities::Field> for Field {
    fn into(self) -> MegalodonEntities::Field {
        MegalodonEntities::Field {
            name: self.name,
            value: self.value,
            verified_at: None,
        }
    }
}
