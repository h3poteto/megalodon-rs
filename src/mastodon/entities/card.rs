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

pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}
