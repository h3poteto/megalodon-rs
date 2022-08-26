use chrono::{DateTime, Utc};

pub struct Field {
    name: String,
    value: String,
    verified_at: DateTime<Utc>,
}
