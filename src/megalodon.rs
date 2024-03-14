//! Megalodon modules

use core::fmt;
use std::str::FromStr;

use crate::error::{Error, Kind};
use crate::oauth::{AppData, TokenData};
use crate::response::Response;
use crate::{entities, Streaming};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tokio::{fs::File, io::AsyncRead};

/// Megalodon API interface
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
    /// Test to make sure that the application token works.
    async fn verify_app_credentials(&self) -> Result<Response<entities::Application>, Error>;

    // ======================================
    // accounts
    // ======================================
    /// Register an user account.
    async fn register_account(
        &self,
        username: String,
        email: String,
        password: String,
        agreement: String,
        locale: String,
        reason: Option<String>,
    ) -> Result<Response<entities::Token>, Error>;

    /// Test to make sure that the user token works.
    async fn verify_account_credentials(&self) -> Result<Response<entities::Account>, Error>;

    /// Update the user's display and preferences.
    async fn update_credentials(
        &self,
        options: Option<&UpdateCredentialsInputOptions>,
    ) -> Result<Response<entities::Account>, Error>;

    /// Get an account information.
    async fn get_account(&self, id: String) -> Result<Response<entities::Account>, Error>;

    /// Get statuses of the account.
    async fn get_account_statuses(
        &self,
        id: String,
        options: Option<&GetAccountStatusesInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    /// Receive notifications when this account posts a status.
    async fn subscribe_account(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    /// Stop receiving notifications when this account posts a status.
    async fn unsubscribe_account(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    /// Get accounts which follow the give account.
    async fn get_account_followers(
        &self,
        id: String,
        options: Option<&AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    /// Get accounts which the given acount is following.
    async fn get_account_following(
        &self,
        id: String,
        options: Option<&AccountFollowersInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    /// Get lists of the given account.
    async fn get_account_lists(&self, id: String) -> Result<Response<Vec<entities::List>>, Error>;

    /// Get proofs of the given account.
    async fn get_identity_proofs(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::IdentityProof>>, Error>;

    /// Follow the given account. Can also be used to update whether to show reblogs or enable notifications.
    async fn follow_account(
        &self,
        id: String,
        options: Option<&FollowAccountInputOptions>,
    ) -> Result<Response<entities::Relationship>, Error>;

    /// Unfollow the given account.
    async fn unfollow_account(&self, id: String)
        -> Result<Response<entities::Relationship>, Error>;

    /// Block the given account.
    async fn block_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    /// Unblock the given account.
    async fn unblock_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    /// Mute the given account.
    async fn mute_account(
        &self,
        id: String,
        notifications: bool,
    ) -> Result<Response<entities::Relationship>, Error>;

    /// Unmute the given account.
    async fn unmute_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    /// Add the given account to the user's featured profiles.
    async fn pin_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    /// Remove the given account from the user's featured profiles.
    async fn unpin_account(&self, id: String) -> Result<Response<entities::Relationship>, Error>;

    /// Find out whether a given account is followed, blocked, muted, etc.
    async fn get_relationships(
        &self,
        ids: Vec<String>,
    ) -> Result<Response<Vec<entities::Relationship>>, Error>;

    /// Search for matching accounts by username or display name.
    async fn search_account(
        &self,
        q: String,
        options: Option<&SearchAccountInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/bookmarks
    // ======================================
    /// Get statuses the user has bookmarked.
    async fn get_bookmarks(
        &self,
        options: Option<&GetBookmarksInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // accounts/favourites
    // ======================================
    /// Get statuses the user has favourited.
    async fn get_favourites(
        &self,
        options: Option<&GetFavouritesInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // accounts/mutes
    // ======================================
    /// Get accounts the user has muted.
    async fn get_mutes(
        &self,
        options: Option<&GetMutesInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/blocks
    // ======================================
    /// Get accounts the user has blocked.
    async fn get_blocks(
        &self,
        options: Option<&GetBlocksInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/domain_blocks
    // ======================================
    /// Get domains the user has blocked.
    async fn get_domain_blocks(
        &self,
        options: Option<&GetDomainBlocksInputOptions>,
    ) -> Result<Response<Vec<String>>, Error>;

    /// Block a domain.
    async fn block_domain(&self, domain: String) -> Result<Response<()>, Error>;

    /// Remove a domain block.
    async fn unblock_domain(&self, domain: String) -> Result<Response<()>, Error>;

    // ======================================
    // accounts/filters
    // ======================================
    /// Get all filters.
    async fn get_filters(&self) -> Result<Response<Vec<entities::Filter>>, Error>;

    /// Get a specified filter.
    async fn get_filter(&self, id: String) -> Result<Response<entities::Filter>, Error>;

    /// Create a filter.
    async fn create_filter(
        &self,
        phrase: String,
        context: Vec<entities::filter::FilterContext>,
        options: Option<&FilterInputOptions>,
    ) -> Result<Response<entities::Filter>, Error>;

    /// Update a filter.
    async fn update_filter(
        &self,
        id: String,
        phrase: String,
        context: Vec<entities::filter::FilterContext>,
        options: Option<&FilterInputOptions>,
    ) -> Result<Response<entities::Filter>, Error>;

    /// Delete a filter.
    async fn delete_filter(&self, id: String) -> Result<Response<()>, Error>;

    // ======================================
    // accounts/reports
    // ======================================
    /// Report an user.
    async fn report(
        &self,
        account_id: String,
        options: Option<&ReportInputOptions>,
    ) -> Result<Response<entities::Report>, Error>;

    // ======================================
    // accounts/follow_requests
    // ======================================
    /// Get accounts who send follow request to the user.
    async fn get_follow_requests(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<FollowRequestOutput>>, Error>;

    /// Accept the follow request.
    async fn accept_follow_request(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    /// Reject the follow request.
    async fn reject_follow_request(
        &self,
        id: String,
    ) -> Result<Response<entities::Relationship>, Error>;

    // ======================================
    // accounts/endorsements
    // ======================================
    /// Get accounts that the user is currently featuring on their profile.
    async fn get_endorsements(
        &self,
        options: Option<&GetEndorsementsInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/featured_tags
    // ======================================
    /// Get featured tags.
    async fn get_featured_tags(&self) -> Result<Response<Vec<entities::FeaturedTag>>, Error>;

    /// Create a featured tag.
    async fn create_featured_tag(
        &self,
        name: String,
    ) -> Result<Response<entities::FeaturedTag>, Error>;

    /// Delete a featured tag.
    async fn delete_featured_tag(&self, id: String) -> Result<Response<()>, Error>;

    /// Shows your 10 most-used tags, with usage history for the past week.
    async fn get_suggested_tags(&self) -> Result<Response<Vec<entities::Tag>>, Error>;

    // ======================================
    // accounts/preferences
    // ======================================
    /// Get preferences defined by the user in their account settings.
    async fn get_preferences(&self) -> Result<Response<entities::Preferences>, Error>;

    // ======================================
    // accounts/followed_tags
    // ======================================
    /// Get all followed tags.
    async fn get_followed_tags(&self) -> Result<Response<Vec<entities::Tag>>, Error>;

    // ======================================
    // accounts/suggestions
    // ======================================
    /// Get accounts the user has had past positive interactions with, but is not yet following.
    async fn get_suggestions(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // accounts/tags
    // ======================================
    /// Get a hashtag and its associated information.
    async fn get_tag(&self, id: String) -> Result<Response<entities::Tag>, Error>;

    /// Follow a hashtag. Posts containing a followed hashtag will be inserted into your home timeline.
    async fn follow_tag(&self, id: String) -> Result<Response<entities::Tag>, Error>;

    /// Unfollow a hashtag. Posts containing this hashtag will no longer be inserted into your home timeline.
    async fn unfollow_tag(&self, id: String) -> Result<Response<entities::Tag>, Error>;

    // ======================================
    // statuses
    // ======================================
    /// Post a new status.
    async fn post_status(
        &self,
        status: String,
        options: Option<&PostStatusInputOptions>,
    ) -> Result<Response<PostStatusOutput>, Error>;

    /// Get information about a status.
    async fn get_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Obtain the source properties for a status so that it can be edited.
    async fn get_status_source(
        &self,
        id: String,
    ) -> Result<Response<entities::StatusSource>, Error>;

    /// Edit a status.
    async fn edit_status(
        &self,
        id: String,
        options: &EditStatusInputOptions,
    ) -> Result<Response<entities::Status>, Error>;

    /// Delete a status of your own statuses.
    async fn delete_status(&self, id: String) -> Result<Response<()>, Error>;

    /// Get statuses above and below this status in the thread.
    async fn get_status_context(
        &self,
        id: String,
        options: Option<&GetStatusContextInputOptions>,
    ) -> Result<Response<entities::Context>, Error>;

    /// Get accounts who boosted a given status.
    async fn get_status_reblogged_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    /// Get accounts who favourited a given status.
    async fn get_status_favourited_by(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    /// Add a status to your favourites list.
    async fn favourite_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Remove a status from your favourites list.
    async fn unfavourite_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Reblog a status.
    async fn reblog_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Undo a reblog of a status.
    async fn unreblog_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Add a status to your bookmark list.
    async fn bookmark_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Remove a status from your bookmark lits.
    async fn unbookmark_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Do not receive notifications for the thread that this status is part of. Must be a thread in which you are a participant.
    async fn mute_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Start receiving notifications again for the thread that this status is part of.
    async fn unmute_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Feature one of your own public statuses at the top of your profile.
    async fn pin_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    /// Unfeature a status from the top of your profile.
    async fn unpin_status(&self, id: String) -> Result<Response<entities::Status>, Error>;

    // ======================================
    // statuses/media
    // ======================================
    /// Creates an attachment to be used with a new status.
    async fn upload_media(
        &self,
        file_path: String,
        options: Option<&UploadMediaInputOptions>,
    ) -> Result<Response<entities::UploadMedia>, Error> {
        let file = File::open(file_path.clone()).await?;
        self.upload_media_reader(Box::new(file), options).await
    }

    async fn upload_media_reader(
        &self,
        reader: Box<dyn AsyncRead + Sync + Send + Unpin>,
        options: Option<&UploadMediaInputOptions>,
    ) -> Result<Response<entities::UploadMedia>, Error>;

    /// Get an Attachment.
    async fn get_media(&self, id: String) -> Result<Response<entities::Attachment>, Error>;

    /// Update an Attachment, before it is attached to a status and posted.
    async fn update_media(
        &self,
        id: String,
        options: Option<&UpdateMediaInputOptions>,
    ) -> Result<Response<entities::Attachment>, Error>;

    // ======================================
    // statuses/polls
    // ======================================
    /// Get a poll information.
    async fn get_poll(&self, id: String) -> Result<Response<entities::Poll>, Error>;

    /// Vote a poll.
    async fn vote_poll(
        &self,
        id: String,
        choices: Vec<u32>,
        status_id: Option<String>,
    ) -> Result<Response<entities::Poll>, Error>;

    // ======================================
    // statuses/scheduled_statuses
    // ======================================
    /// Get statuses which scheduled to publish later.
    async fn get_scheduled_statuses(
        &self,
        options: Option<&GetScheduledStatusesInputOptions>,
    ) -> Result<Response<Vec<entities::ScheduledStatus>>, Error>;

    /// Get a scheduled status.
    async fn get_scheduled_status(
        &self,
        id: String,
    ) -> Result<Response<entities::ScheduledStatus>, Error>;

    /// Schedule a status to publish later.
    async fn schedule_status(
        &self,
        id: String,
        scheduled_at: Option<DateTime<Utc>>,
    ) -> Result<Response<entities::ScheduledStatus>, Error>;

    /// Remove the schdule to publish.
    async fn cancel_scheduled_status(&self, id: String) -> Result<Response<()>, Error>;

    // ======================================
    // timeilnes
    // ======================================
    /// Get statuses of public timeline.
    async fn get_public_timeline(
        &self,
        options: Option<&GetPublicTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    /// Get statuses of local timeline.
    async fn get_local_timeline(
        &self,
        options: Option<&GetLocalTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    /// Get statuses of tag timeline.
    async fn get_tag_timeline(
        &self,
        hashtag: String,
        options: Option<&GetTagTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    /// Get statuses of home timeline.
    async fn get_home_timeline(
        &self,
        options: Option<&GetHomeTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    /// Get status of list timeline.
    async fn get_list_timeline(
        &self,
        list_id: String,
        options: Option<&GetListTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Status>>, Error>;

    // ======================================
    // timeilnes/conversations
    // ======================================
    /// Get statuses of conversation timeline.
    async fn get_conversation_timeline(
        &self,
        options: Option<&GetConversationTimelineInputOptions>,
    ) -> Result<Response<Vec<entities::Conversation>>, Error>;

    /// Delete a conversation.
    async fn delete_conversation(&self, id: String) -> Result<Response<()>, Error>;

    /// Mark to read the conversation.
    async fn read_conversation(
        &self,
        id: String,
    ) -> Result<Response<entities::Conversation>, Error>;

    // ======================================
    // timeilnes/lists
    // ======================================
    /// Get list timelines which you created.
    async fn get_lists(&self) -> Result<Response<Vec<entities::List>>, Error>;

    /// Get a list timeline.
    async fn get_list(&self, id: String) -> Result<Response<entities::List>, Error>;

    /// Create a new list timeline.
    async fn create_list(&self, title: String) -> Result<Response<entities::List>, Error>;

    /// Update the list timeline.
    async fn update_list(
        &self,
        id: String,
        title: String,
    ) -> Result<Response<entities::List>, Error>;

    /// Delete the list timeline.
    async fn delete_list(&self, id: String) -> Result<Response<()>, Error>;

    /// Get accounts which registered to the list.
    async fn get_accounts_in_list(
        &self,
        id: String,
        options: Option<&GetAccountsInListInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    /// Register an account to the list.
    async fn add_accounts_to_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<entities::List>, Error>;

    /// Remove the account from the list.
    async fn delete_accounts_from_list(
        &self,
        id: String,
        account_ids: Vec<String>,
    ) -> Result<Response<()>, Error>;

    // ======================================
    // timeilnes/markers
    // ======================================
    /// Get your position in timelines.
    async fn get_markers(&self, timeline: Vec<String>)
        -> Result<Response<entities::Marker>, Error>;

    /// Save your position in timelines.
    async fn save_markers(
        &self,
        options: Option<&SaveMarkersInputOptions>,
    ) -> Result<Response<entities::Marker>, Error>;

    // ======================================
    // notifications
    // ======================================
    /// Notifications concerning the user.
    async fn get_notifications(
        &self,
        options: Option<&GetNotificationsInputOptions>,
    ) -> Result<Response<Vec<entities::Notification>>, Error>;

    /// Get a notification information.
    async fn get_notification(&self, id: String)
        -> Result<Response<entities::Notification>, Error>;

    /// Clear all notifications from the server.
    async fn dismiss_notifications(&self) -> Result<Response<()>, Error>;

    /// Clear a notification from the server.
    async fn dismiss_notification(&self, id: String) -> Result<Response<()>, Error>;

    // ======================================
    // notifications/push
    // ======================================
    /// Add a Web Push API subscription to receive notifications.
    async fn subscribe_push_notification(
        &self,
        subscription: &SubscribePushNotificationInputSubscription,
        data: Option<&SubscribePushNotificationInputData>,
    ) -> Result<Response<entities::PushSubscription>, Error>;

    /// Get the PushSubscription currently associated with this access token.
    async fn get_push_subscription(&self) -> Result<Response<entities::PushSubscription>, Error>;

    /// Updates the current push subscription.
    async fn update_push_subscription(
        &self,
        data: Option<&SubscribePushNotificationInputData>,
    ) -> Result<Response<entities::PushSubscription>, Error>;

    /// Removes the current Web Push API subscription.
    async fn delete_push_subscription(&self) -> Result<Response<()>, Error>;

    // ======================================
    // search
    // ======================================
    /// Search for content in accounts, statuses and hashtags.
    async fn search(
        &self,
        q: String,
        options: Option<&SearchInputOptions>,
    ) -> Result<Response<entities::Results>, Error>;

    // ======================================
    // instance
    // ======================================
    /// Get information about the server.
    async fn get_instance(&self) -> Result<Response<entities::Instance>, Error>;

    /// Get domains that this instance is aware of.
    async fn get_instance_peers(&self) -> Result<Response<Vec<String>>, Error>;

    /// Get instance activity over the last 3 months, binned weekly.
    async fn get_instance_activity(&self) -> Result<Response<Vec<entities::Activity>>, Error>;

    // ======================================
    // instance/trends
    // ======================================
    /// Tags that are being used more frequently within the past week.
    async fn get_instance_trends(
        &self,
        limit: Option<u32>,
    ) -> Result<Response<Vec<entities::Tag>>, Error>;

    // ======================================
    // instance/directory
    // ======================================
    /// List accounts visible in the directory.
    async fn get_instance_directory(
        &self,
        options: Option<&GetInstanceDirectoryInputOptions>,
    ) -> Result<Response<Vec<entities::Account>>, Error>;

    // ======================================
    // instance/custom_emojis
    // ======================================
    /// Returns custom emojis that are available on the server.
    async fn get_instance_custom_emojis(&self) -> Result<Response<Vec<entities::Emoji>>, Error>;

    // ======================================
    // instance/announcements
    // ======================================
    /// Get all currently active announcements set by admins.
    async fn get_instance_announcements(
        &self,
    ) -> Result<Response<Vec<entities::Announcement>>, Error>;

    /// Dismiss an announcement
    async fn dismiss_instance_announcement(&self, id: String) -> Result<Response<()>, Error>;

    /// Add a reaction to an announcement.
    async fn add_reaction_to_announcement(
        &self,
        id: String,
        name: String,
    ) -> Result<Response<()>, Error>;

    /// Remove a reaction from an announcement.
    async fn remove_reaction_from_announcement(
        &self,
        id: String,
        name: String,
    ) -> Result<Response<()>, Error>;

    // ======================================
    // Emoji reactions
    // ======================================
    /// Add an emoji reaction to the status.
    async fn create_emoji_reaction(
        &self,
        id: String,
        emoji: String,
    ) -> Result<Response<entities::Status>, Error>;

    /// Remove the emoji reaction from the status.
    async fn delete_emoji_reaction(
        &self,
        id: String,
        emoji: String,
    ) -> Result<Response<entities::Status>, Error>;

    /// Get emoji reactions of the status.
    async fn get_emoji_reactions(
        &self,
        id: String,
    ) -> Result<Response<Vec<entities::Reaction>>, Error>;

    /// Get emoji reaction of the status.
    async fn get_emoji_reaction(
        &self,
        id: String,
        emoji: String,
    ) -> Result<Response<entities::Reaction>, Error>;

    // ======================================
    // Streaming
    // ======================================
    /// Get the base URL for streaming endpoints
    async fn streaming_url(&self) -> String;

    /// Get user streaming object.
    async fn user_streaming(&self) -> Box<dyn Streaming + Send + Sync>;

    /// Get public streaming object.
    async fn public_streaming(&self) -> Box<dyn Streaming + Send + Sync>;

    /// Get local streaming object.
    async fn local_streaming(&self) -> Box<dyn Streaming + Send + Sync>;

    /// Get direct streaming object.
    async fn direct_streaming(&self) -> Box<dyn Streaming + Send + Sync>;

    /// Get tag streaming object.
    async fn tag_streaming(&self, tag: String) -> Box<dyn Streaming + Send + Sync>;

    /// Get list streaming object.
    async fn list_streaming(&self, list_id: String) -> Box<dyn Streaming + Send + Sync>;
}

/// Input options for [`Megalodon::register_app`] and [`Megalodon::create_app`].
#[derive(Debug, Clone, Default)]
pub struct AppInputOptions {
    /// List of requested OAuth scopes.
    pub scopes: Option<Vec<String>>,
    /// Set a URI to redirect the user to.
    pub redirect_uris: Option<String>,
    /// URL of the application.
    pub website: Option<String>,
}

/// Input options for [`Megalodon::update_credentials`].
#[derive(Debug, Clone, Default)]
pub struct UpdateCredentialsInputOptions {
    /// Whether the account should be shown in the profile directory.
    pub discoverable: Option<bool>,
    /// Whether the account has a bot flag.
    pub bot: Option<bool>,
    /// The display name to use for the profile.
    pub display_name: Option<String>,
    /// The account bio.
    pub note: Option<String>,
    /// Avatar image encoded using multipart/form-data.
    pub avatar: Option<String>,
    /// Header image encoded using multipart/form-data.
    pub header: Option<String>,
    /// Whether manual approval of follow requests is required.
    pub locked: Option<bool>,
    /// Credential source options.
    pub source: Option<CredentialsSource>,
    /// Profile metadata array.
    pub fields_attributes: Option<Vec<CredentialsFieldAttribute>>,
}

/// Credential source options.
#[derive(Debug, Serialize, Clone, Default)]
pub struct CredentialsSource {
    /// Default post privacy for authored statuses.
    pub privacy: Option<String>,
    /// Whether to mark authored statuses as sensitive by default.
    pub sensitive: Option<bool>,
    /// Default language to use for authored statuses. (ISO 6391)
    pub language: Option<String>,
}

/// Profile metadata array.
#[derive(Debug, Serialize, Clone)]
pub struct CredentialsFieldAttribute {
    /// Name of the metadata.
    pub name: String,
    /// Value of the metadata.
    pub value: String,
}

/// Input options for [`Megalodon::get_account_statuses`].
#[derive(Debug, Clone, Default)]
pub struct GetAccountStatusesInputOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
    /// Include pinned statuses.
    pub pinned: Option<bool>,
    /// Exclude statuses of replies.
    pub exclude_replies: Option<bool>,
    /// Exclude statuses of reblogged.
    pub exclude_reblogs: Option<bool>,
    /// Show only statuses with metia attached.
    pub only_media: Option<bool>,
}

/// Input options for [`Megalodon::get_account_followers`] and [`Megalodon::get_account_following`].
#[derive(Debug, Clone, Default)]
pub struct AccountFollowersInputOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
}

/// Input options for [`Megalodon::follow_account`].
#[derive(Debug, Clone, Default)]
pub struct FollowAccountInputOptions {
    /// Receive this account's reblogs in home timeline.
    pub reblog: Option<bool>,
    /// Receive notifications when this account posts a status.
    pub notify: Option<bool>,
}

/// Input options for [`Megalodon::search_account`].
#[derive(Debug, Clone, Default)]
pub struct SearchAccountInputOptions {
    /// Only who the user is following.
    pub following: Option<bool>,
    /// Attempt WebFinger lookup.
    pub resolve: Option<bool>,
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
}

/// Input options for [`Megalodon::get_bookmarks`].
pub type GetBookmarksInputOptions = GetArrayWithSinceOptions;

/// Input options for [`Megalodon::get_favourites`].
pub type GetFavouritesInputOptions = GetArrayOptions;

/// Input options for [`Megalodon::get_mutes`].
pub type GetMutesInputOptions = GetArrayOptions;

/// Input options for [`Megalodon::get_blocks`].
pub type GetBlocksInputOptions = GetArrayOptions;

/// Input options for [`Megalodon::get_domain_blocks`].
pub type GetDomainBlocksInputOptions = GetArrayOptions;

/// Get array options.
#[derive(Debug, Clone, Default)]
pub struct GetArrayOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<String>,
}

/// Get array options with since.
#[derive(Debug, Clone, Default)]
pub struct GetArrayWithSinceOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<String>,
}

/// Input options for [`Megalodon::create_filter`] and [`Megalodon::update_filter`].
#[derive(Debug, Clone, Default)]
pub struct FilterInputOptions {
    /// Should the server irreversibly drop matching entities from home and notifications.
    pub irreversible: Option<bool>,
    /// Consider word boundaries.
    pub whole_word: Option<bool>,
    /// Number of seconds from now the filter should expire.
    pub expires_in: Option<u64>,
}

/// Input options for [`Megalodon::report`].
#[derive(Debug, Clone, Default)]
pub struct ReportInputOptions {
    /// Array of Statuses to attach to the report.
    pub status_ids: Option<Vec<String>>,
    /// The reason for the report. Default maximum of 1000 characters.
    pub comment: Option<String>,
    /// If the account is remote, should the report be forwarded to the remote admin.
    pub forward: Option<bool>,
    /// Specify if the report is due to spam, violation of enumerated instance rules, or some other reason. Defaults to other. Will be set to violation if rule_ids[] is provided (regardless of any category value you provide).
    pub category: Option<entities::report::Category>,
    /// For violation category reports, specify the ID of the exact rules broken. Rules and their IDs are available via GET /api/v1/instance/rules and GET /api/v1/instance.
    pub rule_ids: Option<Vec<u64>>,
}

/// Input options for [`Megalodon::get_endorsements`].
#[derive(Debug, Clone, Default)]
pub struct GetEndorsementsInputOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
}

/// Input options for [`Megalodon::post_status`].
#[derive(Debug, Clone, Default)]
pub struct PostStatusInputOptions {
    /// Array of Attachment ids to be attached as media.
    pub media_ids: Option<Vec<String>>,
    /// Poll options.
    pub poll: Option<PollOptions>,
    /// ID of the status being replied to, if status is a reply.
    pub in_reply_to_id: Option<String>,
    /// Mark status and attached media as sensitive.
    pub sensitive: Option<bool>,
    /// Text to be shown as a warning or subject before the actual content.
    pub spoiler_text: Option<String>,
    /// Visibility of the posted status.
    pub visibility: Option<entities::status::StatusVisibility>,
    /// ISO 8601 Datetime at which to schedule a status.
    pub scheduled_at: Option<DateTime<Utc>>,
    /// ISO 639 language code for this status.
    pub language: Option<String>,
    /// ID of the status being quoted to.
    pub quote_id: Option<String>,
}

/// Input options for [`Megalodon::edit_status`].
#[derive(Debug, Clone, Default)]
pub struct EditStatusInputOptions {
    /// The plain text content of the status.
    pub status: Option<String>,
    /// Text to be shown as a warning or subject before the actual content.
    pub spoiler_text: Option<String>,
    /// Mark status and attached media as sensitive.
    pub sensitive: Option<bool>,
    /// ISO 639 language code for this status.
    pub language: Option<String>,
    /// Array of Attachment ids to be attached as media.
    pub media_ids: Option<Vec<String>>,
    /// Poll options.
    pub poll: Option<PollOptions>,
}

/// Poll options.
#[derive(Debug, Serialize, Clone, Default)]
pub struct PollOptions {
    /// Array of possible answers.
    pub options: Vec<String>,
    /// Duration the poll should be open, in seconds.
    pub expires_in: Option<u64>,
    /// Allow multiple choices.
    pub multiple: Option<bool>,
    /// Hide vote counts until the poll ends.
    pub hide_totals: Option<bool>,
}

/// Input options for [`Megalodon::get_status_context`].
#[derive(Debug, Clone, Default)]
pub struct GetStatusContextInputOptions {
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// REturn results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
}

/// Input options for [`Megalodon::upload_media`].
#[derive(Debug, Clone, Default)]
pub struct UploadMediaInputOptions {
    /// A plain-text description of the file.
    pub description: Option<String>,
    /// Two floating points (x,y), comma-delimited, ranging from -1.0 to 1.0.
    pub focus: Option<String>,
}

/// Input options for [`Megalodon::update_media`].
#[derive(Debug, Clone, Default)]
pub struct UpdateMediaInputOptions {
    /// The file path to be attached.
    pub file_path: Option<String>,
    /// A plain-text description of the file.
    pub description: Option<String>,
    /// Two floating points (x,y), comma-delimited, ranging from -1.0 to 1.0.
    pub focus: Option<String>,
}

/// Input options for [`Megalodon::get_scheduled_statuses`].
pub type GetScheduledStatusesInputOptions = GetArrayWithSinceOptions;

/// Input options for [`Megalodon::get_public_timeline`].
pub type GetPublicTimelineInputOptions = GetTimelineOptions;
/// Input options for [`Megalodon::get_local_timeline`].
pub type GetLocalTimelineInputOptions = GetTimelineOptions;
/// Input options for [`Megalodon::get_tag_timeline`].
pub type GetTagTimelineInputOptions = GetTimelineOptionsWithLocal;
/// Input options for [`Megalodon::get_home_timeline`].
pub type GetHomeTimelineInputOptions = GetTimelineOptionsWithLocal;
/// Input options for [`Megalodon::get_list_timeline`].
pub type GetListTimelineInputOptions = GetArrayWithSinceOptions;
/// Input options for [`Megalodon::get_conversation_timeline`].
pub type GetConversationTimelineInputOptions = GetArrayWithSinceOptions;

/// Input ptions for [`Megalodon::get_accounts_in_list`].
pub type GetAccountsInListInputOptions = GetArrayOptions;

/// Timeline options.
#[derive(Debug, Clone, Default)]
pub struct GetTimelineOptions {
    /// Show only statuses with media attached.
    pub only_media: Option<bool>,
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<String>,
}

/// Timeline options with local flag.
#[derive(Debug, Clone, Default)]
pub struct GetTimelineOptionsWithLocal {
    /// Show only statuses with media attached.
    pub only_media: Option<bool>,
    /// Maximum number of results to return.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<String>,
    /// Show only local statuses.
    pub local: Option<bool>,
}

/// Input options for [`Megalodon::save_markers`].
#[derive(Debug, Clone, Default)]
pub struct SaveMarkersInputOptions {
    /// ID of the last status read in the home timeline.
    pub home: Option<Marker>,
    /// ID of the last notification read.
    pub notifications: Option<Marker>,
}

/// Marker of timelines.
#[derive(Debug, Serialize, Clone)]
pub struct Marker {
    /// ID of the last status read
    pub last_reading_id: String,
}

/// Input options for [`Megalodon::get_notifications`].
#[derive(Debug, Clone, Default)]
pub struct GetNotificationsInputOptions {
    /// Maximum number of results to return. Default 20.
    pub limit: Option<u32>,
    /// Return results older than this ID.
    pub max_id: Option<String>,
    /// Return results newer than this ID.
    pub since_id: Option<String>,
    /// Return results immediately newer than this ID.
    pub min_id: Option<String>,
    /// Array of types to exclude.
    pub exclude_types: Option<Vec<entities::notification::NotificationType>>,
    /// Return only notifications received from this account.
    pub account_id: Option<String>,
}

/// Subscription input options for [`Megalodon::subscribe_push_notification`].
#[derive(Debug, Serialize, Clone)]
pub struct SubscribePushNotificationInputSubscription {
    /// Endpoint URL that is called when a notification event occurs.
    pub endpoint: String,
    /// Keys for subscription.
    pub keys: SubscriptionKeys,
}

/// Keys for subscription.
#[derive(Debug, Serialize, Clone)]
pub struct SubscriptionKeys {
    /// User agent public key. Base64 encoded string of public key of ECDH key using prime256v1 curve.1
    pub p256h: String,
    /// Auth secret. Base64 encoded string of 16 bytes of random data.
    pub auth: String,
}

/// Input data of [`Megalodon::subscribe_push_notification`].
#[derive(Debug, Serialize, Clone, Default)]
pub struct SubscribePushNotificationInputData {
    /// Alert options.
    pub alerts: Option<DataAlerts>,
}

/// Alert options.
#[derive(Debug, Serialize, Clone, Default)]
pub struct DataAlerts {
    /// Receive follow notification.
    pub follow: Option<bool>,
    /// Receive favourite notification.
    pub favourite: Option<bool>,
    /// Receive reblog notification.
    pub reblog: Option<bool>,
    /// Receive mention notification.
    pub mention: Option<bool>,
    /// Receive poll notification.
    pub poll: Option<bool>,
}

/// Type of search results.
#[derive(Debug, Clone)]
pub enum SearchType {
    /// Search accounts.
    Accounts,
    /// Search hash tags.
    Hashtags,
    /// Search statuses.
    Statuses,
}

impl fmt::Display for SearchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SearchType::Accounts => write!(f, "accounts"),
            SearchType::Hashtags => write!(f, "hashtags"),
            SearchType::Statuses => write!(f, "statuses"),
        }
    }
}

impl FromStr for SearchType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accounts" => Ok(SearchType::Accounts),
            "hashtags" => Ok(SearchType::Hashtags),
            "statuses" => Ok(SearchType::Statuses),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

/// Input options for [`Megalodon::search`].
#[derive(Debug, Clone, Default)]
pub struct SearchInputOptions {
    /// Specify whether to search for only accounts, hashtags, statuses.
    pub r#type: Option<SearchType>,
    /// Maximum number of results. Default 20, max 40.
    pub limit: Option<u32>,
    /// Return results oder than this id.
    pub max_id: Option<String>,
    /// Return results newer than this id.
    pub min_id: Option<String>,
    /// Attempt WebFinger lookup. Defaults to false.
    pub resolve: Option<bool>,
    /// Offset in search results. Used for pagination. Defaults to 0.
    pub offset: Option<u64>,
    /// Only include accounts that the user is following. Defaults to false.
    pub following: Option<bool>,
    /// If provided, statuses returned will be authored only by this account
    pub account_id: Option<String>,
    /// Filter out unreviewed tags? Defaults to false. Use true when trying to find trending tags.
    pub exclude_unreviewed: Option<bool>,
}

/// Input options for [`Megalodon::get_instance_directory`].
#[derive(Debug, Clone, Default)]
pub struct GetInstanceDirectoryInputOptions {
    /// How many accounts to load. Default 40.
    pub limit: Option<u32>,
    /// How many accounts to skip before returning results. Default 0.
    pub offset: Option<u64>,
    /// Sort options of accounts.
    pub order: Option<Order>,
    /// Only returns local accounts.
    pub local: Option<bool>,
}

/// Sort option of accounts.
#[derive(Debug, Clone)]
pub enum Order {
    /// Active to sort by most recently posted statuses.
    Active,
    /// New to sort by most recently created profiles.
    New,
}

impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Order::Active => write!(f, "active"),
            Order::New => write!(f, "new"),
        }
    }
}

impl FromStr for Order {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(Order::Active),
            "new" => Ok(Order::New),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

/// FollowRequest output object. It is FollowRequest object only if Friendica, otherwise it is Account object.
#[derive(Debug, Clone)]
pub enum FollowRequestOutput {
    /// Account object for Mastodon and Pleroma.
    Account(entities::Account),
    /// FollowRequest object for Friendica.
    FollowRequest(entities::FollowRequest),
}

/// PostStatus output object. When the scheduled_at is specified, it returns ScheduledStatus object. Otherwise, it returns Status object.
#[derive(Debug, Clone)]
pub enum PostStatusOutput {
    /// Status object for normal post results.
    Status(entities::Status),
    /// ScheduleStatus object for scheduled_at is specified.
    ScheduledStatus(entities::ScheduledStatus),
}
