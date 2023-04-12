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

impl Into<MegalodonEntities::Instance> for Instance {
    fn into(self) -> MegalodonEntities::Instance {
        MegalodonEntities::Instance {
            uri: self.uri,
            title: self.title,
            description: self.description,
            email: self.email,
            version: self.version,
            thumbnail: self.thumbnail,
            urls: self.urls.map(|i| i.into()),
            stats: self.stats.into(),
            languages: self.languages,
            registrations: self.registrations,
            approval_required: self.approval_required,
            invites_enabled: Some(self.invites_enabled),
            contact_account: Some(self.contact_account.into()),
            configuration: MegalodonEntities::instance::InstanceConfig {
                statuses: MegalodonEntities::instance::Statuses {
                    max_characters: self.max_toot_chars,
                    max_media_attachments: None,
                    characters_reserved_per_url: None,
                },
                polls: None,
            },
            rules: Some(self.rules.into_iter().map(|r| r.into()).collect()),
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
