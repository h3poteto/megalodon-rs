use core::fmt;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

use super::StatusVisibility;
use crate::error::{Error, Kind};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Preferences {
    pub posting_default_visibility: StatusVisibility,
    pub posting_default_sensitive: bool,
    pub posting_default_language: Option<String>,
    pub reading_expand_media: ExpandMedia,
    pub reading_expand_spoilers: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
