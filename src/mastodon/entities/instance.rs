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

impl Into<MegalodonEntities::Instance> for Instance {
    fn into(self) -> MegalodonEntities::Instance {
        MegalodonEntities::Instance {
            uri: self.uri,
            title: self.title,
            description: self.description,
            email: self.email,
            version: self.version,
            thumbnail: self.thumbnail,
            urls: self.urls.into(),
            stats: self.stats.into(),
            languages: self.languages,
            registrations: self.registrations,
            approval_required: self.approval_required,
            invites_enabled: Some(self.invites_enabled),
            contact_account: Some(self.contact_account.into()),
            configuration: self.configuration.into(),
            rules: Some(self.rules.into_iter().map(|r| r.into()).collect()),
        }
    }
}

impl Into<MegalodonEntities::instance::InstanceConfig> for InstanceConfig {
    fn into(self) -> MegalodonEntities::instance::InstanceConfig {
        MegalodonEntities::instance::InstanceConfig {
            statuses: self.statuses.into(),
            polls: self.polls.into(),
        }
    }
}

impl Into<MegalodonEntities::instance::Statuses> for Statuses {
    fn into(self) -> MegalodonEntities::instance::Statuses {
        MegalodonEntities::instance::Statuses {
            max_characters: self.max_characters,
            max_media_attachments: Some(self.max_media_attachments),
            characters_reserved_per_url: Some(self.characters_reserved_per_url),
        }
    }
}

impl Into<MegalodonEntities::instance::Polls> for Polls {
    fn into(self) -> MegalodonEntities::instance::Polls {
        MegalodonEntities::instance::Polls {
            max_options: self.max_options,
            max_characters_per_option: self.max_characters_per_option,
            min_expiration: self.min_expiration,
            max_expiration: self.max_expiration,
        }
    }
}

impl Into<MegalodonEntities::instance::InstanceRule> for InstanceRule {
    fn into(self) -> MegalodonEntities::instance::InstanceRule {
        MegalodonEntities::instance::InstanceRule {
            id: self.id,
            text: self.text,
        }
    }
}
