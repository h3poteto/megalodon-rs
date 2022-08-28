use super::{Account, Status, Tag};

pub struct Results {
    accounts: Vec<Account>,
    statuses: Vec<Status>,
    hashtags: Vec<Tag>,
}
