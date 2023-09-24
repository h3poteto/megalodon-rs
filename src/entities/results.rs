use serde::{Deserialize, Serialize};

use super::{Account, Status, Tag};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Results {
    pub accounts: Vec<Account>,
    pub statuses: Vec<Status>,
    pub hashtags: Vec<Tag>,
}
