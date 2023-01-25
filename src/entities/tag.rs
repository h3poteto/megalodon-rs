use serde::{Deserialize, Serialize};

use super::History;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
    pub history: Option<Vec<History>>,
    pub following: bool,
}
