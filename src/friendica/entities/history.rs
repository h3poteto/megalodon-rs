use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct History {
    #[serde(deserialize_with = "MegalodonEntities::history::parse_from_string")]
    day: u64,
    #[serde(deserialize_with = "MegalodonEntities::history::parse_from_string")]
    uses: usize,
    #[serde(deserialize_with = "MegalodonEntities::history::parse_from_string")]
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
