use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct List {
    pub id: String,
    pub title: String,
}
