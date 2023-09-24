use serde::{Deserialize, Serialize};

use super::History;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
    pub history: Vec<History>,
    pub following: Option<bool>,
}
