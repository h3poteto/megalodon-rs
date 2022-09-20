#[derive(Debug, Clone)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
    pub vapid_key: Option<String>,
}
