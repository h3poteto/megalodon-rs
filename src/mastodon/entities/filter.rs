use chrono::{DateTime, Utc};

pub struct Filter {
    id: String,
    phrase: String,
    context: Vec<FilterContext>,
    expires_at: DateTime<Utc>,
    irreversible: bool,
    whole_word: bool,
}

pub enum FilterContext {
    Home,
    Notifications,
    Public,
    Thread,
}
