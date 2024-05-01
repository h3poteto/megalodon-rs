use crate::entities as MegalodonEntities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StatusSource {
    // ID of the status in the database
    id: String,
    // The plain text used to compose the status
    text: String,
    // The plain text used to compose the status’s subject or content warning
    spoiler_text: String,
}

impl From<StatusSource> for MegalodonEntities::StatusSource {
    fn from(val: StatusSource) -> Self {
        MegalodonEntities::StatusSource {
            id: val.id,
            text: val.text,
            spoiler_text: val.spoiler_text,
        }
    }
}
