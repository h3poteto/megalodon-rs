use core::fmt;

use super::{Account, Application, Attachment, Card, Emoji, Mention, Poll, Reaction};
use crate::{
    entities::{self as MegalodonEntities},
    megalodon,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize, de};

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
    pub edited_at: Option<DateTime<Utc>>,
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
    Local,
}

impl fmt::Display for StatusVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusVisibility::Public => write!(f, "public"),
            StatusVisibility::Unlisted => write!(f, "unlisted"),
            StatusVisibility::Private => write!(f, "private"),
            StatusVisibility::Direct => write!(f, "direct"),
            StatusVisibility::Local => write!(f, "local"),
        }
    }
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
    pub quote: Option<Box<Status>>,
    pub quote_url: Option<String>,
    pub quote_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PleromaContent {
    pub text_plain: String,
}

impl From<StatusVisibility> for MegalodonEntities::status::StatusVisibility {
    fn from(val: StatusVisibility) -> Self {
        match val {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Unlisted => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Private => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Direct => MegalodonEntities::status::StatusVisibility::Direct,
            StatusVisibility::Local => MegalodonEntities::status::StatusVisibility::Local,
        }
    }
}

impl From<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn from(value: MegalodonEntities::status::StatusVisibility) -> Self {
        match value {
            MegalodonEntities::status::StatusVisibility::Public => StatusVisibility::Public,
            MegalodonEntities::status::StatusVisibility::Unlisted => StatusVisibility::Unlisted,
            MegalodonEntities::status::StatusVisibility::Private => StatusVisibility::Private,
            MegalodonEntities::status::StatusVisibility::Direct => StatusVisibility::Direct,
            MegalodonEntities::status::StatusVisibility::Local => StatusVisibility::Local,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
}

impl From<Tag> for MegalodonEntities::status::Tag {
    fn from(val: Tag) -> Self {
        MegalodonEntities::status::Tag {
            name: val.name,
            url: val.url,
        }
    }
}

impl From<Status> for MegalodonEntities::Status {
    fn from(val: Status) -> Self {
        let mut quote: Option<MegalodonEntities::QuotedStatus> = None;

        if let Some(q) = val.pleroma.quote {
            let qs: Status = *q;
            quote = Some(MegalodonEntities::QuotedStatus::Quote(
                MegalodonEntities::quote::Quote {
                    state: MegalodonEntities::quote::QuoteState::Accepted,
                    quoted_status: Some(Box::new((qs).into())),
                },
            ));
        } else if let Some(quote_id) = val.pleroma.quote_id {
            quote = Some(MegalodonEntities::QuotedStatus::ShallowQuote(
                MegalodonEntities::quote::ShallowQuote {
                    state: MegalodonEntities::quote::QuoteState::Accepted,
                    quoted_status_id: Some(quote_id),
                },
            ));
        }

        MegalodonEntities::Status {
            id: val.id,
            uri: val.uri,
            url: val.url,
            account: val.account.into(),
            in_reply_to_id: val.in_reply_to_id,
            in_reply_to_account_id: val.in_reply_to_account_id,
            reblog: val.reblog.map(|r| {
                let rs: Status = *r;
                Box::new((rs).into())
            }),
            content: val.content,
            plain_content: val.pleroma.content.map(|c| c.text_plain),
            created_at: val.created_at,
            edited_at: val.edited_at,
            emojis: val.emojis.into_iter().map(|i| i.into()).collect(),
            replies_count: val.replies_count,
            reblogs_count: val.reblogs_count,
            favourites_count: val.favourites_count,
            reblogged: val.reblogged,
            favourited: val.favourited,
            muted: val.muted,
            sensitive: val.sensitive,
            spoiler_text: val.spoiler_text,
            visibility: val.visibility.into(),
            media_attachments: val
                .media_attachments
                .into_iter()
                .map(|i| i.into())
                .collect(),
            mentions: val.mentions.into_iter().map(|i| i.into()).collect(),
            tags: val.tags.into_iter().map(|i| i.into()).collect(),
            card: val.card.map(|i| i.into()),
            poll: val.poll.map(|i| i.into()),
            application: val.application.map(|i| i.into()),
            language: val.language,
            pinned: val.pinned,
            emoji_reactions: val
                .pleroma
                .emoji_reactions
                .map(|v| v.into_iter().map(|e| e.into()).collect()),
            quote,
            quote_approval: MegalodonEntities::QuoteApproval::automatic_unsupported(),
            bookmarked: val.bookmarked,
        }
    }
}

impl From<Status> for megalodon::PostStatusOutput {
    fn from(val: Status) -> Self {
        megalodon::PostStatusOutput::Status(val.into())
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
