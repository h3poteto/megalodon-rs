use super::PollOption;
use chrono::{DateTime, Utc};

pub struct Poll {
    pub id: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: bool,
    pub options: Vec<PollOption>,
    pub voted: bool,
}
