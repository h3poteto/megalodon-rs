use super::Field;
use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub privacy: Option<String>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
    pub note: String,
    pub fields: Option<Vec<Field>>,
}

impl From<MegalodonEntities::Source> for Source {
    fn from(item: MegalodonEntities::Source) -> Self {
        Self {
            privacy: item.privacy,
            sensitive: item.sensitive,
            language: item.language,
            note: item.note,
            fields: item
                .fields
                .map(|i| i.into_iter().map(|j| j.into()).collect()),
        }
    }
}

impl Into<MegalodonEntities::Source> for Source {
    fn into(self) -> MegalodonEntities::Source {
        MegalodonEntities::Source {
            privacy: self.privacy,
            sensitive: self.sensitive,
            language: self.language,
            note: self.note,
            fields: self
                .fields
                .map(|i| i.into_iter().map(|j| j.into()).collect()),
        }
    }
}
