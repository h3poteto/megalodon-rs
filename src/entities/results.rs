use super::{Account, Status, Tag};

#[derive(Debug, Clone)]
pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<Tag>,
}
