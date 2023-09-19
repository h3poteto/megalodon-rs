use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    name: String,
    uri: String,
    description: String,
    maintainer_email: Option<String>,
    version: String,
    banner_url: String,
    langs: Vec<String>,
    disable_registration: bool,
    max_note_text_length: u32,
}

impl Into<MegalodonEntities::Instance> for Meta {
    fn into(self) -> MegalodonEntities::Instance {
        let mut email = "".to_string();
        if let Some(m) = self.maintainer_email {
            email = m;
        }

        let stats = MegalodonEntities::Stats {
            user_count: 0,
            status_count: 0,
            domain_count: 0,
        };

        MegalodonEntities::Instance {
            uri: self.uri,
            title: self.name,
            description: self.description,
            email,
            version: self.version,
            thumbnail: Some(self.banner_url),
            urls: None,
            stats,
            languages: self.langs,
            registrations: !self.disable_registration,
            approval_required: false,
            invites_enabled: None,
            configuration: MegalodonEntities::instance::InstanceConfig {
                statuses: MegalodonEntities::instance::Statuses {
                    max_characters: self.max_note_text_length,
                    max_media_attachments: None,
                    characters_reserved_per_url: None,
                },
                polls: None,
            },
            contact_account: None,
            rules: None,
        }
    }
}
