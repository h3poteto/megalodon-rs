use super::api_client::APIClient;
use super::entities;
use crate::megalodon::CredentialsOptions;
use crate::{
    default, entities as MegalodonEntities, error::Error, megalodon, oauth, response::Response,
};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, ClientId, ClientSecret, CsrfToken, RedirectUrl, ResponseType, Scope, TokenUrl,
};
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};

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

    async fn register_account(
        &self,
        username: String,
        email: String,
        password: String,
        agreement: String,
        locale: String,
        reason: Option<String>,
    ) -> Result<Response<MegalodonEntities::Token>, Error> {
        let mut params = HashMap::<&str, String>::from([
            ("username", username),
            ("email", email),
            ("password", password),
            ("agreement", agreement),
            ("locale", locale),
        ]);
        if let Some(reason) = reason {
            params.insert("reason", reason);
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
        options: Option<&CredentialsOptions>,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let mut params = HashMap::<&str, String>::new();
        if let Some(options) = options {
            if let Some(discoverable) = options.discoverable {
                params.insert("discoverable", discoverable.to_string());
            }
            if let Some(bot) = options.bot {
                params.insert("bot", bot.to_string());
            }
            if let Some(display_name) = &options.display_name {
                params.insert("display_name", display_name.clone());
            }
            if let Some(note) = &options.note {
                params.insert("note", note.clone());
            }
            if let Some(avatar) = &options.avatar {
                params.insert("avatar", avatar.clone());
            }
            if let Some(header) = &options.header {
                params.insert("header", header.clone());
            }
            if let Some(locked) = options.locked {
                params.insert("locked", locked.to_string());
            }
            if let Some(source) = &options.source {
                params.insert("source", serde_json::to_string(&source).unwrap());
            }
            if let Some(fields_attributes) = &options.fields_attributes {
                params.insert(
                    "fields_attributes",
                    serde_json::to_string(&fields_attributes).unwrap(),
                );
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
        options: Option<&megalodon::AccountStatusesInputOptions>,
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
        let params = HashMap::<&str, String>::from([("notify", true.to_string())]);
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
        let params = HashMap::<&str, String>::from([("notify", false.to_string())]);
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
        id: String,
    ) -> Result<Response<Vec<MegalodonEntities::IdentityProof>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::IdentityProof>>(
                format!("/api/v1/accounts/{}/identity_proofs", id).as_ref(),
                None,
            )
            .await?;

        Ok(Response::<Vec<MegalodonEntities::IdentityProof>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn follow_account(
        &self,
        id: String,
        options: Option<&megalodon::FollowInputOptions>,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let mut params = HashMap::<&str, String>::new();
        if let Some(options) = options {
            if let Some(reblog) = options.reblog {
                params.insert("reblog", reblog.to_string());
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
        let params = HashMap::<&str, String>::new();
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
        let params = HashMap::<&str, String>::new();
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
        let params = HashMap::<&str, String>::new();
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
        id: String,
        notifications: bool,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, String>::from([("notifications", notifications.to_string())]);
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/mute", id).as_ref(),
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

    async fn unmute_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, String>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts{}/unmute", id).as_ref(),
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

    async fn pin_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, String>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/pin", id).as_ref(),
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

    async fn unpin_account(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, String>::new();
        let res = self
            .client
            .post::<entities::Relationship>(
                format!("/api/v1/accounts/{}/unpin", id).as_ref(),
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
        q: String,
        options: Option<&megalodon::SearchAccountInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::from([format!("q={}", q)]);
        if let Some(options) = options {
            if let Some(following) = options.following {
                params.push(format!("following={}", following));
            }
            if let Some(resolve) = options.resolve {
                params.push(format!("resolve={}", resolve));
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
        }
        let mut path = "/api/v1/accounts/search".to_string();
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
        options: Option<&megalodon::GetMutesInputOptions>,
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
        let mut path = "/api/v1/mutes".to_string();
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
        options: Option<&megalodon::GetDomainBlocksInputOptions>,
    ) -> Result<Response<Vec<String>>, Error> {
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
        let mut path = "/api/v1/domain_blocks".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self.client.get::<Vec<String>>(path.as_str(), None).await?;

        Ok(Response::<Vec<String>>::new(
            res.json,
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn block_domain(&self, domain: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, String>::from([("domain", domain)]);
        let res = self
            .client
            .post::<()>("/api/v1/domain_blocks", &params, None)
            .await?;

        Ok(res)
    }

    async fn unblock_domain(&self, domain: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, String>::from([("domain", domain)]);
        let res = self
            .client
            .delete::<()>("/api/v1/domain_blocks", &params, None)
            .await?;

        Ok(res)
    }

    async fn get_filters(&self) -> Result<Response<Vec<MegalodonEntities::Filter>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::Filter>>("/api/v1/filters", None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Filter>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_filter(&self, id: String) -> Result<Response<MegalodonEntities::Filter>, Error> {
        let res = self
            .client
            .get::<entities::Filter>(format!("/api/v1/filters/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Filter>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn create_filter(
        &self,
        phrase: String,
        context: Vec<MegalodonEntities::filter::FilterContext>,
        options: Option<&megalodon::FilterInputOptions>,
    ) -> Result<Response<MegalodonEntities::Filter>, Error> {
        let mut params = HashMap::<&str, String>::from([
            ("phrase", phrase),
            ("context", serde_json::to_string(&context).unwrap()),
        ]);
        if let Some(options) = options {
            if let Some(irreversible) = options.irreversible {
                params.insert("irreversible", irreversible.to_string());
            }
            if let Some(whole_word) = options.whole_word {
                params.insert("whole_word", whole_word.to_string());
            }
            if let Some(expires_in) = options.expires_in {
                params.insert("expires_in", expires_in.to_string());
            }
        }
        let res = self
            .client
            .post::<entities::Filter>("/api/v1/filters", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Filter>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn update_filter(
        &self,
        id: String,
        phrase: String,
        context: Vec<MegalodonEntities::filter::FilterContext>,
        options: Option<&megalodon::FilterInputOptions>,
    ) -> Result<Response<MegalodonEntities::Filter>, Error> {
        let mut params = HashMap::<&str, String>::from([
            ("phrase", phrase),
            ("context", serde_json::to_string(&context).unwrap()),
        ]);
        if let Some(options) = options {
            if let Some(irreversible) = options.irreversible {
                params.insert("irreversible", irreversible.to_string());
            }
            if let Some(whole_word) = options.whole_word {
                params.insert("whole_word", whole_word.to_string());
            }
            if let Some(expires_in) = options.expires_in {
                params.insert("expires_in", expires_in.to_string());
            }
        }
        let res = self
            .client
            .put::<entities::Filter>(format!("/api/v1/filters/{}", id).as_str(), &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Filter>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_filter(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, String>::new();
        let res = self
            .client
            .delete::<()>(format!("/api/v1/filters/{}", id).as_str(), &params, None)
            .await?;

        Ok(res)
    }

    async fn report(
        &self,
        account_id: String,
        comment: String,
        options: Option<&megalodon::ReportInputOptions>,
    ) -> Result<Response<MegalodonEntities::Report>, Error> {
        let mut params =
            HashMap::<&str, String>::from([("account_id", account_id), ("comment", comment)]);
        if let Some(options) = options {
            if let Some(status_ids) = &options.status_ids {
                params.insert("status_ids", serde_json::to_string(&status_ids).unwrap());
            }
            if let Some(forward) = options.forward {
                params.insert("forward", forward.to_string());
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
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
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

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
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
        options: Option<&megalodon::GetEndorsementsInputOptions>,
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
        let mut path = "/api/v1/endorsements".to_string();
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

    async fn get_featured_tags(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::FeaturedTag>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::FeaturedTag>>("/api/v1/featured_tags", None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::FeaturedTag>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn create_featured_tag(
        &self,
        name: String,
    ) -> Result<Response<MegalodonEntities::FeaturedTag>, Error> {
        let params = HashMap::<&str, String>::from([("name", name)]);
        let res = self
            .client
            .post::<entities::FeaturedTag>("/api/v1/featured_tags", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::FeaturedTag>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_featured_tag(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .delete::<()>(
                format!("/api/v1/featured_tags/{}", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(res)
    }

    async fn get_suggested_tags(&self) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::Tag>>("/api/v1/featured_tags/suggestions", None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Tag>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_suggestions(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = Vec::<String>::new();
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        let mut path = "/api/v1/suggestions".to_string();
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

    async fn post_status(
        &self,
        status: String,
        options: Option<&megalodon::PostStatusInputOptions>,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let mut params = HashMap::<&str, String>::from([("status", status)]);
        if let Some(options) = options {
            if let Some(media_ids) = &options.media_ids {
                params.insert("media_ids", serde_json::to_string(&media_ids).unwrap());
            }
            if let Some(in_reply_to_id) = &options.in_reply_to_id {
                params.insert("in_reply_to_id", in_reply_to_id.clone());
            }
            if let Some(sensitive) = options.sensitive {
                params.insert("sensitive", sensitive.to_string());
            }
            if let Some(spoiler_text) = &options.spoiler_text {
                params.insert("spoiler_text", spoiler_text.clone());
            }
            if let Some(visibility) = &options.visibility {
                params.insert("visibility", visibility.to_string());
            }
            if let Some(scheduled_at) = options.scheduled_at {
                params.insert("scheduled_at", scheduled_at.to_rfc3339());
            }
            if let Some(language) = &options.language {
                params.insert("language", language.clone());
            }
            if let Some(quote_id) = &options.quote_id {
                params.insert("quote_id", quote_id.clone());
            }
            if let Some(poll) = &options.poll {
                params.insert("poll", serde_json::to_string(&poll).unwrap());
            }
        }
        let res = self
            .client
            .post::<entities::Status>("/api/v1/statuses", &params, None)
            .await?;

        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
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
        id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::Account>>(
                format!("/api/v1/statuses/{}/reblogged_by", id).as_str(),
                None,
            )
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_status_favourited_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let res = self
            .client
            .get::<Vec<entities::Account>>(
                format!("/api/v1/statuses/{}/favourited_by", id).as_str(),
                None,
            )
            .await?;

        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
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

    async fn mute_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/mute", id).as_str(),
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

    async fn unmute_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .post::<entities::Status>(
                format!("/api/v1/statuses/{}/unmute", id).as_str(),
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
    ) -> Result<Response<MegalodonEntities::Attachment>, Error> {
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
            .post_multipart::<entities::Attachment>("/api/v1/media", form, None)
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

    async fn get_poll(&self, id: String) -> Result<Response<MegalodonEntities::Poll>, Error> {
        let res = self
            .client
            .get::<entities::Poll>(format!("/api/v1/polls/{}", id).as_str(), None)
            .await?;

        Ok(Response::<MegalodonEntities::Poll>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn vote_poll(
        &self,
        id: String,
        choices: Vec<u32>,
    ) -> Result<Response<MegalodonEntities::Poll>, Error> {
        let params =
            HashMap::<&str, String>::from([("choices", serde_json::to_string(&choices).unwrap())]);
        let res = self
            .client
            .post::<entities::Poll>(
                format!("/api/v1/polls/{}/votes", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::Poll>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_scheduled_statuses(
        &self,
        options: Option<&megalodon::GetScheduledStatusesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::ScheduledStatus>>, Error> {
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
        let mut path = "/api/v1/scheduled_statuses".to_string();
        if params.len() > 0 {
            path = path + "?" + params.join("&").as_str();
        }
        let res = self
            .client
            .get::<Vec<entities::ScheduledStatus>>(path.as_str(), None)
            .await?;

        Ok(Response::<Vec<MegalodonEntities::ScheduledStatus>>::new(
            res.json.into_iter().map(|j| j.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_scheduled_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::ScheduledStatus>, Error> {
        let res = self
            .client
            .get::<entities::ScheduledStatus>(
                format!("/api/v1/scheduled_statuses/{}", id).as_str(),
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::ScheduledStatus>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn schedule_status(
        &self,
        id: String,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<Response<MegalodonEntities::ScheduledStatus>, Error> {
        let mut params = HashMap::<&str, String>::new();
        if let Some(scheduled_at) = scheduled_at {
            params.insert("scheduled_at", scheduled_at.to_rfc3339());
        }
        let res = self
            .client
            .put::<entities::ScheduledStatus>(
                format!("/api/v1/scheduled_statuses/{}", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(Response::<MegalodonEntities::ScheduledStatus>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn cancel_scheduled_status(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::new();
        let res = self
            .client
            .delete::<()>(
                format!("/api/v1/scheduled_statuses/{}", id).as_str(),
                &params,
                None,
            )
            .await?;

        Ok(res)
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
