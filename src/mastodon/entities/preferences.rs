use super::StatusVisibility;
use crate::entities as MegalodonEntities;
use serde::ser::SerializeStruct;
use serde::{de, ser, Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq)]
pub struct Preferences {
    pub posting_default_visibility: StatusVisibility,
    pub posting_default_sensitive: bool,
    pub posting_default_language: Option<String>,
    pub reading_expand_media: ExpandMedia,
    pub reading_expand_spoilers: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ExpandMedia {
    Default,
    ShowAll,
    HideAll,
}

const FIELDS: &'static [&'static str] = &[
    "posting:default:visibility",
    "posting:default:sensitive",
    "posting:default:language",
    "reading:expand:media",
    "reading:expand:spoilers",
];

impl ser::Serialize for Preferences {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Preferences", FIELDS.len())?;
        state.serialize_field(
            "posting:default:visibility",
            &self.posting_default_visibility,
        )?;
        state.serialize_field("posting:default:sensitive", &self.posting_default_sensitive)?;
        state.serialize_field("posting:default:language", &self.posting_default_language)?;
        state.serialize_field("reading:expand:media", &self.reading_expand_media)?;
        state.serialize_field("reading:expand:spoilers", &self.reading_expand_spoilers)?;
        state.end()
    }
}

impl<'de> de::Deserialize<'de> for Preferences {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            PostingDefaultVisibility,
            PostingDefaultSensitive,
            PostingDefaultLanguage,
            ReadingExpandMedia,
            ReadingExpandSpoilers,
        }
        impl<'de> de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("`posting:default:visibility` or `posting:default:sensitive` or `posting:default:language` or `reading:expand:media` or `reading:expand:spoilers`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "posting:default:visibility" => Ok(Field::PostingDefaultVisibility),
                            "posting:default:sensitive" => Ok(Field::PostingDefaultSensitive),
                            "posting:default:language" => Ok(Field::PostingDefaultLanguage),
                            "reading:expand:media" => Ok(Field::ReadingExpandMedia),
                            "reading:expand:spoilers" => Ok(Field::ReadingExpandSpoilers),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PreferencesVisitor;

        impl<'de> de::Visitor<'de> for PreferencesVisitor {
            type Value = Preferences;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct Preferences")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let posting_default_visibility = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let posting_default_sensitive = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let posting_default_language = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let reading_expand_media = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let reading_expand_spoilers = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Self::Value {
                    posting_default_visibility,
                    posting_default_sensitive,
                    posting_default_language,
                    reading_expand_media,
                    reading_expand_spoilers,
                })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut posting_default_visibility = None;
                let mut posting_default_sensitive = None;
                let mut posting_default_language = None;
                let mut reading_expand_media = None;
                let mut reading_expand_spoilers = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::PostingDefaultVisibility => {
                            if posting_default_visibility.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "posting_default_visibility",
                                ));
                            }
                            posting_default_visibility = Some(map.next_value()?);
                        }
                        Field::PostingDefaultSensitive => {
                            if posting_default_sensitive.is_some() {
                                return Err(de::Error::duplicate_field(
                                    "posting_default_sensitive",
                                ));
                            }
                            posting_default_sensitive = Some(map.next_value()?);
                        }
                        Field::PostingDefaultLanguage => {
                            if posting_default_language.is_some() {
                                return Err(de::Error::duplicate_field("posting_default_language"));
                            }
                            posting_default_language = Some(map.next_value()?);
                        }
                        Field::ReadingExpandMedia => {
                            if reading_expand_media.is_some() {
                                return Err(de::Error::duplicate_field("reading_expand_media"));
                            }
                            reading_expand_media = Some(map.next_value()?);
                        }
                        Field::ReadingExpandSpoilers => {
                            if reading_expand_spoilers.is_some() {
                                return Err(de::Error::duplicate_field("reading_expand_spoilers"));
                            }
                            reading_expand_spoilers = Some(map.next_value()?);
                        }
                    }
                }
                let posting_default_visibility = posting_default_visibility
                    .ok_or_else(|| de::Error::missing_field("posting_default_visibility"))?;
                let posting_default_sensitive = posting_default_sensitive
                    .ok_or_else(|| de::Error::missing_field("posting_default_sensitive"))?;
                let posting_default_language = posting_default_language
                    .ok_or_else(|| de::Error::missing_field("posting_default_language"))?;
                let reading_expand_media = reading_expand_media
                    .ok_or_else(|| de::Error::missing_field("reading_expand_media"))?;
                let reading_expand_spoilers = reading_expand_spoilers
                    .ok_or_else(|| de::Error::missing_field("reading_exapnd_spoilers"))?;
                Ok(Self::Value {
                    posting_default_visibility,
                    posting_default_sensitive,
                    posting_default_language,
                    reading_expand_media,
                    reading_expand_spoilers,
                })
            }
        }

        deserializer.deserialize_struct("Preferences", FIELDS, PreferencesVisitor)
    }
}

impl Into<MegalodonEntities::Preferences> for Preferences {
    fn into(self) -> MegalodonEntities::Preferences {
        MegalodonEntities::Preferences {
            posting_default_visibility: self.posting_default_visibility.into(),
            posting_default_sensitive: self.posting_default_sensitive.into(),
            posting_default_language: self.posting_default_language.into(),
            reading_expand_media: self.reading_expand_media.into(),
            reading_expand_spoilers: self.reading_expand_spoilers.into(),
        }
    }
}

impl Into<MegalodonEntities::preferences::ExpandMedia> for ExpandMedia {
    fn into(self) -> MegalodonEntities::preferences::ExpandMedia {
        match self {
            ExpandMedia::Default => MegalodonEntities::preferences::ExpandMedia::Default,
            ExpandMedia::ShowAll => MegalodonEntities::preferences::ExpandMedia::ShowAll,
            ExpandMedia::HideAll => MegalodonEntities::preferences::ExpandMedia::HideAll,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preferences_serialize() {
        let preferences = Preferences {
            posting_default_visibility: StatusVisibility::Public,
            posting_default_sensitive: false,
            posting_default_language: Some("Japanese".to_string()),
            reading_expand_media: ExpandMedia::Default,
            reading_expand_spoilers: false,
        };

        let r = serde_json::to_string(&preferences);
        assert!(r.is_ok());
        assert_eq!(
        r.unwrap(),
            r#"{"posting:default:visibility":"public","posting:default:sensitive":false,"posting:default:language":"Japanese","reading:expand:media":"default","reading:expand:spoilers":false}"#.to_string(),
        );
    }

    #[test]
    fn test_preferences_deserialize() {
        let text = r#"{"posting:default:visibility":"public","posting:default:sensitive":false,"posting:default:language":"Japanese","reading:expand:media":"default","reading:expand:spoilers":false}"#;

        let r = serde_json::from_str::<Preferences>(text);
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            Preferences {
                posting_default_visibility: StatusVisibility::Public,
                posting_default_sensitive: false,
                posting_default_language: Some("Japanese".to_string()),
                reading_expand_media: ExpandMedia::Default,
                reading_expand_spoilers: false,
            }
        );
    }
}
