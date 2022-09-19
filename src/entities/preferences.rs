use core::fmt;
use serde::{de, ser};
use std::str::FromStr;

use crate::error::{Error, Kind};
use crate::mastodon::entities::StatusVisibility;

#[derive(Debug, Clone)]
pub struct Preferences {
    pub posting_default_visibility: StatusVisibility,
    pub posting_default_sensitive: bool,
    pub posting_default_language: Option<String>,
    pub reading_expand_media: ExpandMedia,
    pub reading_expand_spoilers: bool,
}

#[derive(Debug, Clone)]
pub enum ExpandMedia {
    Default,
    ShowAll,
    HideAll,
}

impl fmt::Display for ExpandMedia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExpandMedia::Default => write!(f, "default"),
            ExpandMedia::ShowAll => write!(f, "show_all"),
            ExpandMedia::HideAll => write!(f, "hide_all"),
        }
    }
}

impl FromStr for ExpandMedia {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(ExpandMedia::Default),
            "show_all" => Ok(ExpandMedia::ShowAll),
            "hide_all" => Ok(ExpandMedia::HideAll),
            _ => Err(Error::new(None, None, s.to_owned(), Kind::ParseError)),
        }
    }
}

impl ser::Serialize for ExpandMedia {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for ExpandMedia {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match ExpandMedia::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}
