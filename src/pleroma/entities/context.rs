use super::Status;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Context {
    ancestors: Vec<Status>,
    descendants: Vec<Status>,
}

impl Into<MegalodonEntities::Context> for Context {
    fn into(self) -> MegalodonEntities::Context {
        MegalodonEntities::Context {
            ancestors: self.ancestors.into_iter().map(|i| i.into()).collect(),
            descendants: self.descendants.into_iter().map(|i| i.into()).collect(),
        }
    }
}
