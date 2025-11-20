use serde::{Deserialize, Serialize};

use crate::entities as MegalodonEntities;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct QuoteApproval {
    pub automatic: Vec<String>,
    pub manual: Vec<String>,
    pub current_user: String,
}

impl From<QuoteApproval> for MegalodonEntities::QuoteApproval {
    fn from(val: QuoteApproval) -> Self {
        MegalodonEntities::QuoteApproval {
            automatic: val.automatic,
            manual: val.manual,
            current_user: val.current_user,
        }
    }
}
