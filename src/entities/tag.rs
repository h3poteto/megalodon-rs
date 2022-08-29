use super::History;

pub struct Tag {
    pub name: String,
    pub url: String,
    pub history: Option<Vec<History>>,
}
