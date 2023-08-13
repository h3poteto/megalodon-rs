use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Card {
    url: String,
    title: String,
    description: String,
    r#type: CardType,
    image: Option<String>,
    author_name: String,
    author_url: String,
    provider_name: String,
    provider_url: String,
    html: String,
    width: u32,
    height: u32,
    embed_url: String,
    blurhash: Option<String>,
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
            author_name: Some(self.author_name),
            author_url: Some(self.author_url),
            provider_name: self.provider_name,
            provider_url: self.provider_url,
            html: Some(self.html),
            width: Some(self.width),
            height: Some(self.height),
            embed_url: Some(self.embed_url),
            blurhash: self.blurhash,
        }
    }
}
