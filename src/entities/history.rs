use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
    pub day: u64,
    pub uses: usize,
    pub accounts: usize,
}

pub fn parse_from_string<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        D: serde::Deserializer<'de>,
        T: std::str::FromStr,
        <T as std::str::FromStr>::Err: std::fmt::Display,
{
    Ok(String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)?)
}
