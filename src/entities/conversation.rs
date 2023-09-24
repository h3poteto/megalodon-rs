use serde::{Deserialize, Serialize};

use super::{Account, Status};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Conversation {
    pub id: String,
    pub accounts: Vec<Account>,
    pub last_status: Option<Status>,
    pub unread: bool,
}
