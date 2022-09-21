use super::Account;

#[derive(Debug, Clone)]
pub struct Reaction {
    pub count: u32,
    pub me: bool,
    pub name: String,
    pub accounts: Option<Vec<Account>>,
}
