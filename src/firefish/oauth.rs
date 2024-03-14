use crate::oauth;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppDataFromServer {
    pub id: String,
    pub name: String,
    // #[serde(rename = "callbackUrl")]
    pub callback_url: Option<String>,
    pub permission: Vec<String>,
    pub secret: Option<String>,
    #[serde(rename = "isAuthorized")]
    pub is_authorized: Option<bool>,
}

impl From<AppDataFromServer> for oauth::AppData {
    fn from(val: AppDataFromServer) -> Self {
        oauth::AppData::new(
            val.id,
            val.name,
            None,
            val.callback_url,
            "".to_string(),
            val.secret.unwrap(),
        )
    }
}

/// Obteined token data from server.
#[derive(Debug, Deserialize, Clone)]
pub struct TokenDataFromServer {
    #[serde(rename = "accessToken")]
    access_token: String,
}

impl From<TokenDataFromServer> for oauth::TokenData {
    fn from(val: TokenDataFromServer) -> Self {
        oauth::TokenData::new(
            val.access_token,
            "Firefish".to_string(),
            None,
            None,
            None,
            None,
        )
    }
}
