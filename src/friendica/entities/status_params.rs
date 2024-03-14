use super::StatusVisibility;
use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct StatusParams {
    text: String,
    in_reply_to_id: Option<String>,
    media_ids: Option<Vec<String>>,
    sensitive: Option<bool>,
    spoiler_text: Option<String>,
    visibility: Option<StatusVisibility>,
    scheduled_at: Option<DateTime<Utc>>,
    application_id: String,
}

impl From<StatusParams> for MegalodonEntities::StatusParams {
    fn from(val: StatusParams) -> Self {
        let mut app_id: Option<u32> = None;
        if let Ok(val) = val.application_id.parse::<u32>() {
            app_id = Some(val);
        }

        MegalodonEntities::StatusParams {
            text: val.text,
            in_reply_to_id: val.in_reply_to_id,
            media_ids: val.media_ids,
            sensitive: val.sensitive,
            spoiler_text: val.spoiler_text,
            visibility: val.visibility.map(|i| i.into()),
            scheduled_at: val.scheduled_at,
            application_id: app_id,
        }
    }
}
