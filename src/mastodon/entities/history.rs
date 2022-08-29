use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct History {
    day: u64,
    uses: usize,
    accounts: usize,
}

impl Into<MegalodonEntities::History> for History {
    fn into(self) -> MegalodonEntities::History {
        MegalodonEntities::History {
            day: self.day,
            uses: self.uses,
            accounts: self.accounts,
        }
    }
}
