use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct List {
    id: String,
    title: String,
}

impl Into<MegalodonEntities::List> for List {
    fn into(self) -> MegalodonEntities::List {
        MegalodonEntities::List {
            id: self.id,
            title: self.title,
        }
    }
}
