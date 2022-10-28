use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub url: String,
    pub acct: String,
}
