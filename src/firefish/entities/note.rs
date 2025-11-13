use chrono::{DateTime, Utc};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::{collections::HashMap, str::FromStr};

use super::reaction::map_reaction;
use super::{Emoji, File, Poll, User};
use crate::entities as MegalodonEntities;
use crate::error::{Error, Kind};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub(crate) id: String,
    pub(crate) created_at: DateTime<Utc>,
    // user_id: String,
    pub user: User,
    pub(crate) text: Option<String>,
    pub(crate) cw: Option<String>,
    pub(crate) visibility: StatusVisibility,
    pub(crate) renote_count: u32,
    pub(crate) replies_count: u32,
    pub(crate) reactions: HashMap<String, u32>,
    pub(crate) emojis: Option<Vec<Emoji>>,
    // file_ids: Option<Vec<String>>,
    pub(crate) files: Option<Vec<File>>,
    pub(crate) reply_id: Option<String>,
    // renote_id: Option<String>,
    pub(crate) uri: Option<String>,
    // reply: Option<Box<Note>>,
    pub(crate) renote: Option<Box<Note>>,
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) poll: Option<Poll>,
    // mentions: Option<Vec<String>>,
    pub(crate) my_reaction: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum StatusVisibility {
    Public,
    Home,
    Followers,
    Specified,
    Hidden,
}

impl fmt::Display for StatusVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StatusVisibility::Public => write!(f, "public"),
            StatusVisibility::Home => write!(f, "home"),
            StatusVisibility::Followers => write!(f, "followers"),
            StatusVisibility::Specified => write!(f, "specified"),
            StatusVisibility::Hidden => write!(f, "hidden"),
        }
    }
}

impl FromStr for StatusVisibility {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "public" => Ok(StatusVisibility::Public),
            "home" => Ok(StatusVisibility::Home),
            "followers" => Ok(StatusVisibility::Followers),
            "specified" => Ok(StatusVisibility::Specified),
            "hidden" => Ok(StatusVisibility::Hidden),
            _ => Err(Error::new_own(
                s.to_owned(),
                Kind::ParseError,
                None,
                None,
                None,
            )),
        }
    }
}

impl From<MegalodonEntities::status::StatusVisibility> for StatusVisibility {
    fn from(value: MegalodonEntities::status::StatusVisibility) -> Self {
        match value {
            MegalodonEntities::StatusVisibility::Public => StatusVisibility::Public,
            MegalodonEntities::StatusVisibility::Unlisted => StatusVisibility::Home,
            MegalodonEntities::StatusVisibility::Private => StatusVisibility::Followers,
            MegalodonEntities::StatusVisibility::Direct => StatusVisibility::Specified,
            MegalodonEntities::StatusVisibility::Local => StatusVisibility::Hidden,
        }
    }
}

impl From<StatusVisibility> for MegalodonEntities::status::StatusVisibility {
    fn from(val: StatusVisibility) -> Self {
        match val {
            StatusVisibility::Public => MegalodonEntities::status::StatusVisibility::Public,
            StatusVisibility::Home => MegalodonEntities::status::StatusVisibility::Unlisted,
            StatusVisibility::Followers => MegalodonEntities::status::StatusVisibility::Private,
            StatusVisibility::Specified => MegalodonEntities::status::StatusVisibility::Direct,
            StatusVisibility::Hidden => MegalodonEntities::status::StatusVisibility::Local,
        }
    }
}

fn convert_html(plain: String) -> String {
    let c = plain
        .replace("&", "&amp;")
        .replace("<", "&lt;")
        .replace(">", "&gt;")
        .replace("\"", "&quot;")
        .replace(r"'", "&#39;")
        .replace(r"`", "&#x60;");
    Regex::new(r"\r?\n")
        .unwrap()
        .replace_all(c.as_str(), "<br>")
        .to_string()
}

impl From<Note> for MegalodonEntities::Status {
    fn from(val: Note) -> Self {
        let mut uri = "".to_string();
        if let Some(u) = val.uri.clone() {
            uri = u;
        }
        let mut reblog_status: Option<Box<MegalodonEntities::Status>> = None;
        let mut quote: Option<MegalodonEntities::QuotedStatus> = None;

        if let Some(renote) = val.renote {
            let rs: Note = *renote;
            if let Some(_) = val.text.clone() {
                quote = Some(MegalodonEntities::QuotedStatus::Quote(
                    MegalodonEntities::quote::Quote {
                        state: MegalodonEntities::quote::QuoteState::Accepted,
                        quoted_status: Some(Box::new(rs.into())),
                    },
                ));
            } else {
                reblog_status = Some(Box::new(rs.into()));
            }
        }
        let mut content = "".to_string();
        if let Some(text) = val.text.clone() {
            content = convert_html(text);
        }

        let mut spoiler_text = "".to_string();
        if let Some(cw) = val.cw {
            spoiler_text = cw;
        }

        let mut tags: Vec<MegalodonEntities::status::Tag> = [].to_vec();
        if let Some(hashtags) = val.tags {
            tags = hashtags
                .into_iter()
                .map(|t| MegalodonEntities::status::Tag {
                    name: t.clone(),
                    url: t,
                })
                .collect();
        }
        let emoji_reactions = Some(map_reaction(
            val.emojis.clone().unwrap_or_default(),
            val.reactions,
            val.my_reaction.clone(),
        ));

        MegalodonEntities::Status {
            id: val.id,
            uri: uri.clone(),
            url: val.uri,
            account: val.user.into(),
            in_reply_to_id: val.reply_id,
            in_reply_to_account_id: None,
            reblog: reblog_status,
            content,
            plain_content: val.text,
            created_at: val.created_at,
            edited_at: None,
            emojis: val.emojis.map_or([].to_vec(), |o| {
                o.into_iter()
                    .filter(|e| !e.name.contains("@"))
                    .map(|e| e.into())
                    .collect()
            }),
            replies_count: val.replies_count,
            reblogs_count: val.renote_count,
            favourites_count: 0,
            reblogged: None,
            favourited: Some(val.my_reaction.is_some()),
            muted: None,
            sensitive: val
                .files
                .as_ref()
                .map_or(false, |f| f.iter().any(|f| f.is_sensitive)),
            spoiler_text,
            visibility: val.visibility.into(),
            media_attachments: val
                .files
                .map_or([].to_vec(), |f| f.into_iter().map(|f| f.into()).collect()),
            mentions: [].to_vec(),
            tags,
            card: None,
            poll: val.poll.map(|p| p.into()),
            application: None,
            language: None,
            pinned: None,
            emoji_reactions,
            quote,
            quote_approval: MegalodonEntities::QuoteApproval::default(),
            bookmarked: None,
        }
    }
}

impl From<Note> for MegalodonEntities::StatusSource {
    fn from(val: Note) -> Self {
        let Note { id, text, cw, .. } = val;
        MegalodonEntities::StatusSource {
            id,
            text: text.unwrap_or_default(),
            spoiler_text: cw.unwrap_or_default(),
        }
    }
}

impl From<Note> for MegalodonEntities::Conversation {
    fn from(val: Note) -> Self {
        let accounts: Vec<MegalodonEntities::Account> = [val.user.clone().into()].to_vec();
        MegalodonEntities::Conversation {
            id: val.id.clone(),
            accounts,
            last_status: Some(val.into()),
            unread: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_html() {
        let plain_html = String::from("<p>hoge\nfuga\nfuga<p>");
        let escaped_html = String::from("&lt;p&gt;hoge<br>fuga<br>fuga&lt;p&gt;");
        assert_eq!(convert_html(plain_html), escaped_html);

        let plain_text = String::from("hoge\nfuga\nfuga");
        let html_text = String::from("hoge<br>fuga<br>fuga");
        assert_eq!(convert_html(plain_text), html_text);
    }
}
