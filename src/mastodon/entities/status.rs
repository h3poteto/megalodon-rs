use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction, Tag};
use chrono::{DateTime, Utc};

pub struct Status {
    id: String,
    uri: String,
    url: String,
    account: Account,
    in_reply_to_id: Option<String>,
    in_reply_to_account_id: Option<String>,
    reblog: Option<Box<Status>>,
    content: String,
    plain_content: Option<String>,
    created_at: DateTime<Utc>,
    emojis: Vec<Emoji>,
    replies_count: u32,
    reblogs_count: u32,
    favourites_count: u32,
    reblogged: Option<bool>,
    favourited: Option<bool>,
    muted: Option<bool>,
    sensitive: bool,
    spoiler_text: String,
    visibility: StatusVisibility,
    media_attachments: Vec<Attachment>,
    mentions: Vec<Mention>,
    tags: Vec<Tag>,
    card: Option<Card>,
    poll: Option<Poll>,
    application: Option<Application>,
    language: Option<String>,
    pinned: Option<bool>,
    emoji_reactions: Vec<Reaction>,
    quote: bool,
    bookmarked: bool,
}

pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}
