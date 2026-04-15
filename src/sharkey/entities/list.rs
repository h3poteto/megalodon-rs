use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List {
    id: String,
    // created_at: DateTime<Utc>,
    name: String,
    pub user_ids: Option<Vec<String>>,
}

impl From<List> for MegalodonEntities::List {
    fn from(val: List) -> Self {
        MegalodonEntities::List {
            id: val.id,
            title: val.name,
            replies_policy: None,
        }
    }
}
