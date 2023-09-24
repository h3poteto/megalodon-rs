use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
    pub vapid_key: Option<String>,
}
