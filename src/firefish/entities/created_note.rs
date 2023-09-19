use serde::Deserialize;

use super::Note;
use crate::{entities as MegalodonEntities, megalodon};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatedNote {
    created_note: Note,
}

impl Into<MegalodonEntities::Status> for CreatedNote {
    fn into(self) -> MegalodonEntities::Status {
        self.created_note.into()
    }
}

impl Into<megalodon::PostStatusOutput> for CreatedNote {
    fn into(self) -> megalodon::PostStatusOutput {
        megalodon::PostStatusOutput::Status(self.into())
    }
}
