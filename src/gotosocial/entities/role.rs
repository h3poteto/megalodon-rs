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

impl From<Role> for MegalodonEntities::Role {
    fn from(val: Role) -> MegalodonEntities::Role {
        MegalodonEntities::Role { name: val.name }
    }
}
