use super::Field;

pub struct Source {
    privacy: Option<String>,
    sensitive: Option<bool>,
    language: Option<String>,
    note: String,
    fields: Option<Vec<Field>>,
}
