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
    image_url: Option<String>,
    is_read: Option<bool>,
    show_popup: bool,
    is_good_news: bool,
}
