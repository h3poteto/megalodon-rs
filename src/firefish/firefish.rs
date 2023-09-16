use std::collections::HashMap;

use serde_json::Value;

use super::{
    api_client::{APIClient, DEFAULT_SCOPES},
    entities, oauth,
};
use crate::{
    default, entities as MegalodonEntities,
    error::{self, Error},
    megalodon, oauth as MegalodonOAuth,
    response::Response,
};

/// Firefish API Client which satisfies megalodon trait.
#[derive(Debug, Clone)]
pub struct Firefish {
    client: APIClient,
    base_url: String,
}

impl Firefish {
    /// Create a new [`Firefish`].
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Firefish {
        let client = APIClient::new(base_url.clone(), access_token, user_agent);
        Firefish { client, base_url }
    }

    async fn generate_auth_url_and_token(
        &self,
        client_secret: String,
    ) -> Result<entities::Session, Error> {
        let params = HashMap::<&str, Value>::from([("appSecret", Value::String(client_secret))]);
        let res = self
            .client
            .post::<entities::Session>("/api/auth/session/generate", &params, None)
            .await?;
        Ok(res.json)
    }

    // こっからはtraitに移すべき
    pub async fn register_app(
        &self,
        client_name: String,
        options: &megalodon::AppInputOptions,
    ) -> Result<MegalodonOAuth::AppData, Error> {
        let mut app = self.create_app(client_name, options).await?;
        let session = self
            .generate_auth_url_and_token(app.client_secret.clone())
            .await?;
        app.url = Some(session.url);
        app.session_token = Some(session.token);
        Ok(app)
    }

    async fn create_app(
        &self,
        client_name: String,
        options: &megalodon::AppInputOptions,
    ) -> Result<MegalodonOAuth::AppData, Error> {
        let mut scope = DEFAULT_SCOPES.to_vec();
        if let Some(scopes) = &options.scopes {
            scope = scopes.iter().map(|s| s.as_ref()).collect();
        };
        let mut redirect_uris = self.base_url.clone();
        if let Some(uris) = &options.redirect_uris {
            redirect_uris = uris.to_string();
        }

        let mut params = HashMap::<&str, Value>::new();
        params.insert("name", Value::String(client_name.to_string()));
        params.insert("description", Value::String("".to_string()));
        params.insert("callbackUrl", Value::String(redirect_uris.to_string()));
        if let Some(json_scope) = serde_json::to_value(scope).ok() {
            params.insert("permission", json_scope);
        };

        let res = self
            .client
            .post::<oauth::AppDataFromServer>("/api/app/create", &params, None)
            .await?;
        if let Some(_) = res.json.secret {
            Ok(MegalodonOAuth::AppData::from(res.json.into()))
        } else {
            Err(Error::new_own(
                "secret does not exist".to_string(),
                error::Kind::UnsatisfiedError,
                None,
                None,
            ))
        }
    }

    pub async fn fetch_access_token(
        &self,
        _client_id: String,
        client_secret: String,
        session_token: String,
        _redirect_uri: String,
    ) -> Result<MegalodonOAuth::TokenData, Error> {
        let mut params = HashMap::<&str, Value>::new();
        params.insert("appSecret", serde_json::Value::String(client_secret));
        params.insert("token", serde_json::Value::String(session_token));

        let res = self
            .client
            .post::<oauth::TokenDataFromServer>("/api/auth/session/userKey", &params, None)
            .await?;
        Ok(MegalodonOAuth::TokenData::from(res.json.into()))
    }

    pub async fn verify_account_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<entities::UserDetail>("/api/i", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }
}
