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

impl Into<MegalodonEntities::PollOption> for Choice {
    fn into(self) -> MegalodonEntities::PollOption {
        MegalodonEntities::PollOption {
            title: self.text,
            votes_count: Some(self.votes),
        }
    }
}

impl Into<MegalodonEntities::Poll> for Poll {
    fn into(self) -> MegalodonEntities::Poll {
        let mut expired = false;
        if let Some(at) = self.expires_at {
            let now = Utc::now();
            let diff = now - at;
            if diff.num_seconds() > 0 {
                expired = true;
            }
        }
        let votes_count: u32 = self.choices.iter().map(|c| c.votes).sum();

        MegalodonEntities::Poll {
            id: "".to_string(),
            expires_at: self.expires_at,
            expired,
            multiple: self.multiple,
            votes_count,
            voters_count: None,
            options: self.choices.clone().into_iter().map(|c| c.into()).collect(),
            voted: Some(self.choices.iter().any(|c| {
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
