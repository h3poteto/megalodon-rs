use core::fmt;
use std::str::FromStr;

use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction, Tag};
use crate::{entities as MegalodonEntities, error::Error};
use chrono::{DateTime, Utc};
use serde::{de, ser, Deserialize};

#[derive(Debug, Deserialize, Clone)]
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

#[derive(Debug, Clone)]
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
            _ => Err(Error::new(s.to_owned())),
        }
    }
}

impl ser::Serialize for StatusVisibility {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for StatusVisibility {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match StatusVisibility::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl Into<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn into(self) -> MegalodonEntities::status::StatusVisibility {
        match self {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Unlisted => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Private => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Direct => MegalodonEntities::status::StatusVisibility::Direct,
        }
    }
}

impl Into<MegalodonEntities::Status> for Status {
    fn into(self) -> MegalodonEntities::Status {
        let mut reblog_status: Option<Box<MegalodonEntities::Status>> = None;
        if let Some(reblog) = self.reblog {
            let rs: Status = *reblog;
            reblog_status = Some(Box::new(rs.into()));
        }

        MegalodonEntities::Status {
            id: self.id,
            uri: self.uri,
            url: self.url,
            account: self.account.into(),
            in_reply_to_id: self.in_reply_to_id,
            in_reply_to_account_id: self.in_reply_to_account_id,
            reblog: reblog_status,
            content: self.content,
            plain_content: self.plain_content,
            created_at: self.created_at,
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
            replies_count: self.replies_count,
            reblogs_count: self.reblogs_count,
            favourites_count: self.favourites_count,
            reblogged: self.reblogged,
            favourited: self.favourited,
            muted: self.muted,
            sensitive: self.sensitive,
            spoiler_text: self.spoiler_text,
            visibility: self.visibility.into(),
            media_attachments: self
                .media_attachments
                .into_iter()
                .map(|i| i.into())
                .collect(),
            mentions: self.mentions.into_iter().map(|i| i.into()).collect(),
            tags: self.tags.into_iter().map(|i| i.into()).collect(),
            card: self.card.map(|i| i.into()),
            poll: self.poll.map(|i| i.into()),
            application: self.application.map(|i| i.into()),
            language: self.language,
            pinned: self.pinned,
            emoji_reactions: self.emoji_reactions.into_iter().map(|i| i.into()).collect(),
            quote: self.quote,
            bookmarked: self.bookmarked,
        }
    }
}
