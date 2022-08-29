use super::{Account, Status};

pub struct Conversation {
    pub id: String,
    pub accounts: Vec<Account>,
    pub last_status: Option<Status>,
    pub unread: bool,
}
