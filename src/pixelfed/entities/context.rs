use super::Status;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Context {
    ancestors: Vec<Status>,
    descendants: Vec<Status>,
}

impl From<Context> for MegalodonEntities::Context {
    fn from(val: Context) -> Self {
        MegalodonEntities::Context {
            ancestors: val.ancestors.into_iter().map(|i| i.into()).collect(),
            descendants: val.descendants.into_iter().map(|i| i.into()).collect(),
        }
    }
}
