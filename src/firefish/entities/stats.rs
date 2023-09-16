use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    notes_count: u32,
    original_notes_count: u32,
    users_count: u32,
    original_users_count: u32,
    instances: u32,
}
