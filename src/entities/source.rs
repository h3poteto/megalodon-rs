use super::Field;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Source {
    pub privacy: Option<String>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
    pub note: String,
    pub fields: Option<Vec<Field>>,
}
