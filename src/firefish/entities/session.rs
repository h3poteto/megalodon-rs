use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Session {
    pub url: String,
    pub token: String,
}
