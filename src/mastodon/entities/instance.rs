use super::{Account, Stats, URLs};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Instance {
    pub uri: String,
    pub title: String,
    pub description: String,
    pub email: String,
    pub version: String,
    pub thumbnail: Option<String>,
    pub urls: URLs,
    pub stats: Stats,
    pub languages: Vec<String>,
    pub registrations: bool,
    pub approval_required: bool,
    pub invites_enabled: bool,
    pub max_toot_chars: Option<u32>,
    pub configuration: InstanceConfig,
    pub contact_account: Account,
    pub rules: Vec<InstanceRule>,
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

#[derive(Deserialize, Debug, Clone)]
pub struct InstanceRule {
    pub id: String,
    pub text: String,
}

impl From<Instance> for MegalodonEntities::Instance {
    fn from(val: Instance) -> Self {
        MegalodonEntities::Instance {
            uri: val.uri,
            title: val.title,
            description: val.description,
            email: val.email,
            version: val.version,
            thumbnail: val.thumbnail,
            urls: Some(val.urls.into()),
            stats: val.stats.into(),
            languages: val.languages,
            registrations: val.registrations,
            approval_required: val.approval_required,
            invites_enabled: Some(val.invites_enabled),
            contact_account: Some(val.contact_account.into()),
            configuration: val.configuration.into(),
            rules: Some(val.rules.into_iter().map(|r| r.into()).collect()),
        }
    }
}

impl From<InstanceConfig> for MegalodonEntities::instance::InstanceConfig {
    fn from(val: InstanceConfig) -> Self {
        MegalodonEntities::instance::InstanceConfig {
            statuses: val.statuses.into(),
            polls: Some(val.polls.into()),
        }
    }
}

impl From<Statuses> for MegalodonEntities::instance::Statuses {
    fn from(val: Statuses) -> Self {
        MegalodonEntities::instance::Statuses {
            max_characters: val.max_characters,
            max_media_attachments: Some(val.max_media_attachments),
            characters_reserved_per_url: Some(val.characters_reserved_per_url),
        }
    }
}

impl From<Polls> for MegalodonEntities::instance::Polls {
    fn from(val: Polls) -> Self {
        MegalodonEntities::instance::Polls {
            max_options: val.max_options,
            max_characters_per_option: val.max_characters_per_option,
            min_expiration: val.min_expiration,
            max_expiration: val.max_expiration,
        }
    }
}

impl From<InstanceRule> for MegalodonEntities::instance::InstanceRule {
    fn from(val: InstanceRule) -> Self {
        MegalodonEntities::instance::InstanceRule {
            id: val.id,
            text: val.text,
        }
    }
}
