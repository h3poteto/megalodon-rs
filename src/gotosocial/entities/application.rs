use crate::entities as MegalodonEntities;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Application {
    name: String,
    website: Option<String>,
    vapid_key: Option<String>,
}

impl From<Application> for MegalodonEntities::Application {
    fn from(val: Application) -> Self {
        MegalodonEntities::Application {
            name: val.name,
            website: val.website,
            vapid_key: val.vapid_key,
        }
    }
}
