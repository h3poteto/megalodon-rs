use chrono::{DateTime, Utc};
use serde::Serialize;

pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: Vec<FilterContext>,
    pub expires_at: DateTime<Utc>,
    pub irreversible: bool,
    pub whole_word: bool,
}

#[derive(Debug, Serialize)]
pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}
