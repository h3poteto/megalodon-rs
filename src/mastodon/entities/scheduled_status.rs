use super::{Attachment, StatusParams};
use chrono::{DateTime, Utc};

pub struct ScheduledStatus {
    id: String,
    scheduled_at: DateTime<Utc>,
    params: StatusParams,
    media_attachments: Vec<Attachment>,
}
