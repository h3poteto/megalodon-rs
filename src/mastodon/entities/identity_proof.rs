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

impl Into<MegalodonEntities::IdentityProof> for IdentityProof {
    fn into(self) -> MegalodonEntities::IdentityProof {
        MegalodonEntities::IdentityProof {
            provider: self.provider,
            provider_username: self.provider_username,
            updated_at: self.updated_at,
            proof_url: self.proof_url,
            profile_url: self.profile_url,
        }
    }
}
