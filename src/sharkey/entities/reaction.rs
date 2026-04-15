use crate::entities as MegalodonEntities;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Reaction {
    // id: String,
    // created_at: DateTime<Utc>,
    // user: User,
    // r#type: String,
}

pub(crate) fn map_reaction(
    emojis: HashMap<String, String>,
    reactions: HashMap<String, u32>,
    my_reaction: Option<String>,
) -> Vec<MegalodonEntities::Reaction> {
    let emoji_urls: HashMap<String, String> = emojis.clone();
    reactions
        .clone()
        .into_iter()
        .map(|(key, _value)| {
            let shortcode = key.replace(":", "");
            let url = emoji_urls.get::<String>(&shortcode).map(|u| u.to_string());
            let name = shortcode.replace("@.", "");
            let me = if let Some(my) = &my_reaction {
                key == my.clone()
            } else {
                false
            };
            MegalodonEntities::Reaction {
                count: reactions[&key],
                me,
                name,
                url: url.clone(),
                static_url: url,
                accounts: None,
                account_ids: None,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_map_reaction() {
        let emojis = HashMap::<String, String>::from([
            (
                "foxverified".to_owned(),
                "https://example.com/files/foxverified".to_owned(),
            ),
            (
                "verificado".to_owned(),
                "https://example.com/files/verificado".to_owned(),
            ),
            (
                "kawaii@sharkey.example".to_owned(),
                "https://example.com/proxy/sharkeyexample/kawaii".to_owned(),
            ),
            (
                "ablobcatnodfast@.".to_owned(),
                "https://example.com/files/ablobcatnodfast".to_owned(),
            ),
        ]);
        let reactions = HashMap::from([
            (String::from(":ablobcatnodfast@.:"), 2),
            (String::from(":kawaii@sharkey.example:"), 1),
        ]);

        let res = map_reaction(emojis, reactions, None);
        assert_eq!(res.len(), 2);

        let ablobcat = res
            .iter()
            .find(|r| r.name == String::from("ablobcatnodfast"));
        assert_ne!(ablobcat, None);

        assert_eq!(
            ablobcat.unwrap(),
            &MegalodonEntities::Reaction {
                count: 2,
                me: false,
                name: String::from("ablobcatnodfast"),
                url: Some(String::from("https://example.com/files/ablobcatnodfast")),
                static_url: Some(String::from("https://example.com/files/ablobcatnodfast")),
                accounts: None,
                account_ids: None,
            },
        );

        let kawaii = res
            .iter()
            .find(|r| r.name == String::from("kawaii@sharkey.example"));
        assert_ne!(kawaii, None);

        assert_eq!(
            kawaii.unwrap(),
            &MegalodonEntities::Reaction {
                count: 1,
                me: false,
                name: String::from("kawaii@sharkey.example"),
                url: Some(String::from(
                    "https://example.com/proxy/sharkeyexample/kawaii"
                )),
                static_url: Some(String::from(
                    "https://example.com/proxy/sharkeyexample/kawaii"
                )),
                accounts: None,
                account_ids: None,
            }
        );
    }

    #[test]
    fn test_map_reaction_empty_emojis() {
        let emojis = HashMap::<String, String>::new();
        let reactions = HashMap::from([
            (String::from(":ablobcatnodfast@.:"), 2),
            (String::from(":kawaii@sharkey.example:"), 1),
        ]);

        let res = map_reaction(emojis, reactions, None);
        assert_eq!(res.len(), 2);

        let ablobcat = res
            .iter()
            .find(|r| r.name == String::from("ablobcatnodfast"));
        assert_ne!(ablobcat, None);

        assert_eq!(
            ablobcat.unwrap(),
            &MegalodonEntities::Reaction {
                count: 2,
                me: false,
                name: String::from("ablobcatnodfast"),
                url: None,
                static_url: None,
                accounts: None,
                account_ids: None,
            },
        );

        let kawaii = res
            .iter()
            .find(|r| r.name == String::from("kawaii@sharkey.example"));
        assert_ne!(kawaii, None);

        assert_eq!(
            kawaii.unwrap(),
            &MegalodonEntities::Reaction {
                count: 1,
                me: false,
                name: String::from("kawaii@sharkey.example"),
                url: None,
                static_url: None,
                accounts: None,
                account_ids: None,
            }
        );
    }

    #[test]
    fn test_map_reaction_with_me() {
        let emojis = HashMap::<String, String>::from([
            (
                "foxverified".to_owned(),
                "https://example.com/files/foxverified".to_owned(),
            ),
            (
                "verificado".to_owned(),
                "https://example.com/files/verificado".to_owned(),
            ),
            (
                "kawaii@sharkey.example".to_owned(),
                "https://example.com/proxy/sharkeyexample/kawaii".to_owned(),
            ),
            (
                "ablobcatnodfast@.".to_owned(),
                "https://example.com/files/ablobcatnodfast".to_owned(),
            ),
        ]);
        let reactions = HashMap::from([
            (String::from(":ablobcatnodfast@.:"), 2),
            (String::from(":kawaii@sharkey.example:"), 1),
        ]);

        let res = map_reaction(emojis, reactions, Some(String::from(":ablobcatnodfast@.:")));
        assert_eq!(res.len(), 2);

        let ablobcat = res
            .iter()
            .find(|r| r.name == String::from("ablobcatnodfast"));
        assert_ne!(ablobcat, None);

        assert_eq!(
            ablobcat.unwrap(),
            &MegalodonEntities::Reaction {
                count: 2,
                me: true,
                name: String::from("ablobcatnodfast"),
                url: Some(String::from("https://example.com/files/ablobcatnodfast")),
                static_url: Some(String::from("https://example.com/files/ablobcatnodfast")),
                accounts: None,
                account_ids: None,
            },
        );

        let kawaii = res
            .iter()
            .find(|r| r.name == String::from("kawaii@sharkey.example"));
        assert_ne!(kawaii, None);

        assert_eq!(
            kawaii.unwrap(),
            &MegalodonEntities::Reaction {
                count: 1,
                me: false,
                name: String::from("kawaii@sharkey.example"),
                url: Some(String::from(
                    "https://example.com/proxy/sharkeyexample/kawaii"
                )),
                static_url: Some(String::from(
                    "https://example.com/proxy/sharkeyexample/kawaii"
                )),
                accounts: None,
                account_ids: None,
            }
        );
    }
}
