use super::{Account, Status};

pub struct Conversation {
    id: String,
    accounts: Vec<Account>,
    last_status: Option<Status>,
    unread: bool,
}
