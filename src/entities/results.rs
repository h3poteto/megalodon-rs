use super::{Account, Status, Tag};

pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<Tag>,
}
