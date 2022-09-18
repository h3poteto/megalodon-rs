use crate::entities as MegalodonEntities;
use core::fmt;
use serde::{de, ser, Deserialize};
use std::str::FromStr;

use crate::error::{Error, Kind};

#[derive(Debug, Deserialize, Clone)]
pub struct Card {
    url: String,
    title: String,
    description: String,
    r#type: CardType,
    image: Option<String>,
    author_name: Option<String>,
    author_url: Option<String>,
    provider_name: Option<String>,
    provider_url: Option<String>,
    html: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Clone)]
pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}

impl fmt::Display for CardType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CardType::Link => write!(f, "link"),
            CardType::Photo => write!(f, "photo"),
            CardType::Video => write!(f, "video"),
            CardType::Rich => write!(f, "rich"),
        }
    }
}

impl FromStr for CardType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "link" => Ok(CardType::Link),
            "photo" => Ok(CardType::Photo),
            "video" => Ok(CardType::Video),
            "rich" => Ok(CardType::Rich),
            _ => Err(Error::new(None, None, s.to_owned(), Kind::ParseError)),
        }
    }
}

impl ser::Serialize for CardType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

impl<'de> de::Deserialize<'de> for CardType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match CardType::from_str(s.as_str()) {
            Ok(r) => Ok(r),
            Err(e) => Err(de::Error::custom(e)),
        }
    }
}

impl Into<MegalodonEntities::card::CardType> for CardType {
    fn into(self) -> MegalodonEntities::card::CardType {
        match self {
            CardType::Link => MegalodonEntities::card::CardType::Link,
            CardType::Photo => MegalodonEntities::card::CardType::Photo,
            CardType::Video => MegalodonEntities::card::CardType::Video,
            CardType::Rich => MegalodonEntities::card::CardType::Rich,
        }
    }
}

impl Into<MegalodonEntities::Card> for Card {
    fn into(self) -> MegalodonEntities::Card {
        MegalodonEntities::Card {
            url: self.url,
            title: self.title,
            description: self.description,
            r#type: self.r#type.into(),
            image: self.image,
            author_name: self.author_name,
            author_url: self.author_url,
            provider_name: self.provider_name,
            provider_url: self.provider_url,
            html: self.html,
            width: self.width,
            height: self.height,
        }
    }
}
