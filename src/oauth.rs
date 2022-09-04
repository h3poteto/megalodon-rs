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

#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    access_token: String,
    token_type: String,
    scope: String,
    created_at: u64,
    expires_in: Option<u64>,
    refresh_token: Option<String>,
}

pub struct AppData {
    pub id: String,
    pub name: String,
    pub website: Option<String>,
    pub redirect_uri: String,
    pub client_id: String,
    pub client_secret: String,
    pub url: Option<String>,
    pub session_token: Option<String>,
}

impl AppData {
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
            session_token: None,
        }
    }

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

pub struct TokenData {
    pub access_token: String,
    pub token_type: String,
    pub scope: String,
    pub created_at: u64,
    pub expires_in: Option<u64>,
    pub refresh_token: Option<String>,
}

impl TokenData {
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
