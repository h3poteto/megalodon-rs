use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Card {
    url: String,
    title: String,
    description: String,
    r#type: CardType,
    image: Option<String>,
    provider_name: String,
    provider_url: String,
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
    fn from(val: CardType) -> Self {
        match val {
            CardType::Link => MegalodonEntities::card::CardType::Link,
            CardType::Photo => MegalodonEntities::card::CardType::Photo,
            CardType::Video => MegalodonEntities::card::CardType::Video,
            CardType::Rich => MegalodonEntities::card::CardType::Rich,
        }
    }
}

impl From<Card> for MegalodonEntities::Card {
    fn from(val: Card) -> Self {
        MegalodonEntities::Card {
            url: val.url,
            title: val.title,
            description: val.description,
            r#type: val.r#type.into(),
            image: val.image,
            author_name: None,
            author_url: None,
            provider_name: val.provider_name,
            provider_url: val.provider_url,
            html: None,
            width: None,
            height: None,
            embed_url: None,
            blurhash: None,
        }
    }
}
