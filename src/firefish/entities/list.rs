use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List {
    id: String,
    // created_at: DateTime<Utc>,
    name: String,
    // user_ids: Vec<String>,
}

impl Into<MegalodonEntities::List> for List {
    fn into(self) -> MegalodonEntities::List {
        MegalodonEntities::List {
            id: self.id,
            title: self.name,
            replies_policy: None,
        }
    }
}
