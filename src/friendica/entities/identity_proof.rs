use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct IdentityProof {
    provider: String,
    provider_username: String,
    updated_at: DateTime<Utc>,
    proof_url: String,
    profile_url: String,
}

impl From<IdentityProof> for MegalodonEntities::IdentityProof {
    fn from(val: IdentityProof) -> Self {
        MegalodonEntities::IdentityProof {
            provider: val.provider,
            provider_username: val.provider_username,
            updated_at: val.updated_at,
            proof_url: val.proof_url,
            profile_url: val.profile_url,
        }
    }
}
