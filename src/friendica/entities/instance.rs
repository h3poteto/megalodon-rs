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
    pub urls: Option<URLs>,
    pub stats: Stats,
    pub languages: Vec<String>,
    pub registrations: bool,
    pub approval_required: bool,
    pub invites_enabled: bool,
    pub max_toot_chars: u32,
    pub contact_account: Account,
    pub rules: Vec<InstanceRule>,
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
            urls: val.urls.map(|i| i.into()),
            stats: val.stats.into(),
            languages: val.languages,
            registrations: val.registrations,
            approval_required: val.approval_required,
            invites_enabled: Some(val.invites_enabled),
            contact_account: Some(val.contact_account.into()),
            configuration: MegalodonEntities::instance::InstanceConfig {
                statuses: MegalodonEntities::instance::Statuses {
                    max_characters: val.max_toot_chars,
                    max_media_attachments: None,
                    characters_reserved_per_url: None,
                },
                polls: None,
            },
            rules: Some(val.rules.into_iter().map(|r| r.into()).collect()),
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
