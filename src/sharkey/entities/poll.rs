use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::entities as MegalodonEntities;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    multiple: bool,
    expires_at: Option<DateTime<Utc>>,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    text: String,
    votes: u32,
    is_voted: Option<bool>,
}

impl From<Choice> for MegalodonEntities::PollOption {
    fn from(val: Choice) -> Self {
        MegalodonEntities::PollOption {
            title: val.text,
            votes_count: Some(val.votes),
        }
    }
}

impl From<Poll> for MegalodonEntities::Poll {
    fn from(val: Poll) -> Self {
        let mut expired = false;
        if let Some(at) = val.expires_at {
            let now = Utc::now();
            let diff = now - at;
            if diff.num_seconds() > 0 {
                expired = true;
            }
        }
        let votes_count: u32 = val.choices.iter().map(|c| c.votes).sum();

        MegalodonEntities::Poll {
            id: "".to_string(),
            expires_at: val.expires_at,
            expired,
            multiple: val.multiple,
            votes_count,
            voters_count: None,
            options: val.choices.clone().into_iter().map(|c| c.into()).collect(),
            voted: Some(val.choices.iter().any(|c| {
                if let Some(voted) = c.is_voted {
                    voted
                } else {
                    false
                }
            })),
            emojis: [].to_vec(),
        }
    }
}
