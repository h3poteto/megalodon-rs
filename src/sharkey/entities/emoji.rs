use std::{collections::HashMap, ops::Deref};

use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
pub struct Emoji(HashMap<String, String>);

impl Default for Emoji {
    fn default() -> Self {
        Emoji::new()
    }
}

impl Emoji {
    pub fn new() -> Self {
        Emoji(HashMap::<String, String>::new())
    }

    pub fn map(&self) -> HashMap<String, String> {
        self.0.clone()
    }
}

impl Deref for Emoji {
    type Target = HashMap<String, String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Emoji> for Vec<MegalodonEntities::Emoji> {
    fn from(val: Emoji) -> Self {
        val.iter()
            .map(|(name, url)| MegalodonEntities::Emoji {
                shortcode: name.clone(),
                static_url: url.clone(),
                url: url.clone(),
                visible_in_picker: true,
                category: None,
            })
            .collect()
    }
}
