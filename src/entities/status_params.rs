use super::StatusVisibility;
use chrono::{DateTime, Utc};

pub struct StatusParams {
    text: String,
    in_reply_to_id: Option<String>,
    media_ids: Option<Vec<String>>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    visibility: StatusVisibility,
    scheduled_at: Option<DateTime<Utc>>,
    application_id: String,
}
