use serde::Deserialize;

use super::Note;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedNote {
    created_note: Note,
}
