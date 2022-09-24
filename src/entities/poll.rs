use super::{Emoji, PollOption};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Poll {
    pub id: String,
    pub expires_at: Option<DateTime<Utc>>,
    pub expired: bool,
    pub multiple: bool,
    pub votes_count: u32,
    pub voters_count: Option<u32>,
    pub options: Vec<PollOption>,
    pub voted: Option<bool>,
    pub emojis: Vec<Emoji>,
}
