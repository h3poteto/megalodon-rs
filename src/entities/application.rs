use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
    pub vapid_key: Option<String>,
}
