use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct App {
    id: String,
    name: String,
    callback_url: Option<String>,
    permission: Vec<String>,
    secret: Option<String>,
    is_authorized: Option<bool>,
}
