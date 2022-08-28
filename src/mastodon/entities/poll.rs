use super::PollOption;
use chrono::{DateTime, Utc};

pub struct Poll {
    id: String,
    expires_at: Option<DateTime<Utc>>,
    expired: bool,
    multiple: bool,
    votes_count: bool,
    options: Vec<PollOption>,
    voted: bool,
}
