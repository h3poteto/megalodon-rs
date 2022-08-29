use chrono::{DateTime, Utc};

pub struct Filter {
    pub id: String,
    pub phrase: String,
    pub context: Vec<FilterContext>,
    pub expires_at: DateTime<Utc>,
    pub irreversible: bool,
    pub whole_word: bool,
}

pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}
