use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct History {
    pub day: u64,
    pub uses: usize,
    pub accounts: usize,
}
