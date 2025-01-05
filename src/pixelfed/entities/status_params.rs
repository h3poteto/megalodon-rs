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
    application_id: u32,
}

impl From<StatusParams> for MegalodonEntities::StatusParams {
    fn from(val: StatusParams) -> Self {
        MegalodonEntities::StatusParams {
            text: val.text,
            in_reply_to_id: val.in_reply_to_id,
            media_ids: val.media_ids,
            sensitive: val.sensitive,
            spoiler_text: val.spoiler_text,
            visibility: val.visibility.map(|i| i.into()),
            scheduled_at: val.scheduled_at,
            application_id: Some(val.application_id),
        }
    }
}
