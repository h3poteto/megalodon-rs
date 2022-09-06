use crate::error::Error;
use crate::oauth::{AppData, TokenData};
use crate::response::Response;
use crate::{entities, mastodon};
use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait Megalodon {
    /// Register the application to get client_id and client_secret.
    async fn register_app(
        &self,
        client_name: String,
        options: &AppInputOptions,
    ) -> Result<AppData, Error>;

    /// Create an application.
    async fn create_app(
        &self,
        client_name: String,
        options: &AppInputOptions,
    ) -> Result<AppData, Error>;

    // ======================================
    // apps/oauth
    // ======================================
    /// Fetch OAuth access token.
    /// Get an access token based client_id, client_secret and authorization_code.
    async fn fetch_access_token(
        &self,
        client_id: String,
        client_secret: String,
        code: String,
        redirect_uri: String,
    ) -> Result<TokenData, Error>;

    /// Refresh OAuth access token.
    /// Send refresh token and get new access token.
    async fn refresh_access_token(
        &self,
        client_id: String,
        client_secret: String,
        refresh_token: String,
    ) -> Result<TokenData, Error>;

    /// Revoke an access token.
    async fn revoke_access_token(
        &self,
        client_id: String,
        client_secret: String,
        access_token: String,
    ) -> Result<Response<()>, Error>;

    async fn verify_app_credentials(&self) -> Result<Response<entities::Application>, Error>;

    async fn verify_account_credentials(&self) -> Result<Response<entities::Account>, Error>;

    async fn get_instance(&self) -> Result<Response<entities::Instance>, Error>;
}

pub struct AppInputOptions {
    pub scopes: Option<Vec<String>>,
    pub redirect_uris: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Instance {
    title: String,
    uri: String,
    urls: entities::URLs,
    version: String,
}

pub async fn detector(url: &str) -> Result<SNS, Error> {
    let res = reqwest::get(format!("{}{}", url, "/api/v1/instance")).await;

    match res {
        Ok(res) => {
            let obj = res.json::<Instance>().await;
            match obj {
                Ok(json) => {
                    if json.version.contains("Pleroma") == true {
                        Ok(SNS::Pleroma)
                    } else {
                        Ok(SNS::Mastodon)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        Err(_) => {
            let client = reqwest::Client::new();
            let res = client.post(format!("{}{}", url, "/api/meta")).send().await;
            match res {
                Ok(_) => Ok(SNS::Misskey),
                Err(err) => Err(err.into()),
            }
        }
    }
}

#[derive(Debug)]
pub enum SNS {
    Mastodon,
    Pleroma,
    Misskey,
}

pub fn generator(
    sns: SNS,
    base_url: String,
    access_token: Option<String>,
    user_agent: Option<String>,
) -> Box<dyn Megalodon> {
    match sns {
        _ => {
            let mastodon = mastodon::Mastodon::new(base_url, access_token, user_agent);
            Box::new(mastodon)
        }
    }
}
