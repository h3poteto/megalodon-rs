use serde::Deserialize;

use super::status::Status;
use crate::entities as MegalodonEntities;

#[derive(Debug, Clone, Deserialize)]
pub struct ShallowQuote {
    pub state: QuoteState,
    pub quoted_status_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct Quote {
    pub state: QuoteState,
    pub quoted_status: Option<Box<Status>>,
}

#[derive(Debug, Clone, Deserialize)]
pub enum QuotedStatus {
    ShallowQuote(ShallowQuote),
    Quote(Quote),
}

impl From<QuoteState> for MegalodonEntities::QuoteState {
    fn from(val: QuoteState) -> Self {
        match val {
            QuoteState::Pending => MegalodonEntities::QuoteState::Pending,
            QuoteState::Accepted => MegalodonEntities::QuoteState::Accepted,
            QuoteState::Rejected => MegalodonEntities::QuoteState::Rejected,
            QuoteState::Revoked => MegalodonEntities::QuoteState::Revoked,
            QuoteState::Deleted => MegalodonEntities::QuoteState::Deleted,
            QuoteState::Unauthorized => MegalodonEntities::QuoteState::Unauthorized,
            QuoteState::BlockedAccount => MegalodonEntities::QuoteState::BlockedAccount,
            QuoteState::BlockedDomain => MegalodonEntities::QuoteState::BlockedDomain,
            QuoteState::MutedAccount => MegalodonEntities::QuoteState::MutedAccount,
        }
    }
}

impl From<ShallowQuote> for MegalodonEntities::ShallowQuote {
    fn from(val: ShallowQuote) -> Self {
        MegalodonEntities::ShallowQuote {
            state: val.state.into(),
            quoted_status_id: val.quoted_status_id,
        }
    }
}

impl From<Quote> for MegalodonEntities::Quote {
    fn from(val: Quote) -> Self {
        MegalodonEntities::Quote {
            state: val.state.into(),
            quoted_status: val.quoted_status.map(|qs| {
                let status: Status = *qs;
                Box::new(status.into())
            }),
        }
    }
}

impl From<QuotedStatus> for MegalodonEntities::QuotedStatus {
    fn from(val: QuotedStatus) -> Self {
        match val {
            QuotedStatus::ShallowQuote(sq) => {
                MegalodonEntities::QuotedStatus::ShallowQuote(sq.into())
            }
            QuotedStatus::Quote(q) => MegalodonEntities::QuotedStatus::Quote(q.into()),
        }
    }
}
