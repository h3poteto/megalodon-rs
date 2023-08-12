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

impl Into<MegalodonEntities::ScheduledStatus> for ScheduledStatus {
    fn into(self) -> MegalodonEntities::ScheduledStatus {
        MegalodonEntities::ScheduledStatus {
            id: self.id,
            scheduled_at: self.scheduled_at,
            params: self.params.into(),
            media_attachments: Some(
                self.media_attachments
                    .into_iter()
                    .map(|i| i.into())
                    .collect(),
            ),
        }
    }
}

impl Into<megalodon::PostStatusOutput> for ScheduledStatus {
    fn into(self) -> megalodon::PostStatusOutput {
        megalodon::PostStatusOutput::ScheduledStatus(self.into())
    }
}
