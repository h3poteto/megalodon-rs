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

impl From<Announcement> for MegalodonEntities::Announcement {
    fn from(val: Announcement) -> Self {
        MegalodonEntities::Announcement {
            id: val.id,
            content: format!("{}\n{}", val.title, val.text),
            starts_at: None,
            ends_at: None,
            published: true,
            all_day: true,
            published_at: val.created_at,
            updated_at: val.updated_at,
            read: val.is_read,
            mentions: [].to_vec(),
            statuses: [].to_vec(),
            tags: [].to_vec(),
            emojis: [].to_vec(),
            reactions: [].to_vec(),
        }
    }
}
