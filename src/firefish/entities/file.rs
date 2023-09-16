use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct File {
    id: String,
    created_at: DateTime<Utc>,
    name: String,
    r#type: String,
    md5: String,
    size: u32,
    is_sensitive: bool,
    properties: Properties,
    url: Option<String>,
    thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Properties {
    width: u32,
    height: u32,
    orientation: u32,
    avg_color: String,
}
