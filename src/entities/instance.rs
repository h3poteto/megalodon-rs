use super::{Account, Stats, URLs};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
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
    pub invites_enabled: Option<bool>,
    pub configuration: InstanceConfig,
    pub contact_account: Option<Account>,
    pub rules: Option<Vec<InstanceRule>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct InstanceConfig {
    pub statuses: Statuses,
    pub polls: Polls,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Statuses {
    pub max_characters: u32,
    pub max_media_attachments: Option<u32>,
    pub characters_reserved_per_url: Option<u32>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Polls {
    pub max_options: u32,
    pub max_characters_per_option: u32,
    pub min_expiration: u32,
    pub max_expiration: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InstanceRule {
    pub id: String,
    pub text: String,
}
