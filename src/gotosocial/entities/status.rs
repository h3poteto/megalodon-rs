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
    bookmarked: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
    Local,
}

impl fmt::Display for StatusVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusVisibility::Public => write!(f, "public"),
            StatusVisibility::Unlisted => write!(f, "unlisted"),
            StatusVisibility::Private => write!(f, "private"),
            StatusVisibility::Direct => write!(f, "direct"),
            StatusVisibility::Local => write!(f, "local"),
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
            "local" => Ok(StatusVisibility::Local),
            _ => Err(Error::new_own(
                s.to_owned(),
                Kind::ParseError,
                None,
                None,
                None,
            )),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

impl From<Tag> for MegalodonEntities::status::Tag {
    fn from(val: Tag) -> MegalodonEntities::status::Tag {
        MegalodonEntities::status::Tag {
            name: val.name,
            url: val.url,
        }
    }
}

impl From<StatusVisibility> for MegalodonEntities::status::StatusVisibility {
    fn from(val: StatusVisibility) -> MegalodonEntities::status::StatusVisibility {
        match val {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Unlisted => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Private => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Direct => MegalodonEntities::status::StatusVisibility::Direct,
            StatusVisibility::Local => MegalodonEntities::status::StatusVisibility::Local,
        }
    }
}

impl From<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn from(val: MegalodonEntities::status::StatusVisibility) -> StatusVisibility {
        match val {
            MegalodonEntities::status::StatusVisibility::Public => StatusVisibility::Public,
            MegalodonEntities::status::StatusVisibility::Unlisted => StatusVisibility::Unlisted,
            MegalodonEntities::status::StatusVisibility::Private => StatusVisibility::Private,
            MegalodonEntities::status::StatusVisibility::Direct => StatusVisibility::Direct,
            MegalodonEntities::status::StatusVisibility::Local => StatusVisibility::Local,
        }
    }
}

impl From<Status> for MegalodonEntities::Status {
    fn from(val: Status) -> MegalodonEntities::Status {
        MegalodonEntities::Status {
            id: val.id,
            uri: val.uri,
            url: val.url,
            account: val.account.into(),
            in_reply_to_id: val.in_reply_to_id,
            in_reply_to_account_id: val.in_reply_to_account_id,
            reblog: val.reblog.map(|r| {
                let rs: Status = *r;
                Box::new(rs.into())
            }),
            content: val.content,
            plain_content: None,
            created_at: val.created_at,
            edited_at: None,
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
            quote: None,
            quote_approval: MegalodonEntities::QuoteApproval::default(),
            bookmarked: val.bookmarked,
        }
    }
}

impl From<Status> for megalodon::PostStatusOutput {
    fn from(val: Status) -> megalodon::PostStatusOutput {
        megalodon::PostStatusOutput::Status(val.into())
    }
}
