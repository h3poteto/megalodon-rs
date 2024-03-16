use super::{Attachment, StatusParams};
use crate::{entities as MegalodonEntities, megalodon};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ScheduledStatus {
    id: String,
    scheduled_at: DateTime<Utc>,
    params: StatusParams,
    media_attachments: Vec<Attachment>,
}

impl From<ScheduledStatus> for MegalodonEntities::ScheduledStatus {
    fn from(val: ScheduledStatus) -> MegalodonEntities::ScheduledStatus {
        MegalodonEntities::ScheduledStatus {
            id: val.id,
            scheduled_at: val.scheduled_at,
            params: val.params.into(),
            media_attachments: Some(
                val.media_attachments
                    .into_iter()
                    .map(|i| i.into())
                    .collect(),
            ),
        }
    }
}

impl From<ScheduledStatus> for megalodon::PostStatusOutput {
    fn from(val: ScheduledStatus) -> megalodon::PostStatusOutput {
        megalodon::PostStatusOutput::ScheduledStatus(val.into())
    }
}
