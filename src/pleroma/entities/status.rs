use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction, Tag};
use crate::{entities as MegalodonEntities, megalodon};
use chrono::{DateTime, Utc};
use serde::{de, Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Status {
    pub id: String,
    pub uri: String,
    pub url: Option<String>,
    pub account: Account,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub emojis: Vec<Emoji>,
    pub replies_count: u32,
    pub reblogs_count: u32,
    pub favourites_count: u32,
    pub reblogged: Option<bool>,
    pub favourited: Option<bool>,
    pub muted: Option<bool>,
    pub sensitive: bool,
    pub spoiler_text: String,
    pub visibility: StatusVisibility,
    pub media_attachments: Vec<Attachment>,
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub card: Option<Card>,
    pub poll: Option<Poll>,
    pub application: Option<Application>,
    pub language: Option<String>,
    pub pinned: Option<bool>,
    pub bookmarked: Option<bool>,
    pub pleroma: PleromaOptions,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PleromaOptions {
    pub content: Option<PleromaContent>,
    pub spiler_text: Option<PleromaContent>,
    pub conversation_id: Option<i64>,
    pub direct_conversation_id: Option<i64>,
    pub emoji_reactions: Option<Vec<Reaction>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub in_reply_to_account_acct: Option<String>,
    pub local: bool,
    pub parent_visible: Option<bool>,
    pub pinned_at: Option<DateTime<Utc>>,
    pub thread_muted: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PleromaContent {
    pub text_plain: String,
}

impl Into<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn into(self) -> MegalodonEntities::status::StatusVisibility {
        match self {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Unlisted => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Private => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Direct => MegalodonEntities::status::StatusVisibility::Direct,
        }
    }
}

impl Into<MegalodonEntities::Status> for Status {
    fn into(self) -> MegalodonEntities::Status {
        let mut reblog_status: Option<Box<MegalodonEntities::Status>> = None;
        let mut quoted = false;
        if let Some(reblog) = self.reblog {
            let rs: Status = *reblog;
            if rs.content != self.content {
                quoted = true;
            }
            reblog_status = Some(Box::new(rs.into()));
        }

        MegalodonEntities::Status {
            id: self.id,
            uri: self.uri,
            url: self.url,
            account: self.account.into(),
            in_reply_to_id: self.in_reply_to_id,
            in_reply_to_account_id: self.in_reply_to_account_id,
            reblog: reblog_status,
            content: self.content,
            plain_content: self.pleroma.content.map(|c| c.text_plain),
            created_at: self.created_at,
            emojis: self.emojis.into_iter().map(|i| i.into()).collect(),
            replies_count: self.replies_count,
            reblogs_count: self.reblogs_count,
            favourites_count: self.favourites_count,
            reblogged: self.reblogged,
            favourited: self.favourited,
            muted: self.muted,
            sensitive: self.sensitive,
            spoiler_text: self.spoiler_text,
            visibility: self.visibility.into(),
            media_attachments: self
                .media_attachments
                .into_iter()
                .map(|i| i.into())
                .collect(),
            mentions: self.mentions.into_iter().map(|i| i.into()).collect(),
            tags: self.tags.into_iter().map(|i| i.into()).collect(),
            card: self.card.map(|i| i.into()),
            poll: self.poll.map(|i| i.into()),
            application: self.application.map(|i| i.into()),
            language: self.language,
            pinned: self.pinned,
            emoji_reactions: self
                .pleroma
                .emoji_reactions
                .map(|v| v.into_iter().map(|e| e.into()).collect()),
            quote: quoted,
            bookmarked: self.bookmarked,
        }
    }
}

impl Into<megalodon::PostStatusOutput> for Status {
    fn into(self) -> megalodon::PostStatusOutput {
        megalodon::PostStatusOutput::Status(self.into())
    }
}

const FIELDS: &'static [&'static str] = &["text/plain"];

impl<'de> de::Deserialize<'de> for PleromaContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        enum Field {
            TextPlain,
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
                        formatter.write_str("`text/plain`")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: de::Error,
                    {
                        match v {
                            "text/plain" => Ok(Field::TextPlain),
                            _ => Err(de::Error::unknown_field(v, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct PleromaContentVisitor;

        impl<'de> de::Visitor<'de> for PleromaContentVisitor {
            type Value = PleromaContent;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("struct PleromaContent")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: de::SeqAccess<'de>,
            {
                let text_plain = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(Self::Value { text_plain })
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: de::MapAccess<'de>,
            {
                let mut text_plain = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::TextPlain => {
                            if text_plain.is_some() {
                                return Err(de::Error::duplicate_field("text_plain"));
                            }
                            text_plain = Some(map.next_value()?);
                        }
                    }
                }
                let text_plain =
                    text_plain.ok_or_else(|| de::Error::missing_field("text_plain"))?;
                Ok(Self::Value { text_plain })
            }
        }

        deserializer.deserialize_struct("PleromaContent", FIELDS, PleromaContentVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pleroma_content_deserialize() {
        let text = r#"{"text/plain":"posted status example"}"#;

        let r = serde_json::from_str::<PleromaContent>(text);
        assert!(r.is_ok());
        assert_eq!(
            r.unwrap(),
            PleromaContent {
                text_plain: String::from("posted status example")
            }
        );
    }
}
