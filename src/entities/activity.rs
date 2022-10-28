use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Activity {
    pub week: String,
    pub statuses: String,
    pub logins: String,
    pub registrations: String,
}
