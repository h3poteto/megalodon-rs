pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub r#type: CardType,
    pub image: Option<String>,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub html: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}
