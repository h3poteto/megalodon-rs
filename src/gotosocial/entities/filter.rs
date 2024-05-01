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

impl From<FilterContext> for MegalodonEntities::filter::FilterContext {
    fn from(val: FilterContext) -> Self {
        match val {
            FilterContext::Home => MegalodonEntities::filter::FilterContext::Home,
            FilterContext::Notifications => MegalodonEntities::filter::FilterContext::Notifications,
            FilterContext::Public => MegalodonEntities::filter::FilterContext::Public,
            FilterContext::Thread => MegalodonEntities::filter::FilterContext::Thread,
        }
    }
}

impl From<Filter> for MegalodonEntities::Filter {
    fn from(val: Filter) -> Self {
        MegalodonEntities::Filter {
            id: val.id,
            phrase: val.phrase,
            context: val.context.into_iter().map(|i| i.into()).collect(),
            expires_at: val.expires_at,
            irreversible: val.irreversible,
            whole_word: val.whole_word,
        }
    }
}
