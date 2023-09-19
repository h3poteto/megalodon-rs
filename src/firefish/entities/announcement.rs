use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    id: String,
    created_at: DateTime<Utc>,
    updated_at: Option<DateTime<Utc>>,
    text: String,
    title: String,
    // image_url: Option<String>,
    is_read: Option<bool>,
    // show_popup: bool,
    // is_good_news: bool,
}

impl Into<MegalodonEntities::Announcement> for Announcement {
    fn into(self) -> MegalodonEntities::Announcement {
        MegalodonEntities::Announcement {
            id: self.id,
            content: format!("{}\n{}", self.title, self.text),
            starts_at: None,
            ends_at: None,
            published: true,
            all_day: true,
            published_at: self.created_at,
            updated_at: self.updated_at,
            read: self.is_read,
            mentions: [].to_vec(),
            statuses: [].to_vec(),
            tags: [].to_vec(),
            emojis: [].to_vec(),
            reactions: [].to_vec(),
        }
    }
}
