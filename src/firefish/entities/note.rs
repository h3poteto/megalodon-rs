use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, str::FromStr};

use super::{Emoji, File, Poll, User};
use crate::error::{Error, Kind};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    id: String,
    created_at: DateTime<Utc>,
    user_id: String,
    user: User,
    text: Option<String>,
    cw: Option<String>,
    visibility: StatusVisibility,
    renote_count: u32,
    replies_count: u32,
    reactions: HashMap<String, u32>,
    emojis: Vec<Emoji>,
    file_ids: Vec<String>,
    files: Vec<File>,
    reply_id: Option<String>,
    renote_id: Option<String>,
    uri: Option<String>,
    reply: Option<Box<Note>>,
    renote: Option<Box<Note>>,
    tags: Option<Vec<String>>,
    poll: Option<Poll>,
    mentions: Vec<String>,
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
