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

impl Into<MegalodonEntities::StatusParams> for StatusParams {
    fn into(self) -> MegalodonEntities::StatusParams {
        MegalodonEntities::StatusParams {
            text: self.text,
            in_reply_to_id: self.in_reply_to_id,
            media_ids: self.media_ids,
            sensitive: self.sensitive,
            spoiler_text: self.spoiler_text,
            visibility: self.visibility.map(|i| i.into()),
            scheduled_at: self.scheduled_at,
            application_id: Some(self.application_id),
        }
    }
}
