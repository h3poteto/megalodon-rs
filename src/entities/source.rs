use super::Field;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Source {
    pub privacy: Option<String>,
    pub sensitive: Option<bool>,
    pub language: Option<String>,
    pub note: String,
    pub fields: Option<Vec<Field>>,
}
