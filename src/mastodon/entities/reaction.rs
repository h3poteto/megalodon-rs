use super::Account;

pub struct Reaction {
    count: u32,
    me: bool,
    name: String,
    accounts: Option<Vec<Account>>,
}
