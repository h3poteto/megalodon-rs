use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction, Tag};
use chrono::{DateTime, Utc};

pub struct Status {
    pub id: String,
    pub uri: String,
    pub url: String,
    pub account: Account,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub plain_content: Option<String>,
    pub created_at: DateTime<Utc>,
    pub emojis: Vec<Emoji>,
    pub replies_count: u32,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub reblogged: Option<bool>,
    pub favourited: Option<bool>,
    pub muted: Option<bool>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: StatusVisibility,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub card: Option<Card>,
    pub poll: Option<Poll>,
    pub application: Option<Application>,
    pub language: Option<String>,
    pub pinned: Option<bool>,
    pub emoji_reactions: Vec<Reaction>,
    pub quote: bool,
    pub bookmarked: bool,
}

pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}
