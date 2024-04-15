use super::{Emoji, Note};
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::Field;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UserDetail {
    id: String,
    name: Option<String>,
    username: String,
    host: Option<String>,
    avatar_url: Option<String>,
    // avatar_blurhash: Option<String>,
    avatar_color: Option<String>,
    // is_admin: Option<bool>,
    // is_moderator: Option<bool>,
    is_bot: Option<bool>,
    is_locked: bool,
    is_indexable: Option<bool>,
    // is_cat: Option<bool>,
    // speak_as_cat: Option<bool>,
    emojis: Vec<Emoji>,
    // online_status: Option<String>,
    // url: Option<String>,
    // uri: Option<String>,
    // moved_to_uri: Option<String>,
    // also_known_as: Option<String>,
    created_at: DateTime<Utc>,
    // updated_at: Option<DateTime<Utc>>,
    // last_fetched_at: Option<DateTime<Utc>>,
    banner_url: Option<String>,
    // banner_blurhash: Option<String>,
    banner_color: Option<String>,
    is_silenced: bool,
    is_suspended: bool,
    description: Option<String>,
    // location: Option<String>,
    // birthday: Option<String>,
    pub lang: Option<String>,
    fields: Vec<Field>,
    followers_count: i32,
    following_count: u32,
    notes_count: u32,
    // pinned_note_ids: Vec<String>,
    pub pinned_notes: Vec<Note>,
    // pinned_page_id: Option<String>,
    // pinned_page: Option<Page>,
    // public_reactions: bool,
    // ff_visibility: String,
    // two_factor_enabled: bool,
    // use_password_less_login: bool,
    // security_keys: bool,
    // is_following: Option<bool>,
    // is_followed: Option<bool>,
    // has_pending_follow_request_from_you: Option<bool>,
    // has_pending_follow_request_to_you: Option<bool>,
    // is_blocking: Option<bool>,
    // is_blocked: Option<bool>,
    // is_muted: Option<bool>,
    // is_renote_muted: Option<bool>,
    // avatar_id: Option<String>,
    // banner_id: Option<String>,
    // inject_featured_note: Option<bool>,
    // receive_announcement_email: Option<bool>,
    pub always_mark_nsfw: Option<bool>,
    // auto_sensitive: Option<bool>,
    // careful_bot: Option<bool>,
    // auto_accept_followed: Option<bool>,
    // no_crawle: Option<bool>,
    // prevent_ai_learning: Option<bool>,
    // is_explorable: bool,
    // is_deleted: bool,
    // hide_online_status: bool,
    // has_unread_specified_notes: bool,
    // has_unread_mentions: bool,
    // has_unread_announcement: bool,
    // has_unread_antenna: bool,
    // has_unread_channel: bool,
    // has_unread_messaging_message: bool,
    // has_unread_notification: bool,
    // has_pending_received_follow_request: bool,
    // muted_words: Vec<String>,
    // muted_instances: Option<Vec<String>>,
    // muting_notification_types: Option<Vec<String>>,
    // email_notification_types: Option<Vec<String>>,
}

impl From<UserDetail> for MegalodonEntities::Account {
    fn from(val: UserDetail) -> Self {
        let mut acct = val.username.clone();
        if let Some(host) = val.host {
            acct = format!("{}@{}", val.username, host);
        }
        let mut display_name = "".to_string();
        if let Some(name) = val.name {
            display_name = name;
        }
        let mut note = "".to_string();
        if let Some(description) = val.description {
            note = description;
        }
        let mut avatar = "".to_string();
        if let Some(avatar_url) = val.avatar_url {
            avatar = avatar_url;
        }
        let mut avatar_static = "".to_string();
        if let Some(avatar_color) = val.avatar_color {
            avatar_static = avatar_color;
        }
        let mut header = "".to_string();
        if let Some(banner_url) = val.banner_url {
            header = banner_url;
        }
        let mut header_static = "".to_string();
        if let Some(banner_color) = val.banner_color {
            header_static = banner_color;
        }
        let mut bot = false;
        if let Some(is_bot) = val.is_bot {
            bot = is_bot;
        }

        let source = MegalodonEntities::Source {
            privacy: None,
            sensitive: val.always_mark_nsfw,
            language: val.lang,
            note: note.clone(),
            fields: None,
        };

        MegalodonEntities::Account {
            id: val.id,
            username: val.username,
            acct: acct.clone(),
            display_name,
            locked: val.is_locked,
            discoverable: None,
            group: None,
            noindex: val.is_indexable,
            suspended: Some(val.is_suspended),
            limited: Some(val.is_silenced),
            created_at: val.created_at,
            followers_count: val.followers_count,
            following_count: val.following_count,
            statuses_count: val.notes_count,
            note,
            url: acct,
            avatar,
            avatar_static,
            header,
            header_static,
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            moved: None,
            fields: val.fields.into_iter().map(|j| j.into()).collect(),
            bot,
            source: Some(source),
            role: None,
            mute_expires_at: None,
        }
    }
}
