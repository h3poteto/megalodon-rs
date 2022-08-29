use super::{Attachment, StatusParams};
use chrono::{DateTime, Utc};

pub struct ScheduledStatus {
    pub id: String,
    pub scheduled_at: DateTime<Utc>,
    pub params: StatusParams,
    pub media_attachments: Vec<Attachment>,
}
