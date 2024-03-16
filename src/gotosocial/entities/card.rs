use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}

impl From<CardType> for MegalodonEntities::card::CardType {
    fn from(val: CardType) -> MegalodonEntities::card::CardType {
        match val {
            CardType::Link => MegalodonEntities::card::CardType::Link,
            CardType::Photo => MegalodonEntities::card::CardType::Photo,
            CardType::Video => MegalodonEntities::card::CardType::Video,
            CardType::Rich => MegalodonEntities::card::CardType::Rich,
        }
    }
}

impl From<Card> for MegalodonEntities::Card {
    fn from(val: Card) -> MegalodonEntities::Card {
        MegalodonEntities::Card {
            url: val.url,
            title: val.title,
            description: val.description,
            r#type: val.r#type.into(),
            image: val.image,
            author_name: val.author_name,
            author_url: val.author_url,
            provider_name: val.provider_name.map_or(String::from(""), |f| f),
            provider_url: val.provider_url.map_or(String::from(""), |f| f),
            html: val.html,
            width: val.width,
            height: val.height,
            embed_url: None,
            blurhash: None,
        }
    }
}
