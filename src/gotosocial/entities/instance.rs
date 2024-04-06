use super::{Account, Stats, URLs};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Instance {
    pub account_domain: String,
    pub approval_required: bool,
    pub configuration: InstanceConfig,
    pub contact_account: Option<Account>,
    pub description: String,
    pub email: String,
    pub invites_enabled: bool,
    pub languages: Vec<String>,
    pub max_toot_chars: u32,
    pub registrations: bool,
    pub stats: Stats,
    pub thumbnail: Option<String>,
    pub title: String,
    pub uri: String,
    pub urls: URLs,
    pub version: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstanceConfig {
    pub statuses: Statuses,
    pub media_attachments: MediaAttachments,
    pub polls: Polls,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Statuses {
    pub max_characters: u32,
    pub max_media_attachments: u32,
    pub characters_reserved_per_url: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MediaAttachments {
    pub supported_mime_types: Vec<String>,
    pub image_size_limit: u32,
    pub image_matrix_limit: u32,
    pub video_size_limit: u32,
    pub video_frame_rate_limit: u32,
    pub video_matrix_limit: u32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Polls {
    pub max_options: u32,
    pub max_characters_per_option: u32,
    pub min_expiration: u32,
    pub max_expiration: u32,
}

impl From<Instance> for MegalodonEntities::Instance {
    fn from(item: Instance) -> Self {
        MegalodonEntities::Instance {
            uri: item.uri,
            title: item.title,
            description: item.description,
            email: item.email,
            version: item.version,
            thumbnail: item.thumbnail,
            urls: Some(item.urls.into()),
            stats: item.stats.into(),
            languages: item.languages,
            registrations: item.registrations,
            approval_required: item.approval_required,
            invites_enabled: Some(item.invites_enabled),
            contact_account: item.contact_account.map(|i| i.into()),
            configuration: item.configuration.into(),
            rules: None,
        }
    }
}

impl From<InstanceConfig> for MegalodonEntities::instance::InstanceConfig {
    fn from(item: InstanceConfig) -> Self {
        MegalodonEntities::instance::InstanceConfig {
            statuses: item.statuses.into(),
            polls: Some(item.polls.into()),
        }
    }
}

impl From<Statuses> for MegalodonEntities::instance::Statuses {
    fn from(item: Statuses) -> Self {
        MegalodonEntities::instance::Statuses {
            max_characters: item.max_characters,
            max_media_attachments: Some(item.max_media_attachments),
            characters_reserved_per_url: Some(item.characters_reserved_per_url),
        }
    }
}

impl From<Polls> for MegalodonEntities::instance::Polls {
    fn from(item: Polls) -> Self {
        MegalodonEntities::instance::Polls {
            max_options: item.max_options,
            max_characters_per_option: item.max_characters_per_option,
            min_expiration: item.min_expiration,
            max_expiration: item.max_expiration,
        }
    }
}
