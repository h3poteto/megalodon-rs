use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction};
use crate::error::{Error, Kind};
use chrono::{DateTime, Utc};
use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Status {
    pub id: String,
    pub uri: String,
    pub url: Option<String>,
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
    pub emoji_reactions: Option<Vec<Reaction>>,
    pub quote: bool,
    pub bookmarked: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

impl fmt::Display for StatusVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusVisibility::Public => write!(f, "public"),
            StatusVisibility::Unlisted => write!(f, "unlisted"),
            StatusVisibility::Private => write!(f, "private"),
            StatusVisibility::Direct => write!(f, "direct"),
        }
    }
}

impl FromStr for StatusVisibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(StatusVisibility::Public),
            "unlisted" => Ok(StatusVisibility::Unlisted),
            "private" => Ok(StatusVisibility::Private),
            "direct" => Ok(StatusVisibility::Direct),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}
