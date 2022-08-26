use chrono::{DateTime, Utc};

pub struct IdentityProof {
    provider: String,
    provider_username: String,
    updated_at: DateTime<Utc>,
    proof_url: String,
    profile_url: String,
}
