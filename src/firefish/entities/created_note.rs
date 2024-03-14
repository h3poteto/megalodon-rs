use serde::Deserialize;

use super::Note;
use crate::{entities as MegalodonEntities, megalodon};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedNote {
    created_note: Note,
}

impl From<CreatedNote> for MegalodonEntities::Status {
    fn from(val: CreatedNote) -> Self {
        val.created_note.into()
    }
}

impl From<CreatedNote> for megalodon::PostStatusOutput {
    fn from(val: CreatedNote) -> Self {
        megalodon::PostStatusOutput::Status(val.into())
    }
}
