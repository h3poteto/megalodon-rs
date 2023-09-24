use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Token {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u64,
}
