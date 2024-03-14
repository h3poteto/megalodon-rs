use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    id: String,
    title: String,
}

impl From<List> for MegalodonEntities::List {
    fn from(val: List) -> Self {
        MegalodonEntities::List {
            id: val.id,
            title: val.title,
            replies_policy: None,
        }
    }
}
