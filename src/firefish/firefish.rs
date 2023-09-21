use async_trait::async_trait;
use chrono::{DateTime, Utc};
use rand::RngCore;
use serde_json::Value;
use sha1::{Digest, Sha1};
use std::collections::HashMap;
use tokio::io::AsyncRead;
use tokio_util::codec::{BytesCodec, FramedRead};

use super::{
    api_client::{APIClient, DEFAULT_SCOPES},
    entities, oauth,
    web_socket::WebSocket,
};
use crate::{
    entities as MegalodonEntities,
    error::{self, Error},
    megalodon::{self, FollowRequestOutput},
    oauth as MegalodonOAuth,
    response::Response,
    Streaming,
};

/// Firefish API Client which satisfies megalodon trait.
#[derive(Debug, Clone)]
pub struct Firefish {
    client: APIClient,
    base_url: String,
    access_token: Option<String>,
    user_agent: Option<String>,
}

impl Firefish {
    /// Create a new [`Firefish`].
    pub fn new(
        base_url: String,
        access_token: Option<String>,
        user_agent: Option<String>,
    ) -> Firefish {
        let client = APIClient::new(base_url.clone(), access_token.clone(), user_agent.clone());
        Firefish {
            client,
            base_url,
            access_token,
            user_agent,
        }
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

    async fn get_relationship(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::Relation>("/api/users/relation", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::Relationship>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn vote_single_poll(&self, id: String, choice: u32) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([
            ("noteId", Value::String(id)),
            ("choice", serde_json::Number::from(choice).into()),
        ]);
        let res = self
            .client
            .post::<()>("/api/notes/polls/vote", &params, None)
            .await?;
        Ok(res)
    }

    async fn add_account_to_list(
        &self,
        id: String,
        account_id: String,
    ) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([
            ("listId", Value::String(id)),
            ("userId", Value::String(account_id)),
        ]);
        let res = self
            .client
            .post::<()>("/api/users/lists/push", &params, None)
            .await?;
        Ok(res)
    }

    async fn delete_account_from_list(
        &self,
        id: String,
        account_id: String,
    ) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([
            ("listId", Value::String(id)),
            ("userId", Value::String(account_id)),
        ]);
        let res = self
            .client
            .post::<()>("/api/users/lists/pull", &params, None)
            .await?;
        Ok(res)
    }

    async fn search_accounts(
        &self,
        q: String,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<Vec<entities::UserDetail>>, Error> {
        let mut params = HashMap::<&str, Value>::from([
            ("query", Value::String(q)),
            ("detail", Value::Bool(true)),
        ]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(offset) = options.offset {
                params.insert("offset", serde_json::Number::from(offset).into());
            }
            if let Some(resolve) = options.resolve {
                if resolve == false {
                    params.insert("origin", Value::String("local".to_string()));
                }
            }
        }
        let res = self
            .client
            .post::<Vec<entities::UserDetail>>("/api/users/search", &params, None)
            .await?;
        Ok(res)
    }

    async fn search_hashtags(
        &self,
        q: String,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<Vec<entities::Hashtag>>, Error> {
        let mut params = HashMap::<&str, Value>::from([("query", Value::String(q))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(offset) = options.offset {
                params.insert("offset", serde_json::Number::from(offset).into());
            }
        }
        let res = self
            .client
            .post::<Vec<String>>("/api/hashtags/search", &params, None)
            .await?;
        Ok(Response::<Vec<entities::Hashtag>>::new(
            res.json
                .into_iter()
                .map(|h| entities::Hashtag { tag: h })
                .collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn search_statuses(
        &self,
        q: String,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<Vec<entities::Note>>, Error> {
        let mut params = HashMap::<&str, Value>::from([("query", Value::String(q))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(offset) = options.offset {
                params.insert("offset", serde_json::Number::from(offset).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.to_string()));
            }
            if let Some(since_id) = &options.min_id {
                params.insert("sinceId", Value::String(since_id.to_string()));
            }
            if let Some(account_id) = &options.account_id {
                params.insert("userId", Value::String(account_id.to_string()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/search", &params, None)
            .await?;
        Ok(res)
    }

    async fn search_all(
        &self,
        q: String,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<MegalodonEntities::Results>, Error> {
        let accounts = self.search_accounts(q.clone(), options).await?;
        let hashtags = self.search_hashtags(q.clone(), options).await?;
        let statuses = self.search_statuses(q, options).await?;
        Ok(Response::<MegalodonEntities::Results>::new(
            MegalodonEntities::Results {
                accounts: accounts.json.into_iter().map(|i| i.into()).collect(),
                statuses: statuses.json.into_iter().map(|i| i.into()).collect(),
                hashtags: hashtags.json.into_iter().map(|i| i.into()).collect(),
            },
            accounts.status,
            accounts.status_text,
            accounts.header,
        ))
    }
}

#[async_trait]
impl megalodon::Megalodon for Firefish {
    async fn register_app(
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

    async fn fetch_access_token(
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

    async fn refresh_access_token(
        &self,
        _client_id: String,
        _client_secret: String,
        _refresh_token: String,
    ) -> Result<MegalodonOAuth::TokenData, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn revoke_access_token(
        &self,
        _client_id: String,
        _client_secret: String,
        _access_token: String,
    ) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn verify_app_credentials(
        &self,
    ) -> Result<Response<MegalodonEntities::Application>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn register_account(
        &self,
        _username: String,
        _email: String,
        _password: String,
        _agreement: String,
        _locale: String,
        _reason: Option<String>,
    ) -> Result<Response<MegalodonEntities::Token>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn verify_account_credentials(
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

    async fn update_credentials(
        &self,
        options: Option<&megalodon::UpdateCredentialsInputOptions>,
    ) -> Result<Response<MegalodonEntities::Account>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(bot) = options.bot {
                params.insert("isBot", Value::Bool(bot));
            }
            if let Some(display_name) = &options.display_name {
                params.insert("name", Value::String(display_name.clone()));
            }
            if let Some(note) = &options.note {
                params.insert("description", Value::String(note.clone()));
            }
            if let Some(locked) = options.locked {
                params.insert("isLocked", Value::Bool(locked));
            }
            if let Some(source) = &options.source {
                if let Some(language) = &source.language {
                    params.insert("lang", Value::String(language.clone()));
                }
                if let Some(sensitive) = source.sensitive {
                    params.insert("alwaysMarkNsfw", Value::Bool(sensitive));
                }
            }
        }
        let res = self
            .client
            .post::<entities::UserDetail>("/api/i/update", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::Account>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account(&self, id: String) -> Result<Response<MegalodonEntities::Account>, Error> {
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::UserDetail>("/api/users/show", &params, None)
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
        let mut params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        if let Some(options) = options {
            if let Some(_pinned) = options.pinned {
                let res = self
                    .client
                    .post::<entities::UserDetail>("/api/users/show", &params, None)
                    .await?;

                return Ok(Response::<Vec<MegalodonEntities::Status>>::new(
                    res.json
                        .pinned_notes
                        .into_iter()
                        .map(|p| p.into())
                        .collect(),
                    res.status,
                    res.status_text,
                    res.header,
                ));
            }
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(exclude_replies) = options.exclude_replies {
                params.insert("includeReplies", Value::Bool(!exclude_replies));
            }
            if let Some(exclude_reblogs) = options.exclude_reblogs {
                params.insert("includeMyRenotes", Value::Bool(!exclude_reblogs));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/users/notes", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn subscribe_account(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unsubscribe_account(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_account_followers(
        &self,
        id: String,
        options: Option<&megalodon::AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Follow>>("/api/users/followers", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.follower.into()).collect(),
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
        let mut params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Follow>>("/api/users/following", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.followee.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_account_lists(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::List>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_identity_proofs(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::IdentityProof>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn follow_account(
        &self,
        id: String,
        _options: Option<&megalodon::FollowAccountInputOptions>,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<entities::User>("/api/following/create", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<entities::User>("/api/following/delete", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<entities::UserDetail>("/api/blocking/create", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<entities::UserDetail>("/api/blocking/delete", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        _notifications: bool,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<()>("/api/mute/create", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<()>("/api/mute/delete", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        _id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_relationships(
        &self,
        ids: Vec<String>,
    ) -> Result<Response<Vec<MegalodonEntities::Relationship>>, Error> {
        let mut relations = [].to_vec();
        for id in ids.into_iter() {
            let rel = self.get_relationship(id).await?;
            relations.extend([rel.json]);
        }
        Ok(Response::<Vec<MegalodonEntities::Relationship>>::new(
            relations,
            200,
            "200".to_string(),
            reqwest::header::HeaderMap::default(),
        ))
    }

    async fn search_account(
        &self,
        q: String,
        options: Option<&megalodon::SearchAccountInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = HashMap::<&str, Value>::from([
            ("query", Value::String(q)),
            ("detail", Value::Bool(true)),
        ]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(resolve) = options.resolve {
                if resolve == false {
                    params.insert("origin", Value::String("local".to_string()));
                }
            }
        }
        let res = self
            .client
            .post::<Vec<entities::UserDetail>>("/api/users/search", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_bookmarks(
        &self,
        _options: Option<&megalodon::GetBookmarksInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_favourites(
        &self,
        options: Option<&megalodon::GetFavouritesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.min_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Favorite>>("/api/i/favorites", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.note.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_mutes(
        &self,
        options: Option<&megalodon::GetMutesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Mute>>("/api/mute/list", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.mutee.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_blocks(
        &self,
        options: Option<&megalodon::GetBlocksInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Blocking>>("/api/blocking/list", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.blockee.into()).collect(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn block_domain(&self, _domain: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unblock_domain(&self, _domain: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_filters(&self) -> Result<Response<Vec<MegalodonEntities::Filter>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_filter(&self, _id: String) -> Result<Response<MegalodonEntities::Filter>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_filter(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn report(
        &self,
        user_id: String,
        options: Option<&megalodon::ReportInputOptions>,
    ) -> Result<Response<MegalodonEntities::Report>, Error> {
        let mut params = HashMap::<&str, Value>::from([("userId", Value::String(user_id))]);
        let mut report = MegalodonEntities::Report {
            id: "".to_string(),
            action_taken: false,
            action_taken_at: None,
            status_ids: None,
            rule_ids: None,
            category: None,
            comment: None,
            forwarded: None,
            target_account: None,
        };
        if let Some(options) = options {
            if let Some(comment) = &options.comment {
                report.comment = Some(comment.clone());
                params.insert("comment", Value::String(comment.clone()));
            }
        }
        let res = self
            .client
            .post::<()>("/api/users/report-abuse", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::Report>::new(
            report,
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_follow_requests(
        &self,
        _limit: Option<u32>,
    ) -> Result<Response<Vec<FollowRequestOutput>>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<Vec<entities::FollowRequest>>("/api/following/requests/list", &params, None)
            .await?;
        Ok(Response::<Vec<FollowRequestOutput>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn accept_follow_request(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Relationship>, Error> {
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<()>("/api/following/requests/accept", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
        let params = HashMap::<&str, Value>::from([("userId", Value::String(id))]);
        let _ = self
            .client
            .post::<()>("/api/following/requests/reject", &params, None)
            .await?;
        let res = self
            .client
            .post::<entities::Relation>("/api/usres/relation", &params, None)
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_featured_tags(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::FeaturedTag>>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn create_featured_tag(
        &self,
        _name: String,
    ) -> Result<Response<MegalodonEntities::FeaturedTag>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_featured_tag(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_suggested_tags(&self) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_preferences(&self) -> Result<Response<MegalodonEntities::Preferences>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_followed_tags(&self) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_suggestions(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(limit) = limit {
            params.insert("limit", serde_json::Number::from(limit).into());
        }
        let res = self
            .client
            .post::<Vec<entities::UserDetail>>("/api/users/recommendation", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn follow_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unfollow_tag(&self, _id: String) -> Result<Response<MegalodonEntities::Tag>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            HashMap::<&str, Value>::from([("text", serde_json::Value::String(status))]);
        if let Some(options) = options {
            if let Some(media_ids) = &options.media_ids {
                if let Some(json_media_ids) = serde_json::to_value(media_ids).ok() {
                    params.insert("fileIds", json_media_ids);
                }
            }
            if let Some(poll) = &options.poll {
                let mut poll_params = HashMap::<&str, Value>::new();
                if let Some(json_options) = serde_json::to_value(poll.options.clone()).ok() {
                    poll_params.insert("choices", json_options);
                }
                if let Some(expires_in) = poll.expires_in {
                    poll_params.insert("expiredAfter", serde_json::Number::from(expires_in).into());
                }
                if let Some(multiple) = poll.multiple {
                    poll_params.insert("multiple", Value::Bool(multiple));
                }
                if let Some(json_poll) = serde_json::to_value(poll_params).ok() {
                    params.insert("poll", json_poll);
                }
            }
            if let Some(in_reply_to_id) = &options.in_reply_to_id {
                params.insert("replyId", Value::String(in_reply_to_id.clone()));
            }
            // TODO: This field should be applyed to files#is_sensitive.
            // if let Some(sensitivie) = options.sensitive {

            // }

            if let Some(spoiler_text) = &options.spoiler_text {
                params.insert("cw", Value::String(spoiler_text.clone()));
            }
            if let Some(visibility) = &options.visibility {
                let misskey_visibility: entities::note::StatusVisibility =
                    visibility.clone().into();
                params.insert("visibility", Value::String(misskey_visibility.to_string()));
            }
            if let Some(quote_id) = &options.quote_id {
                params.insert("renoteId", Value::String(quote_id.clone()));
            }
        }
        let res = self
            .client
            .post::<entities::CreatedNote>("/api/notes/create", &params, None)
            .await?;
        Ok(Response::<megalodon::PostStatusOutput>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::Note>("/api/notes/show", &params, None)
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
        id: String,
        options: &megalodon::EditStatusInputOptions,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let mut params = HashMap::<&str, Value>::from([("editId", serde_json::Value::String(id))]);
        if let Some(text) = &options.status {
            params.insert("text", Value::String(text.clone()));
        }
        if let Some(spoiler_text) = &options.spoiler_text {
            params.insert("cw", Value::String(spoiler_text.clone()));
        }
        if let Some(media_ids) = &options.media_ids {
            if let Some(json_media_ids) = serde_json::to_value(media_ids).ok() {
                params.insert("fileIds", json_media_ids);
            }
        }
        if let Some(poll) = &options.poll {
            let mut poll_params = HashMap::<&str, Value>::new();
            if let Some(json_options) = serde_json::to_value(poll.options.clone()).ok() {
                poll_params.insert("choices", json_options);
            }
            if let Some(expires_in) = poll.expires_in {
                poll_params.insert("expiredAfter", serde_json::Number::from(expires_in).into());
            }
            if let Some(multiple) = poll.multiple {
                poll_params.insert("multiple", Value::Bool(multiple));
            }
            if let Some(json_poll) = serde_json::to_value(poll_params).ok() {
                params.insert("poll", json_poll);
            }
        }
        let res = self
            .client
            .post::<entities::CreatedNote>("/api/notes/edit", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::Status>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_status(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id))]);
        let res = self
            .client
            .post::<()>("/api/notes/delete", &params, None)
            .await?;
        Ok(res)
    }

    async fn get_status_context(
        &self,
        id: String,
        options: Option<&megalodon::GetStatusContextInputOptions>,
    ) -> Result<Response<MegalodonEntities::Context>, Error> {
        let mut params = HashMap::<&str, Value>::from([("noteId", Value::String(id))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/children", &params, None)
            .await?;
        let context = MegalodonEntities::Context {
            ancestors: [].to_vec(),
            descendants: res.json.into_iter().map(|i| i.into()).collect(),
        };
        Ok(Response::<MegalodonEntities::Context>::new(
            context,
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_status_reblogged_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id))]);
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/renotes", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            res.json.into_iter().map(|i| i.user.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_status_favourited_by(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn favourite_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self
            .client
            .post::<()>("/api/notes/favorites/create", &params, None)
            .await?;
        let res = self.get_status(id).await?;
        Ok(res)
    }

    async fn unfavourite_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self
            .client
            .post::<()>("/api/notes/favorites/delete", &params, None)
            .await?;
        let res = self.get_status(id).await?;
        Ok(res)
    }

    async fn reblog_status(
        &self,
        id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("renoteId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::CreatedNote>("/api/notes/create", &params, None)
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
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self
            .client
            .post::<()>("/api/notes/unrenote", &params, None)
            .await?;
        let res = self.get_status(id).await?;
        Ok(res)
    }

    async fn bookmark_status(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn unbookmark_status(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn mute_status(&self, _id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        Err(Error::new_own(
            "Firefish doest not support".to_string(),
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
            "Firefish doest not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn pin_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self.client.post::<()>("/api/i/pin", &params, None).await?;
        let res = self.get_status(id).await?;
        Ok(res)
    }

    async fn unpin_status(&self, id: String) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self
            .client
            .post::<()>("/api/i/unpin", &params, None)
            .await?;
        let res = self.get_status(id).await?;
        Ok(res)
    }

    async fn upload_media_reader(
        &self,
        reader: Box<dyn AsyncRead + Sync + Send + Unpin>,
        options: Option<&megalodon::UploadMediaInputOptions>,
    ) -> Result<Response<MegalodonEntities::UploadMedia>, Error> {
        let mut file_name_unhash = [0; 32];
        rand::thread_rng().fill_bytes(&mut file_name_unhash);
        let file_name = hex::encode(Sha1::digest(file_name_unhash));

        let stream = FramedRead::new(reader, BytesCodec::new());
        let file_body = reqwest::Body::wrap_stream(stream);
        let part = reqwest::multipart::Part::stream(file_body).file_name(file_name);

        let mut form = reqwest::multipart::Form::new().part("file", part);
        if let Some(options) = options {
            if let Some(description) = &options.description {
                form = form.text("comment", description.clone());
            }
        }

        let res = self
            .client
            .post_multipart::<entities::File>("/api/drive/files/create", form, None)
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
        let params = HashMap::<&str, Value>::from([("fileId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::File>("/api/drive/files/show", &params, None)
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
        let mut params = HashMap::<&str, Value>::from([("fileId", Value::String(id))]);
        if let Some(options) = options {
            if let Some(description) = &options.description {
                params.insert("comment", Value::String(description.clone()));
            }
        }
        let res = self
            .client
            .post::<entities::File>("/api/drive/files/update", &params, None)
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn vote_poll(
        &self,
        id: String,
        choices: Vec<u32>,
        status_id: Option<String>,
    ) -> Result<Response<MegalodonEntities::Poll>, Error> {
        let Some(_status_id) = status_id else {
            return Err(Error::new_own(
                "status_is is required".to_string(),
                error::Kind::UnsatisfiedError,
                None,
                None,
            ));
        };
        for choice in choices.into_iter() {
            self.vote_single_poll(id.clone(), choice).await?;
        }
        let res = self.get_status(id).await?;
        Ok(Response::<MegalodonEntities::Poll>::new(
            res.json.poll.unwrap().into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_scheduled_statuses(
        &self,
        _options: Option<&megalodon::GetScheduledStatusesInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::ScheduledStatus>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn cancel_scheduled_status(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_public_timeline(
        &self,
        options: Option<&megalodon::GetPublicTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.insert("withFiles", Value::Bool(only_media));
            }
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/global-timeline", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_local_timeline(
        &self,
        options: Option<&megalodon::GetLocalTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.insert("withFiles", Value::Bool(only_media));
            }
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/local-timeline", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
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
        let mut params = HashMap::<&str, Value>::from([("tag", Value::String(hashtag))]);
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.insert("withFiles", Value::Bool(only_media));
            }
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/search-by-tag", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_home_timeline(
        &self,
        options: Option<&megalodon::GetHomeTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Status>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(only_media) = options.only_media {
                params.insert("withFiles", Value::Bool(only_media));
            }
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/timeline", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
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
        let mut params = HashMap::<&str, Value>::from([("listId", Value::String(list_id))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/user-list-timeline", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Status>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_conversation_timeline(
        &self,
        options: Option<&megalodon::GetConversationTimelineInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Conversation>>, Error> {
        let mut params =
            HashMap::<&str, Value>::from([("visibility", Value::String("specified".to_string()))]);
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Note>>("/api/notes/mentions", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Conversation>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_conversation(&self, _id: String) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_lists(&self) -> Result<Response<Vec<MegalodonEntities::List>>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<Vec<entities::List>>("/api/users/lists/list", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::List>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_list(&self, id: String) -> Result<Response<MegalodonEntities::List>, Error> {
        let params = HashMap::<&str, Value>::from([("listId", Value::String(id))]);
        let res = self
            .client
            .post::<entities::List>("/api/users/lists/show", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn create_list(&self, title: String) -> Result<Response<MegalodonEntities::List>, Error> {
        let params = HashMap::<&str, Value>::from([("name", Value::String(title))]);
        let res = self
            .client
            .post::<entities::List>("/api/lists/create", &params, None)
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
        let params = HashMap::<&str, Value>::from([
            ("listId", Value::String(id)),
            ("name", Value::String(title)),
        ]);
        let res = self
            .client
            .post::<entities::List>("/api/lists/update", &params, None)
            .await?;
        Ok(Response::<MegalodonEntities::List>::new(
            res.json.into(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn delete_list(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([("listId", Value::String(id))]);
        let res = self
            .client
            .post::<()>("/api/lists/delete", &params, None)
            .await?;
        Ok(res)
    }

    async fn get_accounts_in_list(
        &self,
        id: String,
        _options: Option<&megalodon::GetAccountsInListInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        let params = HashMap::<&str, Value>::from([("listId", Value::String(id))]);
        let list = self
            .client
            .post::<entities::List>("/api/users/lists/show", &params, None)
            .await?;
        let mut accounts = [].to_vec();
        if let Some(ids) = list.json.user_ids {
            for user_id in ids.into_iter() {
                let res = self.get_account(user_id).await?;
                accounts.extend([res.json]);
            }
        }
        Ok(Response::<Vec<MegalodonEntities::Account>>::new(
            accounts,
            200,
            "200".to_string(),
            reqwest::header::HeaderMap::new(),
        ))
    }

    async fn add_accounts_to_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<MegalodonEntities::List>, Error> {
        for account_id in account_ids.into_iter() {
            let _ = self.add_account_to_list(id.clone(), account_id).await?;
        }
        let res = self.get_list(id.clone()).await?;
        Ok(res)
    }

    async fn delete_accounts_from_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<()>, Error> {
        for account_id in account_ids.into_iter() {
            let _ = self
                .delete_account_from_list(id.clone(), account_id)
                .await?;
        }
        Ok(Response::<()>::new(
            (),
            200,
            "200".to_string(),
            reqwest::header::HeaderMap::new(),
        ))
    }

    async fn get_markers(
        &self,
        _timeline: Vec<String>,
    ) -> Result<Response<MegalodonEntities::Marker>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn save_markers(
        &self,
        _options: Option<&megalodon::SaveMarkersInputOptions>,
    ) -> Result<Response<MegalodonEntities::Marker>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_notifications(
        &self,
        options: Option<&megalodon::GetNotificationsInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Notification>>, Error> {
        let mut params = HashMap::<&str, Value>::new();
        if let Some(options) = options {
            if let Some(limit) = options.limit {
                params.insert("limit", serde_json::Number::from(limit).into());
            }
            if let Some(max_id) = &options.max_id {
                params.insert("untilId", Value::String(max_id.clone()));
            }
            if let Some(since_id) = &options.since_id {
                params.insert("sinceId", Value::String(since_id.clone()));
            }
            if let Some(min_id) = &options.min_id {
                params.insert("sinceId", Value::String(min_id.clone()));
            }
            if let Some(exclude_types) = &options.exclude_types {
                let misskey_types: Vec<entities::notification::NotificationType> = exclude_types
                    .clone()
                    .into_iter()
                    .map(|i| i.into())
                    .collect();
                if let Some(json_types) = serde_json::to_value(misskey_types).ok() {
                    params.insert("excludeTypes", json_types);
                }
            }
        }
        let res = self
            .client
            .post::<Vec<entities::Notification>>("/api/i/notifications", &params, None)
            .await?;
        let notifications: Vec<MegalodonEntities::Notification> = res
            .json
            .into_iter()
            .filter(|n| n.r#type != entities::notification::NotificationType::Unknown)
            .map(|n| n.into())
            .collect();
        Ok(Response::<Vec<MegalodonEntities::Notification>>::new(
            notifications,
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_notification(
        &self,
        _id: String,
    ) -> Result<Response<MegalodonEntities::Notification>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn dismiss_notifications(&self) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<()>("/api/notifications/mark-all-as-read", &params, None)
            .await?;
        Ok(res)
    }

    async fn dismiss_notification(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([("notificationId", Value::String(id))]);
        let res = self
            .client
            .post::<()>("/api/notifications/read", &params, None)
            .await?;
        Ok(res)
    }

    async fn subscribe_push_notification(
        &self,
        _subscription: &megalodon::SubscribePushNotificationInputSubscription,
        _data: Option<&megalodon::SubscribePushNotificationInputData>,
    ) -> Result<Response<MegalodonEntities::PushSubscription>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_push_subscription(
        &self,
    ) -> Result<Response<MegalodonEntities::PushSubscription>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn delete_push_subscription(&self) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn search(
        &self,
        q: String,
        options: Option<&megalodon::SearchInputOptions>,
    ) -> Result<Response<MegalodonEntities::Results>, Error> {
        let mut results = MegalodonEntities::Results {
            accounts: [].to_vec(),
            statuses: [].to_vec(),
            hashtags: [].to_vec(),
        };
        if let Some(options) = options {
            if let Some(search_type) = &options.r#type {
                match search_type {
                    megalodon::SearchType::Accounts => {
                        let res = self.search_accounts(q, Some(options)).await?;
                        results.accounts = res.json.into_iter().map(|i| i.into()).collect();
                    }
                    megalodon::SearchType::Hashtags => {
                        let res = self.search_hashtags(q, Some(options)).await?;
                        results.hashtags = res.json.into_iter().map(|i| i.into()).collect();
                    }
                    megalodon::SearchType::Statuses => {
                        let res = self.search_statuses(q, Some(options)).await?;
                        results.statuses = res.json.into_iter().map(|i| i.into()).collect();
                    }
                }
            } else {
                results = self.search_all(q, Some(options)).await?.json;
            }
        } else {
            results = self.search_all(q, options).await?.json;
        }

        Ok(Response::<MegalodonEntities::Results>::new(
            results,
            200,
            "200".to_string(),
            reqwest::header::HeaderMap::new(),
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
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_activity(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Activity>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_trends(
        &self,
        _limit: Option<u32>,
    ) -> Result<Response<Vec<MegalodonEntities::Tag>>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<Vec<entities::Hashtag>>("/api/hashtags/trend", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Tag>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_instance_directory(
        &self,
        _options: Option<&megalodon::GetInstanceDirectoryInputOptions>,
    ) -> Result<Response<Vec<MegalodonEntities::Account>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn get_instance_custom_emojis(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Emoji>>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<entities::Meta>("/api/meta", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Emoji>>::new(
            res.json.emojis.into_iter().map(|e| e.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn get_instance_announcements(
        &self,
    ) -> Result<Response<Vec<MegalodonEntities::Announcement>>, Error> {
        let params = HashMap::<&str, Value>::new();
        let res = self
            .client
            .post::<Vec<entities::Announcement>>("/api/announcements", &params, None)
            .await?;
        Ok(Response::<Vec<MegalodonEntities::Announcement>>::new(
            res.json.into_iter().map(|i| i.into()).collect(),
            res.status,
            res.status_text,
            res.header,
        ))
    }

    async fn dismiss_instance_announcement(&self, id: String) -> Result<Response<()>, Error> {
        let params = HashMap::<&str, Value>::from([("announcementId", Value::String(id))]);
        let res = self
            .client
            .post::<()>("/api/i/read-announcement", &params, None)
            .await?;
        Ok(res)
    }

    async fn add_reaction_to_announcement(
        &self,
        _id: String,
        _name: String,
    ) -> Result<Response<()>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    async fn create_emoji_reaction(
        &self,
        id: String,
        emoji: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([
            ("noteId", Value::String(id.clone())),
            ("reaction", Value::String(emoji)),
        ]);
        let _ = self
            .client
            .post::<()>("/api/notes/reactions/create", &params, None)
            .await?;
        let res = self.get_status(id.clone()).await?;
        Ok(res)
    }

    async fn delete_emoji_reaction(
        &self,
        id: String,
        _emoji: String,
    ) -> Result<Response<MegalodonEntities::Status>, Error> {
        let params = HashMap::<&str, Value>::from([("noteId", Value::String(id.clone()))]);
        let _ = self
            .client
            .post::<()>("/api/notes/reactions/delete", &params, None)
            .await?;
        let res = self.get_status(id.clone()).await?;
        Ok(res)
    }

    async fn get_emoji_reactions(
        &self,
        _id: String,
    ) -> Result<Response<Vec<MegalodonEntities::Reaction>>, Error> {
        Err(Error::new_own(
            "Firefish does not support".to_string(),
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
            "Firefish does not support".to_string(),
            error::Kind::NoImplementedError,
            None,
            None,
        ))
    }

    fn user_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("user"),
            None,
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn public_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("globalTimeline"),
            None,
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn local_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("localTimeline"),
            None,
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn direct_streaming(&self, streaming_url: String) -> Box<dyn Streaming + Send + Sync> {
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("conversation"),
            None,
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }

    fn tag_streaming(
        &self,
        streaming_url: String,
        _tag: String,
    ) -> Box<dyn Streaming + Send + Sync> {
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("hashtag"),
            None,
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
        let c = WebSocket::new(
            streaming_url + "/streaming",
            String::from("list"),
            Some(list_id),
            self.access_token.clone(),
            self.user_agent.clone(),
        );

        Box::new(c)
    }
}
