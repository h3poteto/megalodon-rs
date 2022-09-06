use std::collections::HashMap;

use super::api_client::APIClient;
use super::entities;
use crate::{
    default, entities as MegalodonEntities, error::Error, megalodon, oauth, response::Response,
};
use async_trait::async_trait;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, ResponseType, Scope, TokenUrl,
};

pub struct Mastodon {
    client: APIClient,
    base_url: String,
}

impl Mastodon {
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Mastodon {
        let client = APIClient::new(base_url.clone(), access_token, user_agent);
        Mastodon { client, base_url }
    }

    async fn generate_auth_url(
        &self,
        client_id: String,
        client_secret: String,
        scope: Vec<&str>,
        redirect_uri: String,
    ) -> Result<String, Error> {
        let client = BasicClient::new(
            ClientId::new(client_id),
            Some(ClientSecret::new(client_secret)),
            AuthUrl::new(format!("{}{}", self.base_url, "/oauth/authorize").to_string())?,
            Some(TokenUrl::new(
                format!("{}{}", self.base_url, "/oaut/token").to_string(),
            )?),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_uri)?);

        let scopes: Vec<Scope> = scope.iter().map(|s| Scope::new(s.to_string())).collect();

        let (auth_url, _) = client
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .set_response_type(&ResponseType::new("code".to_string()))
            .url();
        Ok(auth_url.to_string())
    }
}

#[async_trait]
impl megalodon::Megalodon for Mastodon {
    async fn register_app(
        &self,
        client_name: String,
        options: &megalodon::AppInputOptions,
    ) -> Result<oauth::AppData, Error> {
        let mut scope = default::DEFAULT_SCOPES.to_vec();
        if let Some(scopes) = &options.scopes {
            scope = scopes.iter().map(|s| s.as_ref()).collect();
        }

        let mut app = self.create_app(client_name, options).await?;
        let url = self
            .generate_auth_url(
                app.client_id.clone(),
                app.client_secret.clone(),
                scope,
                app.redirect_uri.clone(),
            )
            .await?;
        app.url = Some(url);
        Ok(app)
    }

    async fn create_app(
        &self,
        client_name: String,
        options: &megalodon::AppInputOptions,
    ) -> Result<oauth::AppData, Error> {
        let mut scope = default::DEFAULT_SCOPES.to_vec();
        if let Some(scopes) = &options.scopes {
            scope = scopes.iter().map(|s| s.as_ref()).collect();
        }
        let mut redirect_uris = default::NO_REDIRECT;
        if let Some(uris) = &options.redirect_uris {
            redirect_uris = uris.as_ref();
        }

        let mut params = HashMap::<&str, String>::new();
        params.insert("client_name", client_name);
        params.insert("redirect_uris", redirect_uris.to_string());
        params.insert("scopes", scope.join(" "));
        if let Some(website) = &options.website {
            params.insert("website", website.clone());
        }

        let res = self
            .client
            .post::<oauth::AppDataFromServer>("/api/v1/apps", &params, None)
            .await?;
        Ok(oauth::AppData::from(res.json.into()))
    }

    async fn fetch_access_token(
        &self,
        client_id: String,
        client_secret: String,
        code: String,
        redirect_uri: String,
    ) -> Result<oauth::TokenData, Error> {
        let mut params = HashMap::<&str, String>::new();
        params.insert("client_id", client_id);
        params.insert("client_secret", client_secret);
        params.insert("code", code);
        params.insert("redirect_uri", redirect_uri);
        params.insert("grant_type", "authorization_code".to_string());

        let res = self
            .client
            .post::<oauth::TokenDataFromServer>("/oauth/token", &params, None)
            .await?;
        Ok(oauth::TokenData::from(res.json.into()))
    }

    async fn refresh_access_token(
        &self,
        client_id: String,
        client_secret: String,
        refresh_token: String,
    ) -> Result<oauth::TokenData, Error> {
        let mut params = HashMap::<&str, String>::new();
        params.insert("client_id", client_id);
        params.insert("client_secret", client_secret);
        params.insert("refresh_token", refresh_token);
        params.insert("grant_type", "authorization_code".to_string());

        let res = self
            .client
            .post::<oauth::TokenDataFromServer>("/oauth/token", &params, None)
            .await?;
        Ok(oauth::TokenData::from(res.json.into()))
    }

    async fn revoke_access_token(
        &self,
        client_id: String,
        client_secret: String,
        access_token: String,
    ) -> Result<Response<()>, Error> {
        let mut params = HashMap::<&str, String>::new();
        params.insert("client_id", client_id);
        params.insert("client_secret", client_secret);
        params.insert("token", access_token);

        let res = self
            .client
            .post::<()>("/oauth/revoke", &params, None)
            .await?;
        Ok(res)
    }

    async fn verify_app_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Application>, Error> {
        let res = self
            .client
            .get::<entities::Application>("/api/v1/apps/verify_credentials", None)
            .await?;

        Ok(Response::<MegalodonEntities::Application>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn verify_account_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let res = self
            .client
            .get::<entities::Account>("/api/v1/accounts/verify_credentials", None)
            .await?;
        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_instance(&self) -> Result<Response<MegalodonEntities::Instance>, Error> {
        let res = self
            .client
            .get::<entities::Instance>("/api/v1/instance", None)
            .await?;

        Ok(Response::<MegalodonEntities::Instance>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }
}
