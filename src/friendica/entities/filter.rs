use crate::entities as MegalodonEntities;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Filter {
    id: String,
    phrase: String,
    context: Vec<FilterContext>,
    expires_at: Option<DateTime<Utc>>,
    irreversible: bool,
    whole_word: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}

impl Into<MegalodonEntities::filter::FilterContext> for FilterContext {
    fn into(self) -> MegalodonEntities::filter::FilterContext {
        match self {
            FilterContext::Home => MegalodonEntities::filter::FilterContext::Home,
            FilterContext::Notifications => MegalodonEntities::filter::FilterContext::Notifications,
            FilterContext::Public => MegalodonEntities::filter::FilterContext::Public,
            FilterContext::Thread => MegalodonEntities::filter::FilterContext::Thread,
        }
    }
}

impl Into<MegalodonEntities::Filter> for Filter {
    fn into(self) -> MegalodonEntities::Filter {
        MegalodonEntities::Filter {
            id: self.id,
            phrase: self.phrase,
            context: self.context.into_iter().map(|i| i.into()).collect(),
            expires_at: self.expires_at,
            irreversible: self.irreversible,
            whole_word: self.whole_word,
        }
    }
}
