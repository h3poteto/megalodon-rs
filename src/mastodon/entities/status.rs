use core::fmt;
use std::str::FromStr;

use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll};
use crate::error::{Error, Kind};
use crate::{entities as MegalodonEntities, megalodon};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Status {
    id: String,
    uri: String,
    url: Option<String>,
    account: Account,
    in_reply_to_id: Option<String>,
    in_reply_to_account_id: Option<String>,
    reblog: Option<Box<Status>>,
    content: String,
    created_at: DateTime<Utc>,
    edited_at: Option<DateTime<Utc>>,
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
    quote: Option<Box<Status>>,
    bookmarked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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

impl From<Tag> for MegalodonEntities::status::Tag {
    fn from(val: Tag) -> Self {
        MegalodonEntities::status::Tag {
            name: val.name,
            url: val.url,
        }
    }
}

impl From<StatusVisibility> for MegalodonEntities::status::StatusVisibility {
    fn from(val: StatusVisibility) -> Self {
        match val {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Unlisted => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Private => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Direct => MegalodonEntities::status::StatusVisibility::Direct,
        }
    }
}

impl From<Status> for MegalodonEntities::Status {
    fn from(val: Status) -> Self {
        let mut reblog_status: Option<Box<MegalodonEntities::Status>> = None;
        let mut quoted = false;
        if let Some(reblog) = val.reblog {
            let rs: Status = *reblog;
            reblog_status = Some(Box::new(rs.into()));
        } else if let Some(quote) = val.quote {
            let rs: Status = *quote;
            reblog_status = Some(Box::new(rs.into()));
            quoted = true;
        }

        MegalodonEntities::Status {
            id: val.id,
            uri: val.uri,
            url: val.url,
            account: val.account.into(),
            in_reply_to_id: val.in_reply_to_id,
            in_reply_to_account_id: val.in_reply_to_account_id,
            reblog: reblog_status,
            content: val.content,
            plain_content: None,
            created_at: val.created_at,
            edited_at: val.edited_at,
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            replies_count: val.replies_count,
            reblogs_count: val.reblogs_count,
            favourites_count: val.favourites_count,
            reblogged: val.reblogged,
            favourited: val.favourited,
            muted: val.muted,
            sensitive: val.sensitive,
            spoiler_text: val.spoiler_text,
            visibility: val.visibility.into(),
            media_attachments: val
                .media_attachments
                .into_iter()
                .map(|i| i.into())
                .collect(),
            mentions: val.mentions.into_iter().map(|i| i.into()).collect(),
            tags: val.tags.into_iter().map(|i| i.into()).collect(),
            card: val.card.map(|i| i.into()),
            poll: val.poll.map(|i| i.into()),
            application: val.application.map(|i| i.into()),
            language: val.language,
            pinned: val.pinned,
            emoji_reactions: None,
            quote: quoted,
            bookmarked: val.bookmarked,
        }
    }
}

impl From<Status> for megalodon::PostStatusOutput {
    fn from(val: Status) -> Self {
        megalodon::PostStatusOutput::Status(val.into())
    }
}
