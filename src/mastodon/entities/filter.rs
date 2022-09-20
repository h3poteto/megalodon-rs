use core::fmt;
use std::str::FromStr;

use crate::entities as MegalodonEntities;
use crate::error::{Error, Kind};
use chrono::{DateTime, Utc};
use serde::{de, ser, Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Filter {
    id: String,
    phrase: String,
    context: Vec<FilterContext>,
    expires_at: DateTime<Utc>,
    irreversible: bool,
    whole_word: bool,
}

#[derive(Debug, Clone)]
pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}

impl fmt::Display for FilterContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterContext::Home => write!(f, "home"),
            FilterContext::Notifications => write!(f, "notifications"),
            FilterContext::Public => write!(f, "public"),
            FilterContext::Thread => write!(f, "thread"),
        }
    }
}

impl FromStr for FilterContext {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "home" => Ok(FilterContext::Home),
            "notifications" => Ok(FilterContext::Notifications),
            "public" => Ok(FilterContext::Public),
            "thread" => Ok(FilterContext::Thread),
            _ => Err(Error::new_own(s.to_owned(), Kind::ParseError, None, None)),
        }
    }
}

impl ser::Serialize for FilterContext {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for FilterContext {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match FilterContext::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl Into<MegalodonEntities::filter::FilterContext> for FilterContext {
    fn into(self) -> MegalodonEntities::filter::FilterContext {
        match self {
            FilterContext::Home => MegalodonEntities::filter::FilterContext::Home,
            FilterContext::Notifications => MegalodonEntities::filter::FilterContext::Notifications,
            FilterContext::Public => MegalodonEntities::filter::FilterContext::Public,
            FilterContext::Thread => MegalodonEntities::filter::FilterContext::Thread,
        }
    }
}

impl Into<MegalodonEntities::Filter> for Filter {
    fn into(self) -> MegalodonEntities::Filter {
        MegalodonEntities::Filter {
            id: self.id,
            phrase: self.phrase,
            context: self.context.into_iter().map(|i| i.into()).collect(),
            expires_at: self.expires_at,
            irreversible: self.irreversible,
            whole_word: self.whole_word,
        }
    }
}
