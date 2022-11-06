//! OAuth related modules
use serde::{Deserialize, Serialize};

/// Registered application data from server.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppData {
    /// Application ID.
    pub id: String,
    /// Application name.
    pub name: String,
    /// Website URL of the application.
    pub website: Option<String>,
    /// Redirect URI for the application.
    pub redirect_uri: String,
    /// Client ID.
    pub client_id: String,
    /// Client secret.
    pub client_secret: String,
    /// Authorize URL for the application.
    pub url: Option<String>,
}

impl AppData {
    /// Create a new [`AppData`].
    pub fn new(
        id: String,
        name: String,
        website: Option<String>,
        redirect_uri: String,
        client_id: String,
        client_secret: String,
    ) -> Self {
        Self {
            id,
            name,
            website,
            redirect_uri,
            client_id,
            client_secret,
            url: None,
        }
    }
}

/// Token data in server.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TokenData {
    /// Access token for the authorized user.
    pub access_token: String,
    /// Token type of the access token.
    pub token_type: String,
    /// Scope of the access token.
    pub scope: String,
    /// Created date of the access token.
    pub created_at: u64,
    /// Expires date of the access token.
    pub expires_in: Option<u64>,
    /// Refresh token of the access token.
    pub refresh_token: Option<String>,
}

impl TokenData {
    /// Create a new [`TokenData`].
    pub fn new(
        access_token: String,
        token_type: String,
        scope: String,
        created_at: u64,
        expires_in: Option<u64>,
        refresh_token: Option<String>,
    ) -> Self {
        Self {
            access_token,
            token_type,
            scope,
            created_at,
            expires_in,
            refresh_token,
        }
    }
}
