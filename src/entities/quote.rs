use serde::{Deserialize, Serialize};

use super::status::Status;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ShallowQuote {
    pub state: QuoteState,
    pub quoted_status_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteState {
    Pending,
    Accepted,
    Rejected,
    Revoked,
    Deleted,
    Unauthorized,
    BlockedAccount,
    BlockedDomain,
    MutedAccount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Quote {
    pub state: QuoteState,
    pub quoted_status: Option<Box<Status>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum QuotedStatus {
    Quote(Quote),
    ShallowQuote(ShallowQuote),
}
