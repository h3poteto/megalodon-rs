use super::History;

pub struct Tag {
    name: String,
    url: String,
    history: Option<Vec<History>>,
}
