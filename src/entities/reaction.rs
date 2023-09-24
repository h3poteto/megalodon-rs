use serde::{Deserialize, Serialize};

use super::Account;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub name: String,
    pub url: Option<String>,
    pub static_url: Option<String>,
    pub accounts: Option<Vec<Account>>,
}
