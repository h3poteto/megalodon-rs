use serde::Deserialize;

use super::Note;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Favorite {
    id: String,
    created_at: String,
    note_id: String,
    note: Note,
}
