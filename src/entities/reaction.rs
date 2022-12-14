use serde::{Deserialize, Serialize};

use super::Account;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub name: String,
    pub accounts: Option<Vec<Account>>,
}
