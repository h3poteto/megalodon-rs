use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Poll {
    multiple: bool,
    expires_at: DateTime<Utc>,
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    text: String,
    votes: u32,
    is_voted: Option<bool>,
}
