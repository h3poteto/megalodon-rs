use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: Vec<FilterContext>,
    pub expires_at: Option<DateTime<Utc>>,
    pub irreversible: bool,
    pub whole_word: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}
