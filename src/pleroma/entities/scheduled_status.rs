use super::{Attachment, StatusParams};
use crate::{entities as MegalodonEntities, megalodon};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ScheduledStatus {
    id: String,
    scheduled_at: DateTime<Utc>,
    params: StatusParams,
    media_attachments: Option<Vec<Attachment>>,
}

impl From<ScheduledStatus> for MegalodonEntities::ScheduledStatus {
    fn from(val: ScheduledStatus) -> Self {
        MegalodonEntities::ScheduledStatus {
            id: val.id,
            scheduled_at: val.scheduled_at,
            params: val.params.into(),
            media_attachments: val
                .media_attachments
                .map(|m| m.into_iter().map(|i| i.into()).collect()),
        }
    }
}

impl From<ScheduledStatus> for megalodon::PostStatusOutput {
    fn from(val: ScheduledStatus) -> Self {
        megalodon::PostStatusOutput::ScheduledStatus(val.into())
    }
}
