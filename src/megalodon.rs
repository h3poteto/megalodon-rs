use crate::error::Error;
use crate::oauth::{AppData, TokenData};
use crate::response::Response;
use crate::{entities, mastodon};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

    // ======================================
    // apps
    // ======================================

    async fn verify_app_credentials(&self) -> Result<Response<entities::Application>, Error>;

    // ======================================
    // accounts
    // ======================================

    async fn register_account(
        &self,
        username: String,
        email: String,
        password: String,
        agreement: String,
        locale: String,
        reason: Option<String>,
    ) -> Result<Response<entities::Token>, Error>;

    async fn verify_account_credentials(&self) -> Result<Response<entities::Account>, Error>;

    async fn update_credentials(
        &self,
        options: Option<&CredentialsOptions>,
    ) -> Result<Response<entities::Account>, Error>;

    async fn get_account(&self, id: String) -> Result<Response<entities::Account>, Error>;

    async fn get_account_statuses(
        &self,
        id: String,
        options: Option<&AccountStatusesInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn subscribe_account(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    async fn unsubscribe_account(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    async fn get_account_followers(
        &self,
        id: String,
        options: Option<&AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn get_account_following(
        &self,
        id: String,
        options: Option<&AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn get_account_lists(&self, id: String) -> Result<Response<Vec<entities::List>>, Error>;

    async fn get_identity_proofs(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::IdentityProof>>, Error>;

    async fn follow_account(
        &self,
        id: String,
        options: Option<&FollowInputOptions>,
    ) -> Result<Response<entities::Relationship>, Error>;

    async fn unfollow_account(&self, id: String)
        -> Result<Response<entities::Relationship>, Error>;

    async fn block_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    async fn unblock_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    async fn mute_account(
        &self,
        id: String,
        notifications: bool,
    ) -> Result<Response<entities::Relationship>, Error>;

    async fn unmute_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    async fn pin_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    async fn unpin_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    async fn get_relationships(
        &self,
        ids: Vec<String>,
    ) -> Result<Response<Vec<entities::Relationship>>, Error>;

    async fn search_account(
        &self,
        q: String,
        options: Option<&SearchAccountInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/bookmarks
    // ======================================
    async fn get_bookmarks(
        &self,
        options: Option<&GetBookmarksInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // accounts/favourites
    // ======================================
    async fn get_favourites(
        &self,
        options: Option<&GetFavouritesInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // accounts/mutes
    // ======================================
    async fn get_mutes(
        &self,
        options: Option<&GetMutesInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/blocks
    // ======================================
    async fn get_blocks(
        &self,
        options: Option<&GetBlocksInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/domain_blocks
    // ======================================
    async fn get_domain_blocks(
        &self,
        options: Option<&GetDomainBlocksInputOptions>,
    ) -> Result<Response<Vec<String>>, Error>;

    async fn block_domain(&self, domain: String) -> Result<Response<()>, Error>;

    async fn unblock_domain(&self, domain: String) -> Result<Response<()>, Error>;

    // ======================================
    // accounts/filters
    // ======================================
    async fn get_filters(&self) -> Result<Response<Vec<entities::Filter>>, Error>;

    async fn get_filter(&self, id: String) -> Result<Response<entities::Filter>, Error>;

    async fn create_filter(
        &self,
        phrase: String,
        context: Vec<entities::filter::FilterContext>,
        options: Option<&FilterInputOptions>,
    ) -> Result<Response<entities::Filter>, Error>;

    async fn update_filter(
        &self,
        id: String,
        phrase: String,
        context: Vec<entities::filter::FilterContext>,
        options: Option<&FilterInputOptions>,
    ) -> Result<Response<entities::Filter>, Error>;

    async fn delete_filter(&self, id: String) -> Result<Response<()>, Error>;

    // ======================================
    // accounts/reports
    // ======================================
    async fn report(
        &self,
        account_id: String,
        comment: String,
        options: Option<&ReportInputOptions>,
    ) -> Result<Response<entities::Report>, Error>;

    // ======================================
    // accounts/follow_requests
    // ======================================
    async fn get_follow_requests(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn accept_follow_request(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    async fn reject_follow_request(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    // ======================================
    // accounts/endorsements
    // ======================================
    async fn get_endorsements(
        &self,
        options: Option<&GetEndorsementsInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/featured_tags
    // ======================================
    async fn get_featured_tags(&self) -> Result<Response<Vec<entities::FeaturedTag>>, Error>;

    async fn create_featured_tag(
        &self,
        name: String,
    ) -> Result<Response<entities::FeaturedTag>, Error>;

    async fn delete_featured_tag(&self, id: String) -> Result<Response<()>, Error>;

    async fn get_suggested_tags(&self) -> Result<Response<Vec<entities::Tag>>, Error>;

    // ======================================
    // accounts/suggestions
    // ======================================
    async fn get_suggestions(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // statuses
    // ======================================
    async fn post_status(
        &self,
        status: String,
        options: Option<&PostStatusInputOptions>,
    ) -> Result<Response<entities::Status>, Error>;

    async fn get_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn delete_status(&self, id: String) -> Result<Response<()>, Error>;

    async fn get_status_context(
        &self,
        id: String,
        options: Option<&GetStatusContextInputOptions>,
    ) -> Result<Response<entities::Context>, Error>;

    async fn get_status_reblogged_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn get_status_favourited_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn favourite_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn unfavourite_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn reblog_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn unreblog_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn bookmark_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn unbookmark_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn mute_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn unmute_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn pin_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    async fn unpin_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    // ======================================
    // statuses/media
    // ======================================
    async fn upload_media(
        &self,
        file_path: String,
        options: Option<&UploadMediaInputOptions>,
    ) -> Result<Response<entities::Attachment>, Error>;

    async fn update_media(
        &self,
        id: String,
        options: Option<&UpdateMediaInputOptions>,
    ) -> Result<Response<entities::Attachment>, Error>;

    // ======================================
    // statuses/polls
    // ======================================
    async fn get_poll(&self, id: String) -> Result<Response<entities::Poll>, Error>;

    async fn vote_poll(
        &self,
        id: String,
        choices: Vec<u32>,
    ) -> Result<Response<entities::Poll>, Error>;

    // ======================================
    // statuses/scheduled_statuses
    // ======================================
    async fn get_scheduled_statuses(
        &self,
        options: Option<&GetScheduledStatusesInputOptions>,
    ) -> Result<Response<Vec<entities::ScheduledStatus>>, Error>;

    async fn get_scheduled_status(
        &self,
        id: String,
    ) -> Result<Response<entities::ScheduledStatus>, Error>;

    async fn schedule_status(
        &self,
        id: String,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<Response<entities::ScheduledStatus>, Error>;

    async fn cancel_scheduled_status(&self, id: String) -> Result<Response<()>, Error>;

    // ======================================
    // timeilnes
    // ======================================
    async fn get_public_timeline(
        &self,
        options: Option<&GetPublicTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn get_local_timeline(
        &self,
        options: Option<&GetLocalTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn get_tag_timeline(
        &self,
        hashtag: String,
        options: Option<&GetTagTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn get_home_timeline(
        &self,
        options: Option<&GetHomeTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn get_list_timeline(
        &self,
        list_id: String,
        options: Option<&GetListTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // timeilnes/conversations
    // ======================================
    async fn get_conversation_timeline(
        &self,
        options: Option<&GetConversationTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    async fn delete_conversation(&self, id: String) -> Result<Response<()>, Error>;

    async fn read_conversation(
        &self,
        id: String,
    ) -> Result<Response<entities::Conversation>, Error>;

    // ======================================
    // timeilnes/lists
    // ======================================
    async fn get_lists(&self) -> Result<Response<Vec<entities::List>>, Error>;

    async fn get_list(&self, id: String) -> Result<Response<entities::List>, Error>;

    async fn create_list(&self, title: String) -> Result<Response<entities::List>, Error>;

    async fn update_list(
        &self,
        id: String,
        title: String,
    ) -> Result<Response<entities::List>, Error>;

    async fn delete_list(&self, id: String) -> Result<Response<()>, Error>;

    async fn get_accounts_in_list(
        &self,
        id: String,
        options: Option<&GetAccountsInListInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    async fn add_accounts_to_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<entities::List>, Error>;

    async fn delete_accounts_from_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<()>, Error>;

    // ======================================
    // timeilnes/markers
    // ======================================
    async fn get_markers(&self, timeline: Vec<String>)
        -> Result<Response<entities::Marker>, Error>;

    async fn save_markers(
        &self,
        options: Option<&SaveMarkersInputOptions>,
    ) -> Result<Response<entities::Marker>, Error>;

    // ======================================
    // notifications
    // ======================================
    async fn get_notifications(
        &self,
        options: Option<&GetNotificationsInputOptions>,
    ) -> Result<Response<Vec<entities::Notification>>, Error>;

    async fn get_notification(&self, id: String)
        -> Result<Response<entities::Notification>, Error>;

    async fn dismiss_notifications(&self) -> Result<Response<()>, Error>;

    async fn dismiss_notification(&self, id: String) -> Result<Response<()>, Error>;

    async fn get_instance(&self) -> Result<Response<entities::Instance>, Error>;
}

pub struct AppInputOptions {
    pub scopes: Option<Vec<String>>,
    pub redirect_uris: Option<String>,
    pub website: Option<String>,
}

pub struct CredentialsOptions {
    pub discoverable: Option<bool>,
    pub bot: Option<bool>,
    pub display_name: Option<String>,
    pub note: Option<String>,
    pub avatar: Option<String>,
    pub header: Option<String>,
    pub locked: Option<bool>,
    pub source: Option<CredentialsSource>,
    pub fields_attributes: Option<Vec<CredentialsFieldAttribute>>,
}

#[derive(Debug, Serialize)]
pub struct CredentialsSource {
    pub privacy: Option<String>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CredentialsFieldAttribute {
    pub name: String,
    pub value: String,
}

pub struct AccountStatusesInputOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub pinned: Option<bool>,
    pub exclude_replies: Option<bool>,
    pub exclude_reblogs: Option<bool>,
    pub only_media: Option<bool>,
}

pub struct AccountFollowersInputOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
}

pub struct FollowInputOptions {
    pub reblog: Option<bool>,
}

pub struct SearchAccountInputOptions {
    pub following: Option<bool>,
    pub resolve: Option<bool>,
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
}

pub type GetBookmarksInputOptions = GetArrayWithSinceOptions;

pub type GetFavouritesInputOptions = GetArrayOptions;

pub type GetMutesInputOptions = GetArrayOptions;

pub type GetBlocksInputOptions = GetArrayOptions;

pub type GetDomainBlocksInputOptions = GetArrayOptions;

pub struct GetArrayOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub min_id: Option<String>,
}

pub struct GetArrayWithSinceOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
}

pub struct FilterInputOptions {
    pub irreversible: Option<bool>,
    pub whole_word: Option<bool>,
    pub expires_in: Option<u64>,
}

pub struct ReportInputOptions {
    pub status_ids: Option<Vec<String>>,
    pub forward: Option<bool>,
}

pub struct GetEndorsementsInputOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
}

pub struct PostStatusInputOptions {
    pub media_ids: Option<Vec<String>>,
    pub poll: Option<PollOptions>,
    pub in_reply_to_id: Option<String>,
    pub sensitive: Option<bool>,
    pub spoiler_text: Option<String>,
    pub visibility: Option<entities::status::StatusVisibility>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub language: Option<String>,
    pub quote_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PollOptions {
    pub options: Vec<String>,
    pub expires_in: Option<u64>,
    pub multiple: Option<bool>,
    pub hide_totals: Option<bool>,
}

pub struct GetStatusContextInputOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
}

pub struct UploadMediaInputOptions {
    pub description: Option<String>,
    pub focus: Option<String>,
}

pub struct UpdateMediaInputOptions {
    pub file_path: Option<String>,
    pub description: Option<String>,
    pub focus: Option<String>,
}

pub type GetScheduledStatusesInputOptions = GetArrayWithSinceOptions;

pub type GetPublicTimelineInputOptions = GetTimelineOptions;
pub type GetLocalTimelineInputOptions = GetTimelineOptions;
pub type GetTagTimelineInputOptions = GetTimelineOptionsWithLocal;
pub type GetHomeTimelineInputOptions = GetTimelineOptionsWithLocal;
pub type GetListTimelineInputOptions = GetArrayWithSinceOptions;
pub type GetConversationTimelineInputOptions = GetArrayWithSinceOptions;
pub type GetAccountsInListInputOptions = GetArrayOptions;

pub struct GetTimelineOptions {
    pub only_media: Option<bool>,
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
}

pub struct GetTimelineOptionsWithLocal {
    pub only_media: Option<bool>,
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub local: Option<bool>,
}

pub struct SaveMarkersInputOptions {
    pub home: Option<Marker>,
    pub notifications: Option<Marker>,
}

#[derive(Debug, Serialize)]
pub struct Marker {
    pub last_reading_id: String,
}

pub struct GetNotificationsInputOptions {
    pub limit: Option<u32>,
    pub max_id: Option<String>,
    pub since_id: Option<String>,
    pub min_id: Option<String>,
    pub exclude_types: Option<Vec<entities::notification::NotificationType>>,
    pub account_id: Option<String>,
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
