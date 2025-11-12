use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct QuoteApproval {
    pub automatic: Vec<String>,
    pub manual: Vec<String>,
    pub current_user: String,
}

impl Default for QuoteApproval {
    fn default() -> Self {
        QuoteApproval {
            automatic: vec!["public".to_string()],
            manual: Vec::new(),
            current_user: "automatic".to_string(),
        }
    }
}
