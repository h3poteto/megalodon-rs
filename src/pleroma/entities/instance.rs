use super::{Stats, URLs};
use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

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
    pub max_toot_chars: u32,
    pub max_media_attachments: Option<u32>,
    pub pleroma: PleromaConfig,
    pub poll_limits: PollLimits,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PleromaConfig {
    pub metadata: PleromaMetadata,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PleromaMetadata {
    pub account_activation_required: bool,
    pub birthday_min_age: Option<u32>,
    pub birthday_required: Option<bool>,
    pub features: Vec<String>,
    pub federation: Federation,
    pub fields_limits: FieldsLimits,
    pub post_formats: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Federation {
    pub enabled: bool,
    pub exclusions: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FieldsLimits {
    pub max_fields: u32,
    pub max_remote_fields: u32,
    pub name_length: u32,
    pub value_length: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PollLimits {
    pub max_expiration: u32,
    pub min_expiration: u32,
    pub max_option_chars: u32,
    pub max_options: u32,
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
            invites_enabled: None,
            configuration: MegalodonEntities::instance::InstanceConfig {
                statuses: MegalodonEntities::instance::Statuses {
                    max_characters: val.max_toot_chars,
                    max_media_attachments: val.max_media_attachments,
                    characters_reserved_per_url: None,
                },
                polls: Some(MegalodonEntities::instance::Polls {
                    max_options: val.poll_limits.max_options,
                    max_characters_per_option: val.poll_limits.max_option_chars,
                    min_expiration: val.poll_limits.min_expiration,
                    max_expiration: val.poll_limits.max_expiration,
                }),
            },
            contact_account: None,
            rules: None,
        }
    }
}
