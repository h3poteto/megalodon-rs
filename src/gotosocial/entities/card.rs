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
            provider_name: self.provider_name.map_or(String::from(""), |f| f),
            provider_url: self.provider_url.map_or(String::from(""), |f| f),
            html: self.html,
            width: self.width,
            height: self.height,
            embed_url: None,
            blurhash: None,
        }
    }
}
