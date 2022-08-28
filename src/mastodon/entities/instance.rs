use super::{Account, Stats, URLs};
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Instance {
    uri: String,
    title: String,
    description: String,
    email: String,
    version: String,
    thumbnail: Option<String>,
    urls: URLs,
    stats: Stats,
    languages: Vec<String>,
    contact_account: Option<Account>,
    max_toot_chars: Option<usize>,
    registrations: Option<bool>,
    configuration: Option<InstanceConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InstanceConfig {
    statuses: Statuses,
    media_attachments: MediaAttachments,
    polls: Polls,
}

#[derive(Deserialize, Debug, Clone)]
struct Statuses {
    max_characters: u32,
    max_media_attachments: u32,
    characters_reserved_per_url: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct MediaAttachments {
    supported_mime_types: Vec<String>,
    image_size_limit: u32,
    image_matrix_limit: u32,
    video_size_limit: u32,
    video_frame_rate_limit: u32,
    video_matrix_limit: u32,
}

#[derive(Deserialize, Debug, Clone)]
struct Polls {
    max_options: u32,
    max_characters_per_option: u32,
    min_expiration: u32,
    max_expiration: u32,
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
            contact_account: self.contact_account.map(|i| i.into()),
            max_toot_chars: self.max_toot_chars,
            registrations: self.registrations,
            configuration: self.configuration.map(|i| i.into()),
        }
    }
}

impl Into<MegalodonEntities::instance::InstanceConfig> for InstanceConfig {
    fn into(self) -> MegalodonEntities::instance::InstanceConfig {
        MegalodonEntities::instance::InstanceConfig {
            statuses: self.statuses.into(),
            media_attachments: self.media_attachments.into(),
            polls: self.polls.into(),
        }
    }
}

impl Into<MegalodonEntities::instance::Statuses> for Statuses {
    fn into(self) -> MegalodonEntities::instance::Statuses {
        MegalodonEntities::instance::Statuses {
            max_characters: self.max_characters,
            max_media_attachments: self.max_media_attachments,
            characters_reserved_per_url: self.characters_reserved_per_url,
        }
    }
}

impl Into<MegalodonEntities::instance::MediaAttachments> for MediaAttachments {
    fn into(self) -> MegalodonEntities::instance::MediaAttachments {
        MegalodonEntities::instance::MediaAttachments {
            supported_mime_types: self.supported_mime_types,
            image_size_limit: self.image_size_limit,
            image_matrix_limit: self.image_matrix_limit,
            video_size_limit: self.video_size_limit,
            video_frame_rate_limit: self.video_frame_rate_limit,
            video_matrix_limit: self.video_matrix_limit,
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
