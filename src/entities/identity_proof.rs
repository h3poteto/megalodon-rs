use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IdentityProof {
    pub provider: String,
    pub provider_username: String,
    pub updated_at: DateTime<Utc>,
    pub proof_url: String,
    pub profile_url: String,
}
