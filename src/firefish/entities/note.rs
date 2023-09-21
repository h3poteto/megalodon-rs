use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, str::FromStr};

use super::{Emoji, File, Poll, User};
use crate::entities as MegalodonEntities;
use crate::error::{Error, Kind};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: String,
    created_at: DateTime<Utc>,
    // user_id: String,
    pub user: User,
    text: Option<String>,
    cw: Option<String>,
    visibility: StatusVisibility,
    renote_count: u32,
    replies_count: u32,
    reactions: HashMap<String, u32>,
    emojis: Option<Vec<Emoji>>,
    // file_ids: Option<Vec<String>>,
    files: Option<Vec<File>>,
    reply_id: Option<String>,
    // renote_id: Option<String>,
    uri: Option<String>,
    // reply: Option<Box<Note>>,
    renote: Option<Box<Note>>,
    tags: Option<Vec<String>>,
    poll: Option<Poll>,
    // mentions: Option<Vec<String>>,
    my_reaction: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Home,
    Followers,
    Specified,
    Hidden,
}

impl fmt::Display for StatusVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusVisibility::Public => write!(f, "public"),
            StatusVisibility::Home => write!(f, "home"),
            StatusVisibility::Followers => write!(f, "followers"),
            StatusVisibility::Specified => write!(f, "specified"),
            StatusVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl FromStr for StatusVisibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(StatusVisibility::Public),
            "home" => Ok(StatusVisibility::Home),
            "followers" => Ok(StatusVisibility::Followers),
            "specified" => Ok(StatusVisibility::Specified),
            "hidden" => Ok(StatusVisibility::Hidden),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

impl From<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn from(value: MegalodonEntities::status::StatusVisibility) -> Self {
        match value {
            MegalodonEntities::StatusVisibility::Public => StatusVisibility::Public,
            MegalodonEntities::StatusVisibility::Unlisted => StatusVisibility::Home,
            MegalodonEntities::StatusVisibility::Private => StatusVisibility::Followers,
            MegalodonEntities::StatusVisibility::Direct => StatusVisibility::Specified,
        }
    }
}

impl Into<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn into(self) -> MegalodonEntities::status::StatusVisibility {
        match self {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Home => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Followers => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Specified => MegalodonEntities::status::StatusVisibility::Direct,
            StatusVisibility::Hidden => MegalodonEntities::status::StatusVisibility::Direct,
        }
    }
}

impl Into<MegalodonEntities::Status> for Note {
    fn into(self) -> MegalodonEntities::Status {
        let mut uri = "".to_string();
        if let Some(u) = self.uri.clone() {
            uri = u;
        }
        let mut reblog_status: Option<Box<MegalodonEntities::Status>> = None;
        let mut quoted = false;
        if let Some(renote) = self.renote {
            let rs: Note = *renote;
            reblog_status = Some(Box::new(rs.into()));
            if let Some(_) = self.text.clone() {
                quoted = true;
            }
        }
        let mut content = "".to_string();
        if let Some(text) = self.text.clone() {
            content = text;
        }

        let mut spoiler_text = "".to_string();
        if let Some(cw) = self.cw {
            spoiler_text = cw;
        }

        let mut tags: Vec<MegalodonEntities::status::Tag> = [].to_vec();
        if let Some(hashtags) = self.tags {
            tags = hashtags
                .into_iter()
                .map(|t| MegalodonEntities::status::Tag {
                    name: t.clone(),
                    url: t,
                })
                .collect();
        }
        let emoji_reactions = Some(map_reactions(self.reactions, self.my_reaction.clone()));

        MegalodonEntities::Status {
            id: self.id,
            uri: uri.clone(),
            url: self.uri,
            account: self.user.into(),
            in_reply_to_id: self.reply_id,
            in_reply_to_account_id: None,
            reblog: reblog_status,
            content,
            plain_content: self.text,
            created_at: self.created_at,
            emojis: self
                .emojis
                .map_or([].to_vec(), |o| o.into_iter().map(|e| e.into()).collect()),
            replies_count: self.replies_count,
            reblogs_count: self.renote_count,
            favourites_count: 0,
            reblogged: None,
            favourited: Some(self.my_reaction.is_some()),
            muted: None,
            sensitive: self
                .files
                .as_ref()
                .map_or(false, |f| f.iter().any(|f| f.is_sensitive)),
            spoiler_text,
            visibility: self.visibility.into(),
            media_attachments: self
                .files
                .map_or([].to_vec(), |f| f.into_iter().map(|f| f.into()).collect()),
            mentions: [].to_vec(),
            tags,
            card: None,
            poll: self.poll.map(|p| p.into()),
            application: None,
            language: None,
            pinned: None,
            emoji_reactions,
            quote: quoted,
            bookmarked: None,
        }
    }
}

fn map_reactions(
    reactions: HashMap<String, u32>,
    my_reaction: Option<String>,
) -> Vec<MegalodonEntities::Reaction> {
    if let Some(my) = my_reaction {
        reactions
            .into_iter()
            .map(|(key, value)| {
                let me = my == key;
                MegalodonEntities::Reaction {
                    count: value,
                    me,
                    name: key,
                    accounts: None,
                }
            })
            .collect()
    } else {
        reactions
            .into_iter()
            .map(|(key, value)| MegalodonEntities::Reaction {
                count: value,
                me: false,
                name: key,
                accounts: None,
            })
            .collect()
    }
}

impl Into<MegalodonEntities::Conversation> for Note {
    fn into(self) -> MegalodonEntities::Conversation {
        let accounts: Vec<MegalodonEntities::Account> = [self.user.clone().into()].to_vec();
        MegalodonEntities::Conversation {
            id: self.id.clone(),
            accounts,
            last_status: Some(self.into()),
            unread: false,
        }
    }
}
