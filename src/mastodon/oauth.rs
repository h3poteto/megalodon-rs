use crate::oauth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppDataFromServer {
    id: String,
    name: String,
    website: Option<String>,
    redirect_uri: String,
    client_id: String,
    client_secret: String,
}

/// Obteined token data from server.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

impl oauth::AppData {
    /// Create a new [`oauth::AppData`].
    pub fn from(raw: AppDataFromServer) -> Self {
        Self::new(
            raw.id,
            raw.name,
            raw.website,
            raw.redirect_uri,
            raw.client_id,
            raw.client_secret,
        )
    }
}

impl oauth::TokenData {
    /// Create a new [`oauth::TokenData`].
    pub fn from(raw: TokenDataFromServer) -> Self {
        Self::new(
            raw.access_token,
            raw.token_type,
            raw.scope,
            raw.created_at,
            raw.expires_in,
            raw.refresh_token,
        )
    }
}
