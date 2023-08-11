use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Role {
    name: String,
}

impl From<MegalodonEntities::Role> for Role {
    fn from(value: MegalodonEntities::Role) -> Self {
        Self { name: value.name }
    }
}

impl Into<MegalodonEntities::Role> for Role {
    fn into(self) -> MegalodonEntities::Role {
        MegalodonEntities::Role { name: self.name }
    }
}
