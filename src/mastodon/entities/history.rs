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

impl From<History> for MegalodonEntities::History {
    fn from(val: History) -> Self {
        MegalodonEntities::History {
            day: val.day,
            uses: val.uses,
            accounts: val.accounts,
        }
    }
}
