use super::api_client::APIClient;
use super::entities;
use super::oauth;
use super::web_socket::WebSocket;
use crate::megalodon::FollowRequestOutput;
use crate::{
    default, entities as MegalodonEntities, error::Error, megalodon, oauth as MegalodonOAuth,
    response::Response,
};
use crate::{error, Streaming};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, ResponseType, Scope, TokenUrl,
};
use reqwest::header::HeaderMap;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

/// Gotosocial API client which satisfies megalodon trait.
#[derive(Debug, Clone)]
pub struct Gotosocial {
    client: APIClient,
    base_url: String,
    access_token: Option<String>,
    user_agent: Option<String>,
}

impl Gotosocial {
    /// Create a new [`Gotosocial`].
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Gotosocial {
        let client = APIClient::new(base_url.clone(), access_token.clone(), user_agent.clone());
        Gotosocial {
            client,
            base_url,
            access_token,
            user_agent,
        }
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
                format!("{}{}", self.base_url, "/oauth/token").to_string(),
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
impl megalodon::Megalodon for Gotosocial {
    async fn register_app(
        &self,
        client_name: String,
        options: &megalodon::AppInputOptions,
    ) -> Result<MegalodonOAuth::AppData, Error> {
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
    ) -> Result<MegalodonOAuth::AppData, Error> {
        let mut scope = default::DEFAULT_SCOPES.to_vec();
        if let Some(scopes) = &options.scopes {
            scope = scopes.iter().map(|s| s.as_ref()).collect();
        }
        let mut redirect_uris = default::NO_REDIRECT;
        if let Some(uris) = &options.redirect_uris {
            redirect_uris = uris.as_ref();
        }

        let mut params = HashMap::<&str, Value>::new();
        params.insert("client_name", serde_json::Value::String(client_name));
        params.insert(
            "redirect_uris",
            serde_json::Value::String(redirect_uris.to_string()),
        );
        params.insert("scopes", serde_json::Value::String(scope.join(" ")));
        if let Some(website) = &options.website {
            params.insert("website", serde_json::Value::String(website.clone()));
        }

        let res = self
            .client
            .post::<oauth::AppDataFromServer>("/api/v1/apps", &params, None)
            .await?;
        Ok(MegalodonOAuth::AppData::from(res.json.into()))
    }

    async fn fetch_access_token(
        &self,
        client_id: String,
        client_secret: String,
        code: String,
        redirect_uri: String,
    ) -> Result<MegalodonOAuth::TokenData, Error> {
        let mut params = HashMap::<&str, Value>::new();
        params.insert("client_id", serde_json::Value::String(client_id));
        params.insert("client_secret", serde_json::Value::String(client_secret));
        params.insert("code", serde_json::Value::String(code));
        params.insert("redirect_uri", serde_json::Value::String(redirect_uri));
        params.insert(
            "grant_type",
            serde_json::Value::String("authorization_code".to_string()),
        );

        let res = self
            .client
            .post::<oauth::TokenDataFromServer>("/oauth/token", &params, None)
            .await?;
        Ok(MegalodonOAuth::TokenData::from(res.json.into()))
    }

    async fn refresh_access_token(
        &self,
        client_id: String,
        client_secret: String,
        refresh_token: String,
    ) -> Result<MegalodonOAuth::TokenData, Error> {
        let mut params = HashMap::<&str, Value>::new();
        params.insert("client_id", serde_json::Value::String(client_id));
        params.insert("client_secret", serde_json::Value::String(client_secret));
        params.insert("refresh_token", serde_json::Value::String(refresh_token));
        params.insert(
            "grant_type",
            serde_json::Value::String("authorization_code".to_string()),
        );

        let res = self
            .client
            .post::<oauth::TokenDataFromServer>("/oauth/token", &params, None)
            .await?;
        Ok(MegalodonOAuth::TokenData::from(res.json.into()))
    }

    async fn revoke_access_token(
        &self,
        client_id: String,
        client_secret: String,
        access_token: String,
    ) -> Result<Response<()>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        params.insert("client_id", serde_json::Value::String(client_id));
        params.insert("client_secret", serde_json::Value::String(client_secret));
        params.insert("token", serde_json::Value::String(access_token));

        let res = self
            .client
            .post::<()>("/oauth/revoke", &params, None)
            .await?;
        Ok(res)
    }

    async fn verify_app_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Application>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn register_account(
        &self,
        username: String,
        email: String,
        password: String,
        agreement: String,
        locale: String,
        reason: Option<String>,
    ) -> Result<Response<MegalodonEntities::Token>, Error> {
        let mut params = HashMap::<&str, Value>::from([
            ("username", serde_json::Value::String(username)),
            ("email", serde_json::Value::String(email)),
            ("password", serde_json::Value::String(password)),
            ("agreement", serde_json::Value::String(agreement)),
            ("locale", serde_json::Value::String(locale)),
        ]);
        if let Some(reason) = reason {
            params.insert("reason", serde_json::Value::String(reason));
        }

        let res = self
            .client
            .post::<entities::Token>("/api/v1/accounts", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Token>::new(
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

    async fn update_credentials(
        &self,
        options: Option<&megalodon::UpdateCredentialsInputOptions>,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(discoverable) = options.discoverable {
                params.insert(
                    "discoverable",
                    serde_json::Value::String(discoverable.to_string()),
                );
            }
            if let Some(bot) = options.bot {
                params.insert("bot", serde_json::Value::String(bot.to_string()));
            }
            if let Some(display_name) = &options.display_name {
                params.insert(
                    "display_name",
                    serde_json::Value::String(display_name.clone()),
                );
            }
            if let Some(note) = &options.note {
                params.insert("note", serde_json::Value::String(note.clone()));
            }
            if let Some(avatar) = &options.avatar {
                params.insert("avatar", serde_json::Value::String(avatar.clone()));
            }
            if let Some(header) = &options.header {
                params.insert("header", serde_json::Value::String(header.clone()));
            }
            if let Some(locked) = options.locked {
                params.insert("locked", serde_json::Value::String(locked.to_string()));
            }
            if let Some(source) = &options.source {
                if let Some(json_source) = serde_json::to_value(&source).ok() {
                    params.insert("source", json_source);
                }
            }
            if let Some(fields_attributes) = &options.fields_attributes {
                let json_fields_attributes = serde_json::map::Map::from_iter(
                    fields_attributes
                        .iter()
                        .enumerate()
                        .map(|(x, y)| (x.to_string(), serde_json::to_value(y).ok().into())),
                );

                if let Ok(json_fields_attributes) = serde_json::to_value(json_fields_attributes) {
                    params.insert("fields_attributes", json_fields_attributes);
                }
            }
        }

        let res = self
            .client
            .patch::<entities::Account>("/api/v1/accounts/update_credentials", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account(&self, id: String) -> Result<Response<MegalodonEntities::Account>, Error> {
        let res = self
            .client
            .get::<entities::Account>(format!("/api/v1/accounts/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account_statuses(
        &self,
        id: String,
        options: Option<&megalodon::GetAccountStatusesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(pinned) = options.pinned {
                params.push(format!("pinned={}", pinned));
            }
            if let Some(exclude_replies) = options.exclude_replies {
                params.push(format!("exclude_replies={}", exclude_replies));
            }
            if let Some(exclude_reblogs) = options.exclude_reblogs {
                params.push(format!("exclude_reblogs={}", exclude_reblogs));
            }
            if let Some(only_media) = options.only_media {
                params.push(format!("only_media={}", only_media));
            }
        }
        let mut url = format!("/api/v1/accounts/{}/statuses", id);
        if params.len() > 0 {
            url = url + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(url.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|s| s.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn subscribe_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("notify", serde_json::Value::Bool(true))]);
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/follow", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unsubscribe_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("notify", serde_json::Value::Bool(false))]);
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/follow", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account_followers(
        &self,
        id: String,
        options: Option<&megalodon::AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
        }
        let mut url = format!("/api/v1/accounts/{}/followers", id);
        if params.len() > 0 {
            url = url + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Account>>(&url, None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account_following(
        &self,
        id: String,
        options: Option<&megalodon::AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
        }
        let mut url = format!("/api/v1/accounts/{}/following", id);
        if params.len() > 0 {
            url = url + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Account>>(&url, None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account_lists(
        &self,
        id: String,
    ) -> Result<Response<Vec<MegalodonEntities::List>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::List>>(format!("/api/v1/accounts/{}/lists", id).as_ref(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::List>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_identity_proofs(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::IdentityProof>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn follow_account(
        &self,
        id: String,
        options: Option<&megalodon::FollowAccountInputOptions>,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(reblog) = options.reblog {
                params.insert("reblog", serde_json::Value::String(reblog.to_string()));
            }
        }

        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/follow", id).as_ref(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unfollow_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/unfollow", id).as_ref(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn block_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/block", id).as_ref(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unblock_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/unblock", id).as_ref(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn mute_account(
        &self,
        _id: String,
        _notifications: bool,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unmute_account(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn pin_account(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unpin_account(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_relationships(
        &self,
        ids: Vec<String>,
    ) -> Result<Response<Vec<MegalodonEntities::Relationship>>, Error> {
        let mut params = Vec::<String>::new();
        for id in ids.iter() {
            params.push(format!("id[]={}", id));
        }
        let path = "/api/v1/accounts/relationships?".to_string() + params.join("&").as_str();
        let res = self
            .client
            .get::<Vec<entities::Relationship>>(path.as_ref(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Relationship>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn search_account(
        &self,
        acct: String,
        _options: Option<&megalodon::SearchAccountInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let params = Vec::<String>::from([format!("acct={}", acct)]);
        let mut path = "/api/v1/accounts/lookup".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<entities::Account>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            [res.json.into()].to_vec(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_bookmarks(
        &self,
        options: Option<&megalodon::GetBookmarksInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = "/api/v1/bookmarks".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_favourites(
        &self,
        options: Option<&megalodon::GetFavouritesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = "/api/v1/favourites".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_mutes(
        &self,
        _options: Option<&megalodon::GetMutesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_blocks(
        &self,
        options: Option<&megalodon::GetBlocksInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = "/api/v1/blocks".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Account>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_domain_blocks(
        &self,
        _options: Option<&megalodon::GetDomainBlocksInputOptions>,
    ) -> Result<Response<Vec<String>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn block_domain(&self, _domain: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unblock_domain(&self, _domain: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_filters(&self) -> Result<Response<Vec<MegalodonEntities::Filter>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_filter(&self, _id: String) -> Result<Response<MegalodonEntities::Filter>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn create_filter(
        &self,
        _phrase: String,
        _context: Vec<MegalodonEntities::filter::FilterContext>,
        _options: Option<&megalodon::FilterInputOptions>,
    ) -> Result<Response<MegalodonEntities::Filter>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn update_filter(
        &self,
        _id: String,
        _phrase: String,
        _context: Vec<MegalodonEntities::filter::FilterContext>,
        _options: Option<&megalodon::FilterInputOptions>,
    ) -> Result<Response<MegalodonEntities::Filter>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_filter(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn report(
        &self,
        account_id: String,
        options: Option<&megalodon::ReportInputOptions>,
    ) -> Result<Response<MegalodonEntities::Report>, Error> {
        let mut params =
            HashMap::<&str, Value>::from([("account_id", serde_json::Value::String(account_id))]);
        if let Some(options) = options {
            if let Some(status_ids) = &options.status_ids {
                if let Some(json_status_ids) = serde_json::to_value(&status_ids).ok() {
                    params.insert("status_ids", json_status_ids);
                }
            }
            if let Some(comment) = &options.comment {
                params.insert("comment", Value::String(comment.to_string()));
            }
            if let Some(forward) = &options.forward {
                params.insert("forward", Value::String(forward.to_string()));
            }
            if let Some(category) = &options.category {
                params.insert("category", Value::String(category.to_string()));
            }
            if let Some(rule_ids) = &options.rule_ids {
                if let Some(json_rule_ids) = serde_json::to_value(&rule_ids).ok() {
                    params.insert("rule_ids", json_rule_ids);
                }
            }
        }
        let res = self
            .client
            .post::<entities::Report>("/api/v1/reports", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Report>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_follow_requests(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<FollowRequestOutput>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        let mut path = "/api/v1/follow_requests".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }

        let res = self
            .client
            .get::<Vec<entities::Account>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<FollowRequestOutput>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn accept_follow_request(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/follow_requests/{}/authorize", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn reject_follow_request(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/follow_requests/{}/reject", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_endorsements(
        &self,
        _options: Option<&megalodon::GetEndorsementsInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_featured_tags(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::FeaturedTag>>, Error> {
        Ok(Response::<Vec<MegalodonEntities::FeaturedTag>>::new(
            [].to_vec(),
            200,
            "200".to_string(),
            HeaderMap::new(),
        ))
    }

    async fn create_featured_tag(
        &self,
        _name: String,
    ) -> Result<Response<MegalodonEntities::FeaturedTag>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_featured_tag(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_suggested_tags(&self) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_preferences(&self) -> Result<Response<MegalodonEntities::Preferences>, Error> {
        let res = self
            .client
            .get::<entities::Preferences>("/api/v1/preferences", None)
            .await?;

        Ok(Response::<MegalodonEntities::Preferences>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_followed_tags(&self) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_suggestions(
        &self,
        _limit: Option<u32>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn follow_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unfollow_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn post_status(
        &self,
        status: String,
        options: Option<&megalodon::PostStatusInputOptions>,
    ) -> Result<Response<megalodon::PostStatusOutput>, Error> {
        let mut params =
            HashMap::<&str, Value>::from([("status", serde_json::Value::String(status))]);
        if let Some(options) = options {
            if let Some(media_ids) = &options.media_ids {
                if let Some(json_media_ids) = serde_json::to_value(media_ids).ok() {
                    params.insert("media_ids", json_media_ids);
                }
            }
            if let Some(in_reply_to_id) = &options.in_reply_to_id {
                params.insert(
                    "in_reply_to_id",
                    serde_json::Value::String(in_reply_to_id.to_string()),
                );
            }
            if let Some(sensitive) = options.sensitive {
                params.insert(
                    "sensitive",
                    serde_json::Value::String(sensitive.to_string()),
                );
            }
            if let Some(spoiler_text) = &options.spoiler_text {
                params.insert(
                    "spoiler_text",
                    serde_json::Value::String(spoiler_text.clone()),
                );
            }
            if let Some(visibility) = &options.visibility {
                params.insert(
                    "visibility",
                    serde_json::to_value(visibility.to_string()).unwrap(),
                );
            }
            if let Some(scheduled_at) = options.scheduled_at {
                params.insert(
                    "scheduled_at",
                    serde_json::to_value(scheduled_at.to_rfc3339()).unwrap(),
                );
            }
            if let Some(language) = &options.language {
                params.insert("language", serde_json::Value::String(language.clone()));
            }

            if let Some(poll) = &options.poll {
                params.insert("poll", serde_json::to_value(&poll).unwrap());
            }
        }

        if params.contains_key("scheduled_at") {
            let res = self
                .client
                .post::<entities::ScheduledStatus>("/api/v1/statuses", &params, None)
                .await?;

            Ok(Response::<megalodon::PostStatusOutput>::new(
                res.json.into(),
                res.status,
                res.status_text,
                res.header,
            ))
        } else {
            let res = self
                .client
                .post::<entities::Status>("/api/v1/statuses", &params, None)
                .await?;

            Ok(Response::<megalodon::PostStatusOutput>::new(
                res.json.into(),
                res.status,
                res.status_text,
                res.header,
            ))
        }
    }

    async fn get_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let res = self
            .client
            .get::<entities::Status>(format!("/api/v1/statuses/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn edit_status(
        &self,
        _id: String,
        _options: &megalodon::EditStatusInputOptions,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_status(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .delete::<()>(format!("/api/v1/statuses/{}", id).as_str(), &params, None)
            .await?;

        Ok(res)
    }

    async fn get_status_context(
        &self,
        id: String,
        options: Option<&megalodon::GetStatusContextInputOptions>,
    ) -> Result<Response<MegalodonEntities::Context>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("sinde_id={}", since_id));
            }
        }
        let mut path = format!("/api/v1/statuses/{}/context", id).to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<entities::Context>(path.as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Context>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_status_reblogged_by(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_status_favourited_by(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn favourite_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/favourite", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unfavourite_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/unfavourite", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn reblog_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/reblog", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unreblog_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/unreblog", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn bookmark_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/bookmark", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unbookmark_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/unbookmark", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn mute_status(&self, _id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unmute_status(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn pin_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/pin", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn unpin_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/unpin", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn upload_media(
        &self,
        file_path: String,
        options: Option<&megalodon::UploadMediaInputOptions>,
    ) -> Result<Response<MegalodonEntities::UploadMedia>, Error> {
        let file = File::open(file_path.clone()).await?;

        let file_name = hex::encode(Sha1::digest(file_path.as_bytes()));

        let stream = FramedRead::new(file, BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(file_body).file_name(file_name);

        let mut form = reqwest::multipart::Form::new().part("file", part);
        if let Some(options) = options {
            if let Some(description) = &options.description {
                form = form.text("description", description.clone());
            }
            if let Some(focus) = &options.focus {
                form = form.text("focus", focus.clone());
            }
        }

        let res = self
            .client
            .post_multipart::<entities::Attachment>("/api/v2/media", form, None)
            .await?;

        Ok(Response::<MegalodonEntities::UploadMedia>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_media(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Attachment>, Error> {
        let res = self
            .client
            .get::<entities::Attachment>(format!("/api/v1/media/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Attachment>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn update_media(
        &self,
        id: String,
        options: Option<&megalodon::UpdateMediaInputOptions>,
    ) -> Result<Response<MegalodonEntities::Attachment>, Error> {
        let mut form = reqwest::multipart::Form::new();
        if let Some(options) = options {
            if let Some(description) = &options.description {
                form = form.text("description", description.clone());
            }
            if let Some(focus) = &options.focus {
                form = form.text("focus", focus.clone());
            }
            if let Some(file_path) = &options.file_path {
                let file = File::open(file_path).await?;

                let file_name = hex::encode(Sha1::digest(file_path.as_bytes()));

                let stream = FramedRead::new(file, BytesCodec::new());
                let file_body = reqwest::Body::wrap_stream(stream);
                let part = reqwest::multipart::Part::stream(file_body).file_name(file_name);
                form = form.part("file", part);
            }
        }

        let res = self
            .client
            .put_multipart::<entities::Attachment>(
                format!("/api/v1/media/{}", id).as_str(),
                form,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Attachment>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_poll(&self, _id: String) -> Result<Response<MegalodonEntities::Poll>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn vote_poll(
        &self,
        _id: String,
        _choices: Vec<u32>,
    ) -> Result<Response<MegalodonEntities::Poll>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_scheduled_statuses(
        &self,
        _options: Option<&megalodon::GetScheduledStatusesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::ScheduledStatus>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_scheduled_status(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::ScheduledStatus>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn schedule_status(
        &self,
        _id: String,
        _scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<Response<MegalodonEntities::ScheduledStatus>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn cancel_scheduled_status(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_public_timeline(
        &self,
        options: Option<&megalodon::GetPublicTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::from([format!("local={}", false)]);
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.push(format!("only_media={}", only_media));
            }
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = "/api/v1/timelines/public".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_local_timeline(
        &self,
        options: Option<&megalodon::GetLocalTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::from([format!("local={}", true)]);
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.push(format!("only_media={}", only_media));
            }
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = "/api/v1/timelines/public".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_tag_timeline(
        &self,
        hashtag: String,
        options: Option<&megalodon::GetTagTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.push(format!("only_media={}", only_media));
            }
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
            if let Some(local) = options.local {
                params.push(format!("local={}", local));
            }
        }
        let mut path = format!("/api/v1/timelines/tag/{}", hashtag);
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_home_timeline(
        &self,
        options: Option<&megalodon::GetHomeTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.push(format!("only_media={}", only_media));
            }
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
            if let Some(local) = options.local {
                params.push(format!("local={}", local));
            }
        }
        let mut path = "/api/v1/timelines/home".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_list_timeline(
        &self,
        list_id: String,
        options: Option<&megalodon::GetListTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = format!("/api/v1/timelines/list/{}", list_id);
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Status>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_conversation_timeline(
        &self,
        _options: Option<&megalodon::GetConversationTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_conversation(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn read_conversation(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Conversation>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_lists(&self) -> Result<Response<Vec<MegalodonEntities::List>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::List>>("/api/v1/lists", None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::List>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_list(&self, id: String) -> Result<Response<MegalodonEntities::List>, Error> {
        let res = self
            .client
            .get::<entities::List>(format!("/api/v1/lists/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn create_list(&self, title: String) -> Result<Response<MegalodonEntities::List>, Error> {
        let params = HashMap::<&str, Value>::from([("title", serde_json::Value::String(title))]);
        let res = self
            .client
            .post::<entities::List>("/api/v1/lists", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn update_list(
        &self,
        id: String,
        title: String,
    ) -> Result<Response<MegalodonEntities::List>, Error> {
        let params = HashMap::<&str, Value>::from([("title", serde_json::Value::String(title))]);
        let res = self
            .client
            .put::<entities::List>(format!("/api/v1/lists/{}", id).as_str(), &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_list(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .delete::<()>(format!("/api/v1/lists/{}", id).as_str(), &params, None)
            .await?;

        Ok(res)
    }

    async fn get_accounts_in_list(
        &self,
        id: String,
        options: Option<&megalodon::GetAccountsInListInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
        }
        let mut path = format!("/api/v1/lists/{}/accounts", id);
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Account>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn add_accounts_to_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<MegalodonEntities::List>, Error> {
        let params = HashMap::<&str, Value>::from([(
            "account_ids",
            serde_json::to_value(&account_ids).ok().unwrap_or_default(),
        )]);
        let res = self
            .client
            .post::<entities::List>(
                format!("/api/v1/lists/{}/accounts", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_accounts_from_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([(
            "account_ids",
            serde_json::to_value(&account_ids).ok().unwrap_or_default(),
        )]);
        let res = self
            .client
            .delete::<()>(
                format!("/api/v1/lists/{}/accounts", id).as_str(),
                &params,
                None,
            )
            .await?;
        Ok(res)
    }

    async fn get_markers(
        &self,
        timeline: Vec<String>,
    ) -> Result<Response<MegalodonEntities::Marker>, Error> {
        let params: Vec<String> = timeline
            .into_iter()
            .map(|t| format!("timeline[]={}", t))
            .collect();

        let mut path = "/api/v1/markers".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<entities::Marker>(path.as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Marker>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn save_markers(
        &self,
        options: Option<&megalodon::SaveMarkersInputOptions>,
    ) -> Result<Response<MegalodonEntities::Marker>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(home) = &options.home {
                if let Some(json_home) = serde_json::to_value(&home).ok() {
                    params.insert("home", json_home);
                }
            }
            if let Some(notifications) = &options.notifications {
                if let Some(json_notifications) = serde_json::to_value(&notifications).ok() {
                    params.insert("notifications", json_notifications);
                }
            }
        }
        let res = self
            .client
            .post::<entities::Marker>("/api/v1/makers", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Marker>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_notifications(
        &self,
        options: Option<&megalodon::GetNotificationsInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Notification>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(since_id) = &options.since_id {
                params.push(format!("since_id={}", since_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
            if let Some(exclude_types) = &options.exclude_types {
                params.push(format!(
                    "exclude_types={}",
                    serde_json::to_string(exclude_types).unwrap()
                ));
            }
            if let Some(account_id) = &options.account_id {
                params.push(format!("account_id={}", account_id));
            }
        }
        let mut path = "/api/v1/notifications".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::Notification>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Notification>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_notification(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Notification>, Error> {
        let res = self
            .client
            .get::<entities::Notification>(format!("/api/v1/notifications/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Notification>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn dismiss_notifications(&self) -> Result<Response<()>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<()>("/api/v1/notifications/clear", &params, None)
            .await?;
        Ok(res)
    }

    async fn dismiss_notification(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn subscribe_push_notification(
        &self,
        _subscription: &megalodon::SubscribePushNotificationInputSubscription,
        _data: Option<&megalodon::SubscribePushNotificationInputData>,
    ) -> Result<Response<MegalodonEntities::PushSubscription>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_push_subscription(
        &self,
    ) -> Result<Response<MegalodonEntities::PushSubscription>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn update_push_subscription(
        &self,
        _data: Option<&megalodon::SubscribePushNotificationInputData>,
    ) -> Result<Response<MegalodonEntities::PushSubscription>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_push_subscription(&self) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn search(
        &self,
        q: String,
        r#type: &megalodon::SearchType,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<MegalodonEntities::Results>, Error> {
        let mut params = Vec::<String>::from([format!("q={}", q), format!("type={}", r#type)]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.push(format!("limit={}", limit));
            }
            if let Some(max_id) = &options.max_id {
                params.push(format!("max_id={}", max_id));
            }
            if let Some(min_id) = &options.min_id {
                params.push(format!("min_id={}", min_id));
            }
            if let Some(resolve) = options.resolve {
                params.push(format!("resolve={}", resolve));
            }
            if let Some(offset) = options.offset {
                params.push(format!("offset={}", offset));
            }
            if let Some(following) = options.following {
                params.push(format!("following={}", following));
            }
            if let Some(exclude_unreviewed) = options.exclude_unreviewed {
                params.push(format!("exclude_unreviewed={}", exclude_unreviewed));
            }
        }
        let mut path = "/api/v2/search".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<entities::Results>(path.as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Results>::new(
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

    async fn get_instance_peers(&self) -> Result<Response<Vec<String>>, Error> {
        let res = self
            .client
            .get::<Vec<String>>("/api/v1/instance/peers", None)
            .await?;
        Ok(res)
    }

    async fn get_instance_activity(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Activity>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_trends(
        &self,
        _limit: Option<u32>,
    ) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_directory(
        &self,
        _options: Option<&megalodon::GetInstanceDirectoryInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_custom_emojis(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Emoji>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::Emoji>>("/api/v1/custom_emojis", None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Emoji>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_instance_announcements(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Announcement>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn dismiss_instance_announcement(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn add_reaction_to_announcement(
        &self,
        _id: String,
        _name: String,
    ) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn remove_reaction_from_announcement(
        &self,
        _id: String,
        _name: String,
    ) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn create_emoji_reaction(
        &self,
        _id: String,
        _emoji: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_emoji_reaction(
        &self,
        _id: String,
        _emoji: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_emoji_reactions(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Reaction>>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_emoji_reaction(
        &self,
        _id: String,
        _emoji: String,
    ) -> Result<Response<MegalodonEntities::Reaction>, Error> {
        Err(Error::new_own(
            "Gotosocial doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    fn user_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::new();
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("user"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn public_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::new();
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("public"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn local_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::new();
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("public:local"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn direct_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::new();
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("direct"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn tag_streaming(
        &self,
        streaming_url: String,
        tag: String,
    ) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::from([format!("tag={}", tag)]);
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("hashtag"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn list_streaming(
        &self,
        streaming_url: String,
        list_id: String,
    ) -> Box<dyn Streaming + Send + Sync> {
        let params = Vec::<String>::from([format!("list={}", list_id)]);
        let c = WebSocket::new(
            streaming_url + "/api/v1/streaming",
            String::from("list"),
            Some(params),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }
}
